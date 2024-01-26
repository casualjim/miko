use std::{convert::Infallible, path::PathBuf, str::FromStr, time::Duration};

use axum::{
  body::Body,
  extract::{Path, State},
  http::Request,
  response::{
    sse::{Event, Sse},
    IntoResponse,
  },
  routing::get,
  Json,
};
use bytes::Bytes;
use futures::{
  stream::{Stream, StreamExt},
  TryStreamExt,
};
use notify::Watcher;
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;
use tower::{BoxError, Service};
use tower_http::services::ServeFile;
use tracing::info;
use uuid::Uuid;

use crate::{
  app::{handlers::AuthSession, state::AppState},
  models::UploadedFile,
  Error, Result,
};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .route(
      "/:chat_id",
      get(list_files).post(upload_files).delete(remove_workspace),
    )
    .route("/:chat_id/watch", get(watch_files))
    .route("/:chat_id/files/:file_name", get(serve_file))
    .with_state(app_state)
}

#[tracing::instrument(skip(app_state, auth))]
async fn serve_file(
  State(app_state): State<AppState>,
  Path((chat_id, file_name)): Path<(Uuid, String)>,
  auth: AuthSession,
  req: Request<Body>,
) -> Result<impl IntoResponse> {
  if !auth.is_authenticated() {
    return Err(Error::UserNotAuthenticated);
  }

  let file_path = app_state
    .upload_store
    .join(chat_id.to_string())
    .join(file_name);
  Ok(ServeFile::new(file_path).call(req).await.unwrap())
}

#[tracing::instrument(skip(app_state, auth))]
async fn watch_files(
  State(app_state): State<AppState>,
  Path(chat_id): Path<Uuid>,
  auth: AuthSession,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
  if !auth.is_authenticated() {
    return Err(Error::UserNotAuthenticated);
  }

  let (tx, rx) = futures::channel::mpsc::unbounded();

  let mut watcher =
    notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
      if let Ok(event) = res {
        let kind = &event.kind;
        if !kind.is_create() && !kind.is_modify() && !kind.is_remove() {
          return;
        }

        info!("event: {:?}", event);
        if let Some(path) = event.paths.last() {
          let mime = mime_guess::from_path(path).first_or_octet_stream();
          let filename = path.file_name().unwrap().to_str().unwrap().to_string();
          let data = serde_json::to_string(&UploadedFile {
            file_name: filename,
            mime_type: mime.to_string(),
            workspace: chat_id.to_string(),
          })
          .unwrap();
          info!("sending event for file {}", data);
          _ = tx.unbounded_send(Event::default().data(data));
        }
      }
    })
    .map_err(Error::Watcher)?;

  watcher
    .watch(
      app_state.upload_store.join(chat_id.to_string()).as_path(),
      notify::RecursiveMode::NonRecursive,
    )
    .map_err(Error::Watcher)?;

  std::mem::forget(watcher);

  Ok(
    Sse::new(rx.map(Ok)).keep_alive(
      axum::response::sse::KeepAlive::new()
        .interval(Duration::from_secs(15))
        .text("keep-alive-text"),
    ),
  )
}

#[tracing::instrument(skip(app_state, auth))]
async fn remove_workspace(
  State(app_state): State<AppState>,
  Path(chat_id): Path<Uuid>,
  auth: AuthSession,
) -> Result<()> {
  if !auth.is_authenticated() {
    return Err(Error::UserNotAuthenticated);
  }
  let workspace_dir = app_state.upload_store.join(chat_id.to_string());

  if !tokio::fs::try_exists(&workspace_dir).await? {
    return Ok(());
  }
  tokio::fs::remove_dir_all(workspace_dir).await?;
  Ok(())
}

#[tracing::instrument(skip(app_state, auth))]
async fn list_files(
  State(app_state): State<AppState>,
  Path(chat_id): Path<Uuid>,
  auth: AuthSession,
) -> Result<Json<Vec<String>>> {
  if !auth.is_authenticated() {
    return Err(Error::UserNotAuthenticated);
  }
  let workspace_dir = app_state.upload_store.join(chat_id.to_string());

  if !tokio::fs::try_exists(&workspace_dir).await? {
    return Ok(Json(vec![]));
  }

  let mut reader = tokio::fs::read_dir(workspace_dir).await?;
  let mut result = vec![];
  while let Some(file) = reader.next_entry().await? {
    result.push(file.file_name().to_string_lossy().to_string());
  }
  Ok(Json(result))
}

#[tracing::instrument(skip(app_state, auth))]
async fn upload_files(
  State(app_state): State<AppState>,
  Path(chat_id): Path<Uuid>,
  auth: AuthSession,
  mut multipart: axum::extract::Multipart,
) -> Result<Json<usize>> {
  if !auth.is_authenticated() {
    return Err(Error::UserNotAuthenticated);
  }
  let workspace_dir = app_state.upload_store.join(chat_id.to_string());
  if !tokio::fs::try_exists(&workspace_dir).await? {
    tokio::fs::create_dir_all(&workspace_dir).await?;
  }

  let mut count = 0;
  while let Ok(Some(field)) = multipart.next_field().await {
    let file_name = if let Some(file_name) = field.file_name() {
      PathBuf::from_str(file_name).unwrap() // this is infallible
    } else {
      continue;
    };

    stream_to_file(&workspace_dir, &file_name, field).await?;
    count += 1;
  }
  Ok(Json(count))
}

// Save a `Stream` to a file
pub async fn stream_to_file<P, S, E>(base_path: P, file_name: P, stream: S) -> Result<()>
where
  P: AsRef<std::path::Path>,
  S: Stream<Item = Result<Bytes, E>>,
  E: Into<BoxError>,
{
  let file_name = file_name.as_ref();
  let base_path = base_path.as_ref();
  if !path_is_valid(file_name) {
    return Err(Error::InvalidArgument(format!(
      "Invalid path: {}",
      file_name.display()
    )));
  }

  async {
    // Convert the stream into an `AsyncRead`.
    let body_with_io_error =
      stream.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));
    let body_reader = StreamReader::new(body_with_io_error);
    futures::pin_mut!(body_reader);

    // Create the file. `File` implements `AsyncWrite`.
    let path = base_path.join(file_name);
    let mut file = BufWriter::new(File::create(path).await?);

    // Copy the body into the file.
    tokio::io::copy(&mut body_reader, &mut file).await?;

    Ok::<_, std::io::Error>(())
  }
  .await?;
  Ok(())
}

// to prevent directory traversal attacks we ensure the path consists of exactly one normal
// component
fn path_is_valid<P: AsRef<std::path::Path>>(path: P) -> bool {
  let path = path.as_ref();
  let mut components = path.components().peekable();

  if let Some(first) = components.peek() {
    if !matches!(first, std::path::Component::Normal(_)) {
      return false;
    }
  }

  components.count() == 1
}

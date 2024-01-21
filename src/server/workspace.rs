use std::{path::PathBuf, str::FromStr};

use axum::{
  extract::{Path, State},
  routing::get,
  Json,
};
use bytes::Bytes;
use futures::{Stream, TryStreamExt};
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;
use tower::BoxError;
use uuid::Uuid;

use crate::{
  app::{handlers::AuthSession, state::AppState},
  Error, Result,
};

pub fn routes(app_state: AppState) -> axum::Router<AppState> {
  axum::Router::new()
    .route("/:chat_id", get(list_files).post(upload_files))
    .with_state(app_state)
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
async fn stream_to_file<P, S, E>(base_path: P, file_name: P, stream: S) -> Result<()>
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

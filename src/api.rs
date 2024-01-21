use leptos::Serializable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_sys::{File, FormData};

use crate::error_template::AppError;

#[cfg(feature = "hydrate")]
pub async fn upload_file<T>(chat_id: Uuid, data: FormData) -> Option<T>
where
  T: Serializable,
{
  let abort_controller = web_sys::AbortController::new().ok();
  let abort_signal = abort_controller.as_ref().map(|a| a.signal());

  // abort in-flight requests if e.g., we've navigated away from this page
  leptos::on_cleanup(move || {
    if let Some(abort_controller) = abort_controller {
      abort_controller.abort()
    }
  });

  let uri = format!("/api/v1/workspace/{}", chat_id);

  let json = gloo_net::http::Request::post(&uri)
    .abort_signal(abort_signal.as_ref())
    .body(data)
    .unwrap()
    .send()
    .await
    .ok()?
    .text()
    .await
    .ok()?;

  T::de(&json).ok()
}

// #[cfg(feature = "ssr")]
// pub async fn upload_file<T>(chat_id: Uuid, data: Vec<File>) -> Option<T>
// where
//   T: Serializable,
// {
//   use reqwest::multipart;

//   let mut form = multipart::Form::new();
//   for file in data {
//     form = form.part("file", multipart::Part::stream(file).file_name("file"));
//   }
//   unimplemented!("upload_file is not implemented for ssr")
// }

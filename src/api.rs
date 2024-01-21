use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature="hydrate")] {
    use leptos::Serializable;
    use uuid::Uuid;
    use web_sys::{File, FormData};

    pub async fn upload_file<T>(chat_id: Uuid, files: Vec<File>) -> Option<T>
    where
      T: Serializable,
    {
      let data = FormData::new().ok()?;
      for file in files {
        data.append_with_blob("file", &file).ok()?;
      }

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
  } else {
    pub async fn upload_file<T>(_chat_id: uuid::Uuid, _files: Vec<web_sys::File>) -> Option<T>
    where
      T: leptos::Serializable + Default,
    {
      Some(T::de(Default::default()).unwrap())
    }
  }
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

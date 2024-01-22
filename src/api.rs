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
      data.append_with_str("chat_id", &chat_id.to_string()).ok()?;
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

    pub async fn get_text_file(chat_id: String, file_name: String) -> Option<String> {
      let uri = format!("/api/v1/workspace/{}/files/{}", chat_id, file_name);

      let abort_controller = web_sys::AbortController::new().ok();
      let abort_signal = abort_controller.as_ref().map(|a| a.signal());

      // abort in-flight requests if e.g., we've navigated away from this page
      leptos::on_cleanup(move || {
        if let Some(abort_controller) = abort_controller {
          abort_controller.abort()
        }
      });

      let file = gloo_net::http::Request::get(&uri)
        .abort_signal(abort_signal.as_ref())
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()?;

      Some(file)
    }


  } else if #[cfg(feature = "ssr")]{
    pub async fn upload_file<T>(_chat_id: uuid::Uuid, _files: Vec<web_sys::File>) -> Option<T>
    where
      T: leptos::Serializable + Default,
    {
      Some(T::de(Default::default()).unwrap())
    }

    pub async fn get_text_file(chat_id: String, file_name: String) -> Option<String> {
      let uri = format!("/api/v1/workspace/{}/files/{}", chat_id, file_name);
      let mut res = reqwest::get(uri).await.ok()?;
      let text = res.text().await.ok()?;
      Some(text)

    }
  } else {
    pub async fn upload_file<T>(_chat_id: uuid::Uuid, _files: Vec<web_sys::File>) -> Option<T>
    where
      T: leptos::Serializable + Default,
    {
      Some(T::de(Default::default()).unwrap())
    }

    pub async fn get_text_file(_chat_id: String, _file_name: String) -> Option<String> {
      None
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

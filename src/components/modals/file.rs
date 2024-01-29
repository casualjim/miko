use leptos::*;

use crate::{
  api,
  components::{mdown::Markdown, modals::Modal},
  models::UploadedFile,
};

#[component]
pub fn FileModal(
  show_modal: RwSignal<bool>,
  content: ReadSignal<Option<UploadedFile>>,
) -> impl IntoView {
  let (markdown_content, set_markdown_content) = create_signal(String::new());
  let (text_content, set_text_content) = create_signal(String::new());
  let (file_type, set_file_type) = create_signal(String::new());

  create_effect(move |_| {
    if let Some(file) = content().as_ref() {
      let prefix = file
        .mime_type
        .as_str()
        .split('/')
        .next()
        .unwrap_or_default();

      match (prefix, file.mime_type.as_str()) {
        ("text", "text/markdown") => {
          set_file_type.update(|v| *v = "markdown".to_string());
          let file_name = file.file_name.clone();
          let chat_id = file.workspace.clone();

          spawn_local(async move {
            let content = api::get_text_file(chat_id, file_name).await.unwrap();
            set_markdown_content.update(|v| *v = content);
          })
        }
        ("application", "application/pdf") => {
          set_file_type.update(|v| *v = "pdf".to_string());
        }
        ("text", _) => {
          set_file_type.update(|v| *v = prefix.to_string());
          let file_name = file.file_name.clone();
          let chat_id = file.workspace.clone();

          spawn_local(async move {
            let content = api::get_text_file(chat_id, file_name).await.unwrap();
            set_text_content.update(|v| *v = content);
          })
        }
        ("image", _) => {
          set_file_type.update(|v| *v = prefix.to_string());
        }
        ("video", _) => {
          set_file_type.update(|v| *v = prefix.to_string());
        }
        ("audio", _) => {
          set_file_type.update(|v| *v = prefix.to_string());
        }
        _ => {
          set_file_type.update(|v| *v = prefix.to_string());
        }
      }

      // read_file(file.clone(), set_markdown_content);
      // spa
    }
  });

  view! {
    <Modal id="fileModal" show_modal=show_modal>
      <div class="modal-action">
        <Show when=move || content().is_some()>
          <Show when=move || { file_type() == "markdown" }>
            <div class="prose prose-invert w-full max-w-none overflow-auto p-8 pr-[1.5rem] [scrollbar-gutter:stable]">
              <Markdown content=markdown_content/>
            </div>
          </Show>
          <Show when=move || { file_type() == "pdf" }>
            <div class="w-full h-full">
              <object
                data=move || {
                    let UploadedFile { file_name, workspace, .. } = content().unwrap();
                    format!("/api/v1/workspace/{workspace}/files/{file_name}")
                }

                type="application/pdf"
                width="100%"
                height="500px"
              >
                <p>
                  "Your browser does not support PDFs. "
                  <a href=move || {
                      let UploadedFile { file_name, workspace, .. } = content().unwrap();
                      format!("/api/v1/workspace/{workspace}/files/{file_name}")
                  }>"Download the PDF"</a> .
                </p>
              </object>
            </div>
          </Show>
          <Show when=move || { file_type() == "image" }>
            <div class="w-full h-full">
              <pre>{text_content()}</pre>
            </div>
          </Show>
          <Show when=move || { file_type() == "image" }>
            <div class="w-full h-full">
              <img
                class="w-full h-full"
                src=move || {
                    let UploadedFile { file_name, workspace, .. } = content().unwrap();
                    format!("/api/v1/workspace/{workspace}/files/{file_name}")
                }
              />

            </div>
          </Show>
          <Show when=move || { file_type() == "audio" }>
            <div class="w-full h-full">
              <audio class="w-full h-full" controls>
                {move || {
                    let UploadedFile { file_name, workspace, mime_type, .. } = content().unwrap();
                    view! { <source src=format!("/api/v1/workspace/{workspace}/files/{file_name}") type=mime_type/> }
                }}

              </audio>
            </div>
          </Show>
          <Show when=move || { file_type() == "video" }>
            <div class="w-full h-full">
              <video class="w-full h-full" controls>
                {move || {
                    let UploadedFile { file_name, workspace, mime_type, .. } = content().unwrap();
                    view! { <source src=format!("/api/v1/workspace/{workspace}/files/{file_name}") type=mime_type/> }
                }}

              </video>
            </div>
          </Show>
        </Show>
      </div>

    </Modal>
  }
}

use leptos::*;
use web_sys::{File, FileReader};

use crate::components::{mdown::Markdown, modals::Modal};

#[component]
pub fn FileModal(show_modal: RwSignal<bool>, content: ReadSignal<Option<File>>) -> impl IntoView {
  let markdown_content = Signal::derive(move || {
    content()
      .as_ref()
      .map(|file| {
        let reader = FileReader::new().unwrap();
        reader.read_as_text(file).unwrap();
        reader.result().unwrap().as_string().unwrap()
      })
      .unwrap_or_default()
  });

  view! {
    <Modal id="fileModal" show_modal=show_modal>
      <div class="modal-action">
        <div class="prose prose-invert w-full max-w-none overflow-auto p-8 pr-[1.5rem] text-xs [scrollbar-gutter:stable]">
          <Markdown content=markdown_content/>
        </div>
      </div>

    </Modal>
  }
}

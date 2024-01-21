use futures::io::Write;
use leptos::{logging::log, *};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{File, FileReader};

use crate::components::{mdown::Markdown, modals::Modal};

#[component]
pub fn FileModal(show_modal: RwSignal<bool>, content: ReadSignal<Option<File>>) -> impl IntoView {
  let (markdown_content, set_markdown_content) = create_signal(String::new());

  create_effect(move |_| {
    if let Some(file) = content().as_ref() {
      read_file(file.clone(), set_markdown_content);
    }
  });

  view! {
    <Modal id="fileModal" show_modal=show_modal>
      <div class="modal-action">
        <div class="prose prose-invert w-full max-w-none overflow-auto p-8 pr-[1.5rem] [scrollbar-gutter:stable]">
          <Markdown content=markdown_content/>
        </div>
      </div>

    </Modal>
  }
}

fn read_file(file: File, set_markdown_content: WriteSignal<String>) {
  let reader = FileReader::new().unwrap();
  let cloned_reader = reader.clone();

  let onload = Closure::wrap(Box::new(move |_event: web_sys::Event| {
    if let Ok(text) = cloned_reader.result() {
      let content = text.as_string().unwrap_or_default();
      set_markdown_content.update(|v| *v = content);
    }
  }) as Box<dyn FnMut(_)>);

  reader.set_onload(Some(onload.as_ref().unchecked_ref()));
  reader.read_as_text(&file).unwrap();
  onload.forget();
}

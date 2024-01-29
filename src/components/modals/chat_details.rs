use leptos::*;

use crate::{
  components::{mdown::Markdown, modals::Modal},
  UiMessage,
};

#[component]
pub fn ChatDetailsModal(
  show_modal: RwSignal<bool>,
  message: RwSignal<UiMessage>,
  status: RwSignal<String>,
) -> impl IntoView {
  let details = move || message().details.into_iter().enumerate();
  view! {
    <Modal id="chatDetails" show_modal=show_modal>
      <For
        each=details
        key=move |(_, (id, _))| { id.clone() }
        children=move |(idx, (id, item))| {
            let is_goal = id.starts_with("## Goal");
            let title = {
                let id = id.clone();
                move || id.clone()
            };
            let last_index = item.len() - 1;
            let dets = move || item.clone().into_iter().enumerate();
            view! {
              <div class="collapse collapse-arrow space-y-2 md:space-y-4">
                <input
                  type="radio"
                  name={
                      let id = id.clone();
                      move || id.clone()
                  }
                />

                <div class="prose prose-condensed prose-invert relative">
                  <div class="collapse-title space-x-2">
                    <Markdown
                      content=Signal::derive(title)
                      class="prose-headings:mt-0 prose-headings:text-lg prose-headings:text-inherit"
                    />
                  </div>
                  <div class="collapse-content">
                    <For each=dets key=move |(i, _)| { *i } let:detail>
                      <div class="px-4 pt-0">
                        <Markdown content=detail.1/>
                      </div>
                    </For>
                  </div>
                </div>
              </div>
              <Show when=move || { !status().is_empty() && last_index == idx && !is_goal }>
                <div class="flex items-center space-x-2 text-accent">
                  <span class="loading loading-infinity loading-lg text-accent"></span>
                  <div>{status()}</div>
                </div>
              </Show>
            }
        }
      />

    </Modal>
  }
}

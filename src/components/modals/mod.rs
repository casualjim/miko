mod file;
mod logout;

pub use file::FileModal;
use leptos::{html::Div, *};
use leptos_use::on_click_outside;
pub use logout::LogoutModal;

#[derive(Debug, Default, Clone)]
pub struct ModalsContext {
  pub show_logout_modal: RwSignal<bool>,
  pub show_signin_modal: RwSignal<bool>,
  pub show_settings_modal: RwSignal<bool>,
  pub show_welcome_modal: RwSignal<bool>,
}

#[component]
pub fn Modal(id: &'static str, show_modal: RwSignal<bool>, children: Children) -> impl IntoView {
  let modal_ref = create_node_ref::<Div>();

  let _ = on_click_outside(modal_ref, move |_| show_modal.set(false));

  let close_modal = move |_| show_modal.set(false);

  view! {
    <dialog id=id class="modal backdrop-blur w-screen" class:modal-open=show_modal>
      <div class="modal-box max-w-fit" node_ref=modal_ref>
        <div class="flex flex-col p-2">
          <form method="dialog flex-1">
            <button class="btn btn-sm btn-ghost absolute right-2 top-2" on:click=close_modal>
              "✕"
            </button>
          </form>
          <div class="flex-none mt-3">{children()}</div>
        </div>
      </div>
    </dialog>
  }
}

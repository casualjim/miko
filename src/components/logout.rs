use leptos::{
  html::{Dialog, Div},
  *,
};
use leptos_router::*;

use crate::app::handlers::Logout;

#[component]
pub fn Logout(show_modal: WriteSignal<bool>) -> impl IntoView {
  view! { <a on:click=move |_| show_modal.set(true)>"Log out"</a> }
}

#[derive(Debug, Clone)]
pub struct ModalAction(pub Memo<bool>, pub WriteSignal<bool>);

#[component]
pub fn LogoutModal(
  show_modal_state: ModalAction,
  logout: Action<Logout, Result<(), ServerFnError>>,
  modal_ref: NodeRef<Div>,
) -> impl IntoView {
  let ModalAction(show_modal, set_show_modal) = show_modal_state;

  let close_modal = move |_| set_show_modal.set(false);

  view! {
      <dialog id="logoutModal" class="modal backdrop-blur" class:modal-open=show_modal>
          <div class="modal-box" node_ref=modal_ref>
              <form method="dialog">
                  <button class="btn btn-sm btn-ghost absolute right-2 top-2" on:click=close_modal>
                      "✕"
                  </button>
              </form>
              <p>"Are you sure you want to log out?"</p>
              <div class="modal-action">
                  <ActionForm action=logout>
                      <button type="submit" class="btn btn-primary" on:click=close_modal>
                          "Log Out"
                      </button>
                  </ActionForm>
              </div>
          </div>
      </dialog>
  }
}

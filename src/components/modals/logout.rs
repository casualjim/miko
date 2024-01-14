use gloo_events::EventListener;
use leptos::{html::Div, *};
use leptos_router::*;
use wasm_bindgen::JsCast as _;
use web_sys::Node;

use crate::routes::authn::Logout;

pub type LogoutAction = Action<Logout, Result<(), ServerFnError>>;

#[component]
pub fn LogoutModal(show_modal: RwSignal<bool>, logout: LogoutAction) -> impl IntoView {
  let modal_ref = create_node_ref::<Div>();

  let show_modal_memo = create_memo(move |_| show_modal.get());

  create_effect(move |_| {
    EventListener::new(&leptos_dom::document(), "keyup", move |event| {
      let keyboard_event = event
        .clone()
        .dyn_into::<leptos::ev::KeyboardEvent>()
        .unwrap();
      if show_modal_memo() && keyboard_event.key_code() == 27 {
        show_modal.set(false);
      }
    })
    .forget();

    EventListener::new(&leptos_dom::document(), "mouseup", move |event| {
      let mouse_event = event.clone().dyn_into::<leptos::ev::MouseEvent>().unwrap();
      if let Some(modal_ref) = modal_ref.get_untracked() {
        if let Some(target) = mouse_event.target() {
          let target = target.dyn_ref::<Node>();
          if !modal_ref.contains(target) {
            show_modal.set(false);
          }
        }
      }
    })
    .forget();
  });

  let close_modal = move |_| show_modal.set(false);

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

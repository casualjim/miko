mod file;
mod logout;

pub use file::FileModal;
use gloo_events::EventListener;
use leptos::{html::Div, *};
pub use logout::LogoutModal;
use wasm_bindgen::{closure::Closure, JsCast as _};
use web_sys::{js_sys::Function, Node};

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

  create_effect(move |_| {
    let keyup_listener = Closure::wrap(Box::new(move |event: ev::Event| {
      let keyboard_event = event
        .clone()
        .dyn_into::<leptos::ev::KeyboardEvent>()
        .unwrap();
      if show_modal() && keyboard_event.key_code() == 27 {
        show_modal.set(false);
      }
    }) as Box<dyn FnMut(_)>);
    let keyup = keyup_listener.as_ref().clone().into();
    keyup_listener.forget();
    let mouseup_listener = Closure::wrap(Box::new(move |event: ev::Event| {
      let mouse_event = event.clone().dyn_into::<leptos::ev::MouseEvent>().unwrap();
      if let Some(modal_ref) = modal_ref.get_untracked() {
        if let Some(target) = mouse_event.target() {
          let target = target.dyn_ref::<Node>();
          if !modal_ref.contains(target) {
            show_modal.set(false);
          }
        }
      }
    }) as Box<dyn FnMut(_)>);
    let mouseup: Function = mouseup_listener.as_ref().clone().into();
    mouseup_listener.forget();

    leptos_dom::document()
      .add_event_listener_with_callback("keyup", &keyup)
      .unwrap();
    leptos_dom::document()
      .add_event_listener_with_callback("mouseup", &mouseup)
      .unwrap();

    on_cleanup(move || {
      leptos_dom::document()
        .remove_event_listener_with_callback("keyup", &keyup)
        .unwrap();
      leptos_dom::document()
        .remove_event_listener_with_callback("mouseup", &mouseup)
        .unwrap();
    });
  });

  let close_modal = move |_| show_modal.set(false);

  view! {
    <dialog id=id class="modal backdrop-blur" class:modal-open=show_modal>
      <div class="modal-box" node_ref=modal_ref>
        <form method="dialog">
          <button class="btn btn-sm btn-ghost absolute right-2 top-2" on:click=close_modal>
            "✕"
          </button>
        </form>
        {children()}
      </div>
    </dialog>
  }
}

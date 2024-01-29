mod chat_details;
mod file;
mod logout;

pub use chat_details::ChatDetailsModal;
pub use file::FileModal;
use leptos::{html::Div, *};
use leptos_use::{
  on_click_outside, use_scroll_with_options, ScrollBehavior, UseScrollOptions, UseScrollReturn,
};
pub use logout::LogoutModal;

#[derive(Debug, Default, Clone)]
pub struct ModalsContext {
  pub show_logout_modal: RwSignal<bool>,
  pub show_signin_modal: RwSignal<bool>,
  pub show_settings_modal: RwSignal<bool>,
  pub show_welcome_modal: RwSignal<bool>,
}

#[component]
pub fn Modal(
  id: &'static str,
  show_modal: RwSignal<bool>,
  #[prop(optional, default = false)] auto_scroll: bool,
  children: Children,
) -> impl IntoView {
  let modal_ref = create_node_ref::<Div>();

  let _ = on_click_outside(modal_ref, move |_| show_modal.set(false));

  let (is_at_bottom, set_is_at_bottom) = create_signal(false);
  let UseScrollReturn { set_y, .. } = use_scroll_with_options(
    modal_ref,
    UseScrollOptions::default().behavior(ScrollBehavior::Smooth),
  );

  let scroll_to_bottom = {
    let set_y = set_y.clone();
    move || {
      modal_ref
        .get()
        .map(|modal| {
          set_y(modal.scroll_height() as f64);
        })
        .unwrap_or_default()
    }
  };
  let handle_scroll = move || {
    modal_ref
      .get()
      .map(|modal| {
        let is_scrolled_to_bottom =
          modal.scroll_height() - modal.scroll_top() <= modal.client_height();
        set_is_at_bottom(is_scrolled_to_bottom);
      })
      .unwrap_or_default()
  };

  create_effect(move |_| {
    if is_at_bottom() && auto_scroll {
      scroll_to_bottom();
    }
  });

  let close_modal = move |ev: ev::MouseEvent| {
    ev.prevent_default();
    ev.stop_propagation();
    show_modal.set(false)
  };

  view! {
    <dialog id=id class="modal backdrop-blur w-screen" class:modal-open=show_modal>
      <div
        class="modal-box max-w-fit"
        node_ref=modal_ref
        on:scroll=move |_| {
            if auto_scroll {
                handle_scroll();
            }
        }
      >

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

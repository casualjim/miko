use ev::MouseEvent;
use leptos::*;
use uuid::Uuid;

use crate::components::sidebar::Sidebar;

#[component]
pub fn SidebarLayout<E, F>(sidebar: F, children: Children) -> impl IntoView
where
  E: IntoView,
  F: Fn() -> E + 'static,
{
  view! {
    <div class="flex-1 flex flex-row m-4">
      <aside class="flex-none flex flex-col w-[calc(100%-72px)] md:w-sidebar md:min-w-sidebar">{sidebar()}</aside>
      <main class="flex-1 flex flex-col p-2">{children()}</main>
    </div>
  }
}

#[component]
pub fn SingleLayout(children: Children) -> impl IntoView {
  view! { <main class="flex-1 flex flex-col m-4 p-2">{children()}</main> }
}

#[component]
pub fn CenteredLayout(children: Children) -> impl IntoView {
  view! { <main class="flex-1 flex flex-col m-4 p-2 items-center justify-center overflow-auto">{children()}</main> }
}

#[component]
pub fn SidebarLayoutWithHeader(
  show_logout: RwSignal<bool>,
  is_dark: ReadSignal<bool>,
  chat_id: ReadSignal<Option<Uuid>>,
  #[prop(into)] on_toggle_theme: Callback<ev::Event>,
  children: Children,
) -> impl IntoView {
  let (sidebar_open, set_sidebar_open) = create_signal(true);
  let (close_button_hover, set_close_button_hover) = create_signal(false);

  let shared_css = "fixed bottom-0 left-0 top-0 z-10 h-full border-r-2 bg-base-900 transition-all duration-300 ease-in-out md:relative border-base-800";
  let clazz = move || {
    if sidebar_open() {
      format!("{shared_css} w-[calc(100%-35px)] md:w-sidebar md:min-w-sidebar")
    } else {
      format!("{} w-0 min-w-0 border-none shadow-none", shared_css)
    }
  };

  view! {
      <div class="relative flex h-screen overflow-hidden m-4">
          <aside
            class=clazz
            class:border-base-500=close_button_hover
          >

              <Sidebar chat_id sidebar_open is_hovering=close_button_hover show_logout is_dark on_toggle_theme/>
              <CloseSidebarButton
                  click=move |_| set_sidebar_open.update(|value| *value = !*value)
                  sidebar_is_open=sidebar_open
                  is_hovering=close_button_hover
                  set_is_hovering=set_close_button_hover
              />

          </aside>
          {children()}
      </div>
  }
}

#[component]
fn CloseSidebarButton(
  sidebar_is_open: ReadSignal<bool>,
  is_hovering: ReadSignal<bool>,
  set_is_hovering: WriteSignal<bool>,

  #[prop(into)] click: Callback<MouseEvent>,
) -> impl IntoView {
  let line = move || {
    if is_hovering() {
      "M16 6.18872L12 16L16 25.8113"
    } else {
      "M16.0002 6.18872L15.9902 16L16.0002 25.8113"
    }
  };

  let clazz = move || {
    if is_hovering() {
      "stroke-white"
    } else {
      "stroke-neutral-content"
    }
  };

  let rotated = move || {
    if sidebar_is_open() {
      "rotate(0 0 0)"
    } else {
      "rotate(180 0 0)"
    }
  };

  view! {
    <button
      class="absolute -right-8 top-1/2 z-10 cursor-pointer"
      on:click=click
      on:mouseenter=move |_| { set_is_hovering.set(true) }
      on:mouseleave=move |_| { set_is_hovering.set(false) }
    >
      <svg width="32" height="32" viewbox="0 0 32 32" fill="none" transform=rotated xmlns="http://www.w3.org/2000/svg">
        <path class=clazz d=line stroke-width="4" stroke-linecap="round" stroke-linejoin="round"></path>
      </svg>
    </button>
  }
}

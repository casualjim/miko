use leptos::*;
use leptos_router::*;

use crate::{
  components::{logo::Logo, logout::Logout},
  models::CurrentUser,
};

#[component]
pub fn Sidebar(
  sidebar_open: ReadSignal<bool>,
  show_modal: WriteSignal<bool>,
  is_hovering: ReadSignal<bool>,
) -> impl IntoView {
  let hover_transition = move || sidebar_open() && is_hovering();
  let navigate = use_navigate();
  let user = expect_context::<ReadSignal<CurrentUser>>();

  view! {
      <div
          class="flex h-full flex-col justify-between transition-opacity"
          class:opacity-50=hover_transition
          class:delay-0=hover_transition
          class:duration-300=hover_transition
          class:pointer-events-none=move || !sidebar_open()
          class:opacity-0=move || !sidebar_open()
          class:delay-0=move || !sidebar_open()
          class:duration-0=move || !sidebar_open()
      >
      <div class="flex h-full animate-fade-in flex-col justify-between opacity-0" style:animationDelay=move ||{ if sidebar_open() { "150ms" } else { "0ms"} }>
        <div class="flex h-full flex-col justify-between">
          <div class:space-y-6=move || { false  }>
            <header on:click=move |_| {navigate("/", Default::default()) }>
              <div class="flex flex-row h-full items-center text-accent text-lg lg:text-2xl hover:cursor-pointer">
                <Logo class="w-[75px] h-[75px] cursor-pointer p-4 transition-opacity hover:opacity-50" />
                <h1>"Miko"</h1>
              </div>
            </header>
          </div>
          <div class="relative flex w-full items-center justify-between space-x-2 p-4">
            <ul class="menu p-2 bg-base-500">
            { move || { match user() {
              CurrentUser(Some(_user)) => {
                view! {
                  <li>
                      <A href="/">"Home"</A>
                  </li>
                  <li>
                      <A href="/about">"About"</A>
                  </li>
                  <li>
                      <Logout show_modal=show_modal/>
                  </li>
                }.into_view()
              }
              _ => {
                view! {
                  <li>
                    <a rel="external" href="/oauth/start">"Login"</a>
                  </li>
                }.into_view()
              }
            }
            }}

            </ul>
          </div>
          <div class="flex items-center space-x-1 text-lg text-white">
          </div>
        </div>
      </div>

      </div>
  }
}

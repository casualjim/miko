use leptos::*;
use leptos_router::*;
use phosphor_leptos::{GithubLogo, IconWeight, NotePencil};

use crate::{
  components::{account_dropdown::AccountDropdown, logo::Logo, workspace::Workspace},
  models::CurrentUser,
  routes::chats::get_chats,
};

#[component]
pub fn Sidebar(
  sidebar_open: ReadSignal<bool>,
  show_logout: RwSignal<bool>,
  is_hovering: ReadSignal<bool>,
) -> impl IntoView {
  let hover_transition = move || sidebar_open() && is_hovering();
  let navigate = use_navigate();
  let user = expect_context::<ReadSignal<CurrentUser>>();
  let chats = create_resource(|| (), |_| async { get_chats().await });

  let empty_chats = move || {
    view! {
      <div class="mt-1 flex cursor-pointer flex-col items-center justify-center space-y-2 rounded-lg border-2 border-dashed border-base-500 p-7 text-center transition-colors duration-300 hover:border-cyan-500 hover:bg-base-950 hover:text-cyan-500">
        <NotePencil size="24" class="text-[currentColor]"/>
        <p class="leading-regular text-xs text-base-500">"You currently have no chats"</p>
      </div>
    }
  };

  let chat_view = move || {
    chats.and_then(|chats| {
      chats
        .iter()
        .map(|chats| {
          let id = format!("/chat/{}", chats.id);
          view! {
            <li>
              <a href=id>{chats.title.clone()}</a>
            </li>
          }
        })
        .collect_view()
    })
  };

  let chats_view = move || {
    chats.and_then(|chats| {
      let is_empty = chats.is_empty();

      view! {
        <Show when=move || { !is_empty } fallback=empty_chats>
          <ul class="menu p-2 bg-base-500">{chat_view()}</ul>
        </Show>
      }
    })
  };
  let chats_loading = chats.loading();

  let is_authenticated = move || user().is_authenticated();

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
          <div class:space-y-6=is_authenticated>
            <header on:click=move |_| {navigate("/", Default::default()) }>
              <div class="flex flex-row h-full items-center text-accent text-lg lg:text-2xl hover:cursor-pointer">
                <Logo class="w-[75px] h-[75px] cursor-pointer p-4 transition-opacity hover:opacity-50" />
                <h1>"Miko"</h1>
              </div>
            </header>
            <Show when=is_authenticated >
              <div class="space-y-1 px-2">
                  <div class="flex w-full items-center justify-between space-x-1 px-3">
                    <div class="text-xs uppercase tracking-widest text-base-500">
                      "Recent Chats"
                    </div>
                    <Show when=move || !chats_loading()>
                      <button class="btn btn-ghost">
                        <NotePencil size="18" weight=IconWeight::Bold />
                      </button>
                    </Show>
                  </div>
                  <Suspense fallback=move || { view! { <div class="skeleton mx-2 h-[20vh] w-[calc(100%-1rem)]" /> }} >
                    { chats_view() }
                  </Suspense>
                </div>
            </Show>
            <Workspace chats=chats />
          </div>

          <SidebarBottom show_logout=show_logout />
        </div>
      </div>

    </div>
  }
}

#[component]
fn ChatHistory() -> impl IntoView {
  view! {}
}

#[component]
fn SidebarBottom(show_logout: RwSignal<bool>) -> impl IntoView {
  view! {
    <div class="relative flex w-full items-center justify-between space-x-2 p-4">
      <AccountDropdown show_logout=show_logout/>
      // <ul class="menu p-2 bg-base-500">
      // {move || {
      // match user() {
      // CurrentUser(Some(_user)) => {
      // view! {
      // <li>
      // <A href="/">"Home"</A>
      // </li>
      // <li>
      // <A href="/about">"About"</A>
      // </li>
      // <li>
      // <Logout show_modal=show_modal/>
      // </li>
      // }
      // .into_view()
      // }
      // _ => {
      // view! {
      // <li>
      // <a rel="external" href="/oauth/start">
      // "Login"
      // </a>
      // </li>
      // }
      // .into_view()
      // }
      // }
      // }}

      // </ul>
      <div class="flex items-center space-x-1 text-lg text-white">
        // <button class="btn btn-ghost hover:text-primary">
        // <DiscordLogo size="20"/>
        // </button>
        <a class="hover:text-primary" href="https://github.com/casualjim/miko" rel="external" target="_blank">
          <GithubLogo size="20"/>
        </a>
      </div>
    </div>
  }
}

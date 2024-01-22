#[cfg(feature = "hydrate")] use gloo_events::EventListener;
use leptos::{html::Input, logging::log, *};
use leptos_router::*;
use phosphor_leptos::{GithubLogo, IconWeight, NotePencil, PencilSimple, TrashSimple};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{Event, Node, SubmitEvent};

use crate::{
  components::{account_dropdown::AccountDropdown, logo::Logo, workspace::Workspace},
  models::{ChatInfo, CurrentUser, EditChat},
  routes::chats::UpdateChatTitle,
  ChatDeleteAction, ChatResourceContext, ChatState, ChatUpdateTitleAction,
};

#[component]
pub fn Sidebar(
  chat_id: ReadSignal<Option<Uuid>>,
  sidebar_open: ReadSignal<bool>,
  show_logout: RwSignal<bool>,
  is_hovering: ReadSignal<bool>,
  is_dark: ReadSignal<bool>,
  #[prop(into)] on_toggle_theme: Callback<ev::Event>,
) -> impl IntoView {
  let hover_transition = move || sidebar_open() && is_hovering();
  let navigate = use_navigate();
  let user = expect_context::<ReadSignal<CurrentUser>>();
  let chat_state = expect_context::<ChatState>();

  let edit_chat = chat_state.edit_chat;
  let active_chat = chat_state.active_chat;

  let ChatResourceContext(chats, create_chat, _, _) = expect_context();

  let chats_loading = chats.loading();

  let is_authenticated = move || user().is_authenticated();

  create_effect(move |_| {
    let cid = chat_id();
    if cid != active_chat.get() {
      active_chat.set(cid);
    }
    log!("is_active_selected_chat: chat_id={chat_id:?} active_chat={active_chat:?} edit_chat={edit_chat:?}", chat_id=chat_id(), active_chat=active_chat.get(), edit_chat=edit_chat.get());
  });

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
                  <div class="flex w-full items-center justify-between space-x-1 px-3 text-neutral-content p-1">
                    <div class="text-xs uppercase tracking-widest">
                      "Recent Chats"
                    </div>
                    <Show when=move || !chats_loading()>
                      <ActionForm action=create_chat>
                        <input type="hidden" name="id" prop:value=move || Uuid::new_v4().to_string() />
                        <button class="btn-link text-neutral-content hover:text-primary">
                          <NotePencil size="18" weight=IconWeight::Bold />
                        </button>
                      </ActionForm>
                    </Show>
                  </div>

                  <ChatList edit_chat active_chat />
                </div>
                <Workspace chats chat_id />
            </Show>
          </div>
          <SidebarBottom show_logout is_dark on_toggle_theme />
        </div>
      </div>

    </div>
  }
}

#[component]
fn ChatList(
  edit_chat: RwSignal<Option<EditChat>>,
  active_chat: RwSignal<Option<Uuid>>,
) -> impl IntoView {
  let ChatResourceContext(chat_resource, _, _, _) = expect_context::<ChatResourceContext>();

  let empty_chats = move || {
    view! {
      <div class="mt-1 flex cursor-pointer flex-col items-center justify-center space-y-2 rounded-lg border-2 border-dashed text-neutral-content border-neutral-content p-7 text-center transition-colors duration-300 hover:border-primary hover:bg-base-950 hover:text-primary">
        <NotePencil size="24" class="text-[currentColor]"/>
        <p class="leading-regular text-xs text-[currentColor]">"You currently have no chats"</p>
      </div>
    }
  };

  let (is_empty, set_is_empty) = create_signal(true);
  create_effect(move |_| {
    chat_resource.and_then(move |chats| {
      set_is_empty.set(chats.is_empty());
    });
  });

  view! {
    <div class="h-full max-h-[30vh] space-y-0.5 overflow-y-auto [scrollbar-gutter:stable]">
      <Show when=move || { !is_empty() } fallback=empty_chats>
        <Suspense fallback=move || {
            view! { <div class="skeleton mx-2 h-[20vh] w-[calc(100%-1rem)]"></div> }
        }>
          // <ul class="menu p-2 bg-base-500 w-full">{chat_view()}</ul>
          <div class="px-2">
            <ChatListItems edit_chat active_chat/>
          </div>
        </Suspense>
      </Show>
    </div>
  }
}

#[component]
fn ChatListItems(
  edit_chat: RwSignal<Option<EditChat>>,
  active_chat: RwSignal<Option<Uuid>>,
) -> impl IntoView {
  let ChatResourceContext(chat_resource, _, delete_chat, update_title) = expect_context();

  view! {
    {move || {
        chat_resource
            .and_then(move |chats| {
                chats
                    .iter()
                    .map(|chat| {
                        let info = chat.into();
                        view! { <ChatListItem chat=info update_title delete_chat edit_chat active_chat/> }
                    })
                    .collect_view()
            })
    }}
  }
}

#[component]
fn ChatListItem(
  chat: ChatInfo,
  update_title: ChatUpdateTitleAction,
  delete_chat: ChatDeleteAction,
  edit_chat: RwSignal<Option<EditChat>>,
  active_chat: RwSignal<Option<Uuid>>,
) -> impl IntoView {
  let (title, _) = create_signal(chat.name);
  let id = chat.id;
  let is_active_selected_chat = move || Some(chat.id) == active_chat();
  let is_selected_chat = move || edit_chat().as_ref().map_or(false, |chat| chat.id == id);
  let (is_hovering, set_is_hovering) = create_signal(false);
  let input_ref = create_node_ref::<Input>();

  let submit_form = move |ev: SubmitEvent| {
    ev.prevent_default();

    let params = UpdateChatTitle::from_event(&ev).expect("failed to parse form data");
    update_title.dispatch(params);
    edit_chat.update(|v| *v = None);
  };

  create_effect(move |_| {
    if input_ref().is_some() {
      let input = input_ref().unwrap();
      #[allow(unused_variables)]
      let click_outside = move |ev: &Event| {
        log!("click_outside: {:?}", ev);
        let mouse_event = ev.clone().dyn_into::<ev::MouseEvent>().unwrap();
        if let Some(target) = mouse_event.target() {
          let target = target.dyn_ref::<Node>();
          if !input.contains(target) {
            edit_chat.update(|v| *v = None);
            log!("click_outside edit_chat={:?}", edit_chat());
          }
        }
      };

      #[cfg(feature = "hydrate")]
      EventListener::new(&leptos_dom::document(), "mousedown", click_outside).forget();
    }
  });

  view! {
    <div
      class="relative w-full cursor-pointer overflow-x-hidden text-ellipsis whitespace-nowrap rounded p-1 text-sm transition-colors duration-300"
      class:pr-14=is_active_selected_chat
      class:bg-base-300=is_active_selected_chat
      class:text-accent=is_hovering
      class:bg-base-200=is_hovering
    >
      <Show
        when=is_selected_chat
        fallback=move || {
            let navigate = use_navigate();
            let detail_url = format!("/chat/{}", id);
            view! {
              <div
                class="flex-none"
                on:mouseenter=move |_| set_is_hovering.set(true)
                on:mouseleave=move |_| set_is_hovering.set(false)
                on:click=move |_| {
                    if edit_chat().is_none() {
                        navigate(&detail_url, Default::default());
                    }
                }
              >

                {title()}
              </div>
            }
        }
      >

        <div>
          <ActionForm action=update_title on:submit=submit_form>
            <input type="hidden" name="id" prop:value=id.to_string()/>
            <input
              type="text"
              name="title"
              class="flex-none rounded-lg w-full border-none bg-neutral p-1 outline-none focus:outline-none focus:border-none"
              node_ref=input_ref
              value={title()}
            />
            <button class="hidden" type="submit"/>
          </ActionForm>

        </div>
      </Show>
      <Show when=move || !is_selected_chat()>
        <div class="absolute right-1 top-1/2 -translate-y-1/2 transform animate-fade-in items-center flex flex-row">
          <form class="px-1">
            <button
              class="btn-link text-neutral-content hover:text-primary"
              on:click={
                  move |ev| {
                      let title = title().unwrap_or_else(|| "New Session".to_string());
                      let end = title.len() as u32;
                      ev.prevent_default();
                      edit_chat.set(Some(EditChat { id, title }));
                      input_ref().map(|input| {
                        input.set_selection_range(end, end).expect("failed to move to the end of the input");
                        input.focus()
                      });
                  }
              }
            >

              <PencilSimple size="16" weight=IconWeight::Bold/>
            </button>
          </form>
          <ActionForm action=delete_chat>
            <button class="btn-link text-neutral-content hover:text-primary">
              <TrashSimple size="16" weight=IconWeight::Bold/>
            </button>
            <input type="hidden" name="id" prop:value=chat.id.to_string()/>
          </ActionForm>
        </div>
      </Show>
    </div>
  }
}

#[component]
fn SidebarBottom(
  show_logout: RwSignal<bool>,
  is_dark: ReadSignal<bool>,
  #[prop(into)] on_toggle_theme: Callback<ev::Event>,
) -> impl IntoView {
  view! {
    <div class="relative flex w-full items-center justify-between space-x-2 p-4">
      <AccountDropdown show_logout=show_logout/>
      <div class="flex items-center space-x-1 text-lg p-1">
        <ThemeToggle is_dark on_toggle_theme/>
        <a
          class="hover:text-primary"
          href="https://git.wagyu.icu/casualjim/miko"
          rel="external noopener noreferrer"
          target="_blank"
        >
          <GithubLogo size="20"/>
        </a>
      </div>
    </div>
  }
}

#[component]
fn ThemeToggle(
  is_dark: ReadSignal<bool>,
  #[prop(into)] on_toggle_theme: Callback<ev::Event>,
) -> impl IntoView {
  view! {
    <label class="swap swap-rotate">

      <input type="checkbox" class="theme-controller" value="night" checked=is_dark on:change=on_toggle_theme/>

      <svg class="swap-on fill-current w-5 h-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
        <path d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z"></path>
      </svg>

      <svg class="swap-off fill-current w-5 h-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
        <path d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"></path>
      </svg>

    </label>
  }
}

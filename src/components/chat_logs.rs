use leptos::{html::Div, *};
use leptos_use::{use_scroll_with_options, ScrollBehavior, UseScrollOptions, UseScrollReturn};
use phosphor_leptos::ArrowSquareRight;

use crate::{
  components::{logo::Logo, mdown::Markdown},
  models::{ChatLog, CurrentUser},
  ShowChatDetailsModal, UiMessage,
};

#[component]
pub fn ChatLogs(
  chat_name: ReadSignal<String>,
  chat_logs: ReadSignal<Vec<ChatLog>>,
  is_running: ReadSignal<bool>,
) -> impl IntoView {
  let list_container_ref = create_node_ref::<Div>();
  let messages = create_memo(move |_| chat_logs_for_ui(chat_logs()));
  let enumerated_messages = move || messages().into_iter().enumerate();
  let user = use_context::<CurrentUser>();
  let has_image_and_email = if let Some(user) = user.as_ref() {
    user.picture().is_some() && user.email().is_some()
  } else {
    false
  };

  let ShowChatDetailsModal(show_details_modal, chat_details_message, chat_status) =
    expect_context();

  let (log_details, set_log_details) = create_signal(ChatLogDetails::default());

  let selected_index = create_memo(move |_| log_details().index);

  create_effect(move |_| {
    if log_details().open {
      show_details_modal.set(true);
    }
  });

  create_effect(move |_| {
    if !show_details_modal() {
      set_log_details.update(|v| v.open = false);
      return;
    }
  });

  create_effect(move |_| {
    let index = selected_index();
    let messages = messages();
    if let Some(message) = messages.get(index) {
      chat_details_message.set(message.clone());
    } else {
      show_details_modal.set(false);
    }
  });

  let (is_at_bottom, set_is_at_bottom) = create_signal(false);
  let UseScrollReturn { set_y, .. } = use_scroll_with_options(
    list_container_ref,
    UseScrollOptions::default().behavior(ScrollBehavior::Smooth),
  );

  let scroll_to_bottom = {
    let set_y = set_y.clone();
    move || {
      list_container_ref
        .get()
        .map(|container| {
          set_y(container.scroll_height() as f64);
        })
        .unwrap_or_default()
    }
  };
  let handle_scroll = move || {
    list_container_ref
      .get()
      .map(|container| {
        let is_scrolled_to_bottom =
          container.scroll_height() - container.scroll_top() <= container.client_height();
        set_is_at_bottom(is_scrolled_to_bottom);
      })
      .unwrap_or_default()
  };

  create_effect(move |_| {
    let _ = chat_logs();
    if is_at_bottom() {
      scroll_to_bottom();
    }
  });

  use leptos::logging::log;
  create_effect(move |_| {
    log!("is_at_bottom: {}", is_at_bottom());
    log!("chat name: {}", chat_name());
  });

  view! {
    <div class="flex h-20 items-center justify-center border-neutral border-b-2 md:h-12 text-neutral-content">
      <div>{chat_name}</div>
    </div>
    <div
      class="w-full flex-1 items-center space-y-6 overflow-y-auto overflow-x-clip px-2 py-3 text-left [scrollbar-gutter:stable]"
      node_ref=list_container_ref
      on:scroll=move |_| {
          handle_scroll();
      }
    >

      <For
        each=enumerated_messages
        key=move |(idx, _)| { *idx }
        children=move |(idx, message)| {
            let user = user.clone();
            let user_name = user.as_ref().and_then(|user| user.name_opt()).unwrap_or_else(|| "User".to_string());
            let picture = user.as_ref().and_then(|user| user.picture());
            let has_miko_message = message.miko_message.as_ref().is_some();
            view! {
              <div class="m-auto w-full max-w-[56rem] self-center">
                <div class="group relative flex w-full animate-slide-down items-start space-x-3 rounded-lg p-2 pb-10 opacity-0 transition-colors duration-300">
                  <Show
                    when=move || has_image_and_email
                    fallback=move || view! { <div class="h-8 w-8 rounded-full bg-primary"></div> }
                  >
                    <img class="h-8 w-8 rounded-full bg-primary" src=picture.clone()/>
                  </Show>
                  <div class="w-full max-w-[calc(100vw-84px)] space-y-2 pt-1 md:max-w-[49rem]">
                    <div class="flex items-center justify-between">
                      <span class="font-medium">{user_name}</span>
                    </div>
                    <div class="prose prose-invert w-full max-w-none">
                      <Markdown content=message.user_message/>
                    </div>
                  </div>
                </div>
                <div class="m-auto w-full max-w-[56rem] self-center">
                  <div class="group relative flex w-full animate-slide-down items-start space-x-3 rounded-lg p-2 pb-10 opacity-0 transition-colors duration-300">
                    <Logo class="!w-8 !min-w-[2rem]"/>
                    <div class="w-full max-w-[calc(100vw-84px)] space-y-2 pt-1 md:max-w-[49rem]">
                      <div class="flex items-center justify-between">
                        <span class="font-medium">{"Miko"}</span>
                        <Show when=move || { !is_running() }>
                          <button
                            class="group/button flex items-center space-x-2 text-cyan-500 hover:text-cyan-400"
                            on:click=move |_| {
                                set_log_details(ChatLogDetails {
                                    index: idx,
                                    open: true,
                                });
                            }
                          >

                            <div class="font-regular text-xs group-hover/button:underline">{"View Details"}</div>
                            <ArrowSquareRight size="24"/>
                          </button>
                        </Show>
                      </div>
                      <Show when=move || { has_miko_message }>
                        <Markdown
                          content=message.miko_message.clone().unwrap_or_default()
                          class="prose prose-invert w-full max-w-none"
                        />
                      </Show>
                      <Show when=move || { !has_miko_message && is_running() && messages().len() - 1 == idx }>
                        <div class="flex items-center space-x-2 text-accent">
                          <span class="loading loading-infinity loading-lg text-accent"></span>
                          <div class="group/loading flex cursor-pointer items-center space-x-2 text-cyan-500 transition-all duration-500 hover:text-cyan-700">
                            <div
                              class="group-hover/loading:underline"
                              on:click=move |_| {
                                  set_log_details(ChatLogDetails {
                                      index: idx,
                                      open: true,
                                  });
                              }
                            >

                              {chat_status}
                            </div>
                          </div>
                        </div>
                      </Show>
                      <Show when=move || { !has_miko_message && !is_running() }>
                        <Markdown
                          content="It seems I have been interrupted, please try again."
                          class="prose prose-invert w-full max-w-none"
                        />
                      </Show>
                    </div>
                  </div>
                </div>
              </div>
            }
        }
      />

    </div>
  }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct ChatLogDetails {
  pub index: usize,
  pub open: bool,
}

fn chat_logs_for_ui(mut value: Vec<ChatLog>) -> Vec<UiMessage> {
  value.sort_by(|a, b| a.created_at.cmp(&b.created_at));

  let is_empty = value.is_empty();

  value
    .into_iter()
    .fold(vec![], |acc, item| add_chat_log_to_ui(acc, item, is_empty))
}

fn add_chat_log_to_ui(mut acc: Vec<UiMessage>, item: ChatLog, is_empty: bool) -> Vec<UiMessage> {
  if item.user == "user" {
    acc.push(UiMessage {
      user_message: item.title,
      ..Default::default()
    });
    return acc;
  }

  if is_empty {
    return acc;
  }

  acc.push(UiMessage::default());
  if item.title.is_empty() || item.title == "{}" {
    acc
      .last_mut()
      .map(|m| m.miko_message = Some("An error has occured, please try again".to_string()));
    return acc;
  }

  if item.title.starts_with("#") {
    acc.last_mut().map(|m| m.miko_message = Some(item.title));
  } else {
    if item.title.starts_with("## ") {
      acc.last_mut().map(|m| m.details.insert(item.title, vec![]));
    } else {
      acc.last_mut().map(|m| {
        m.details.last_mut().map(|(_, details)| {
          let content = if let Some(content) = item.content {
            format!("{}\n{}", item.title, content)
          } else {
            item.title
          };
          details.push(content);
        });
      });
    }
  }
  acc
}

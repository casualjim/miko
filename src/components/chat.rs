use leptos::{logging::log, *};
use leptos_router::*;
use phosphor_leptos::{ArrowRight, IconWeight, UploadSimple};
use uuid::Uuid;

use crate::{
  components::{chat_logs::ChatLogs, example_prompts::ExamplePrompts, logo::Logo},
  models::ChatLog,
  ChatResourceContext, OnGoalSubmit,
};

#[component]
pub fn Chat(
  id: Signal<Option<Uuid>>,
  is_running: ReadSignal<bool>,
  is_starting: ReadSignal<bool>,
  #[prop(into, default = Callback::from(|_|{}))] on_upload: Callback<web_sys::File>,
) -> impl IntoView {
  let ChatResourceContext {
    resource: chat_resource,
    create_chat: create_action,
    submit_goal: on_goal_submit,
    ..
  } = expect_context::<ChatResourceContext>();
  let should_show_example_prompts = move || id().is_none();

  let container_class = move || {
    let mut class = "mt-4 flex w-full space-y-4".to_string();
    if should_show_example_prompts() {
      class.push_str(" flex-col-reverse space-y-reverse px-4 md:px-8 lg:px-4");
    } else {
      class.push_str(" mx-auto max-w-[56rem] flex-col px-4");
    }
    class
  };

  let form_class = move || {
    let mut class = "mb-4 flex w-full items-center justify-center gap-4 self-center".to_string();
    if should_show_example_prompts() {
      class.push_str(" max-w-[42rem]");
    } else {
      class.push_str(" max-w-[56rem]");
    }
    class
  };

  let chat_is_loading = chat_resource.loading();
  let message = create_rw_signal("".to_string());
  let update_message_on_change = move |ev: web_sys::Event| {
    let val = event_target_value(&ev);
    message.update(|msg| *msg = val);
  };

  let (chat_name, set_chat_name) = create_signal("".to_string());
  let (chat_logs, set_chat_logs) = create_signal(Vec::new());
  create_effect(move |_| {
    if let Some(id) = id() {
      if let Some(Ok(chat)) = chat_resource.get() {
        let active_chat = chat.iter().find(|chat| chat.id == id).map(|chat| {
          let name = chat
            .title
            .clone()
            .unwrap_or_else(|| "New Session".to_string());
          set_chat_name.update(|v| *v = name);
          set_chat_logs.update(|v| *v = chat.logs.clone());
          chat
        });
        if active_chat.is_none() {
          set_chat_name.update(|v| v.clear());
          set_chat_logs.update(|v| v.clear());
        }
      }
    } else {
      set_chat_logs.update(|v| v.clear());
    }
  });

  let handle_goal_submit = move |prompt: String| {
    if prompt.is_empty() {
      // TODO: Fix error handling
      return;
    }
    message.update(|msg| msg.clear());
    on_goal_submit.dispatch(prompt);
  };

  let input_keydown = move |ev: web_sys::KeyboardEvent| {
    if ev.key() == "Enter" && !is_starting() && !is_running() {
      handle_goal_submit(message());
    }
  };

  view! {
    <main
      class="relative flex h-full w-full flex-col"
      class:items-center=should_show_example_prompts
      class:justify-center=should_show_example_prompts
    >
      <Show
        when=move || !should_show_example_prompts()
        fallback=move || {
            view! {
              <div class="w-full flex flex-col items-center text-center">
                <Logo class="mt-16 w-16 flex-col text-center"/>
              </div>
            }
        }
      >

        <ChatLogs chat_name chat_logs is_running/>
      </Show>
      <div class=container_class>
        <Show when=should_show_example_prompts>
          <ExamplePrompts on_click=handle_goal_submit/>
        </Show>
        <div class=form_class>
          <ActionForm action=create_action>
            <TextInput
              name="content"
              placeholder="Ask Miko anything..."
              value=message
              is_running=chat_is_loading
              on_change=update_message_on_change
              on_keydown=input_keydown
            />
          </ActionForm>
        </div>
      </div>
    </main>
  }
}

#[component]
pub fn TextInput<KDF: Fn(web_sys::KeyboardEvent) + 'static>(
  #[prop(into)] name: MaybeSignal<&'static str>,
  #[prop(into)] placeholder: MaybeSignal<&'static str>,
  #[prop(into)] value: RwSignal<String>,
  #[prop(into)] is_running: MaybeSignal<bool>,
  #[prop(optional, into)] disabled: MaybeSignal<bool>,
  #[prop(into)] on_change: Callback<web_sys::Event>,
  on_keydown: KDF,
) -> impl IntoView {
  let can_send = move || !value.get().is_empty();

  view! {
    <div class="form-control w-full space-y-1">
      <div class="flex border-neutral border m-2 rounded-lg cursor-text hover:border-accent">
        <button type="button" class="flex-1 hover:btn-accent btn-neutral btn btn-square">
          <UploadSimple size="20" class="text-[currentColor]" />
        </button>
        <input
          type="text"
          name=name
          class="input flex-none border-none bg-transparent outline-none focus:outline-none focus:border-none focus:ring-3 w-[40rem]"
          placeholder=placeholder
          on:change=on_change
          on:keydown=on_keydown
          prop:value=value
          prop:disabled=disabled
          class:cursor-default=disabled
          class:opacity-50=disabled
          class:cursor-text=move || !disabled()
        />
        <ChatSendButton is_running enabled=Signal::derive(can_send) />
      </div>
    </div>
  }
}

#[component]
fn ChatSendButton(
  #[prop(into)] is_running: MaybeSignal<bool>,
  #[prop(into)] enabled: MaybeSignal<bool>,
) -> impl IntoView {
  view! {
    <Show
      when=move || { !is_running() }
      fallback=move || view! { <span class="loading loading-infinity text-accent"></span> }
    >
      <button type="submit" class="hover:btn-accent btn btn-neutral btn-square flex-1" prop:disabled=enabled>
        <ArrowRight weight=IconWeight::Bold/>
      </button>
    </Show>
  }
}

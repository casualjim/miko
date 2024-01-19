use leptos::*;
use leptos_router::*;
use phosphor_leptos::{ArrowRight, IconWeight, UploadSimple};
use uuid::Uuid;

use crate::{components::logo::Logo, ChatResourceContext};

#[component]
pub fn Chat(id: Signal<Option<Uuid>>) -> impl IntoView {
  let ChatResourceContext(chat_resource, create_action, _, _) =
    expect_context::<ChatResourceContext>();
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

  view! {
    <main
      class="relative flex h-full w-full flex-col"
      class:items-center=should_show_example_prompts
      class:justify-center=should_show_example_prompts
    >
      <Show
        when=should_show_example_prompts
        fallback=move || view! { <Logo class="mb-16 w-10 transition-opacity hover:opacity-50"/> }
      >
        <div>"Chat logs go here"</div>
      </Show>
      <div class=container_class>
        <Show when=should_show_example_prompts>
          <div>"Example prompts would go here"</div>
        </Show>
        <div class=form_class>
          <ActionForm action=create_action>
            <TextInput
              name="content"
              placeholder="Ask Miko anything..."
              value=message
              is_running=chat_is_loading
              on_change=update_message_on_change
            />
          </ActionForm>
        </div>
      </div>
    </main>
  }
}

#[component]
pub fn TextInput(
  #[prop(into)] name: MaybeSignal<&'static str>,
  #[prop(into)] placeholder: MaybeSignal<&'static str>,
  #[prop(into)] value: RwSignal<String>,
  #[prop(into)] is_running: MaybeSignal<bool>,
  #[prop(optional, into)] disabled: MaybeSignal<bool>,
  #[prop(into)] on_change: Callback<web_sys::Event>,
) -> impl IntoView {
  let can_send = move || !value.get().is_empty();

  view! {
    <div class="form-control w-full space-y-1">
      <div class="flex border-primary border pl-2 m-2 rounded-lg cursor-text hover:border-primary hover:bg-base-300">
        <button type="button" class="flex-1 hover:text-primary">
          <UploadSimple size="20" class="text-[currentColor]" />
        </button>
        <input
          type="text"
          name=name
          class="input flex-none border-none bg-transparent outline-none focus:outline-none focus:border-none focus:ring-3"
          placeholder=placeholder
          on:change=on_change
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
      fallback=move || view! { <span class="loading loading-infinity text-primary"></span> }
    >
      <button type="submit" class="btn btn-neutral flex-1" prop:disabled=enabled>
        <ArrowRight weight=IconWeight::Bold/>
      </button>
    </Show>
  }
}

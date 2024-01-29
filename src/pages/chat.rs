use leptos::*;
use leptos_router::*;
use uuid::Uuid;

use crate::components::chat::Chat;

#[derive(Debug, Clone, Params, Eq, PartialEq)]
pub struct ChatPageParams {
  pub id: Uuid,
}

#[component]
pub fn ChatPage(set_chat_id: WriteSignal<Option<Uuid>>) -> impl IntoView {
  let params = use_params::<ChatPageParams>();
  let id = move || params.with(|params| params.as_ref().map(|params| params.id).ok());

  let (is_running, set_is_running) = create_signal(false);
  let (is_starting, set_is_starting) = create_signal(false);

  create_effect(move |_| {
    set_chat_id.update(move |value| *value = id());
  });
  view! { <Chat id=Signal::derive(id) is_running is_starting/> }
}

use indexmap::IndexMap;
use leptos::*;
use uuid::Uuid;

use crate::{
  models::{Chat, EditChat, UploadedFile},
  routes::chats::{get_chats, CreateChat, DeleteChat, UpdateChatTitle},
};

pub type ChatResource = Resource<(usize, usize, usize, usize), Result<Vec<Chat>, ServerFnError>>;
pub type ChatCreateAction = Action<CreateChat, Result<(), ServerFnError>>;
pub type ChatDeleteAction = Action<DeleteChat, Result<(), ServerFnError>>;
pub type ChatUpdateTitleAction = Action<UpdateChatTitle, Result<(), ServerFnError>>;
pub type OnGoalSubmit = Action<String, Result<(), ServerFnError>>;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UiMessage {
  pub user_message: String,
  pub miko_message: Option<String>,
  pub details: IndexMap<String, Vec<String>>,
}

#[derive(Clone, Copy)]
pub struct ShowFileModal(pub RwSignal<bool>, pub WriteSignal<Option<UploadedFile>>);

#[derive(Clone, Copy)]
pub struct ShowChatDetailsModal(
  pub RwSignal<bool>,
  pub RwSignal<UiMessage>,
  pub RwSignal<String>,
);

pub fn create_chat_details_state() -> ShowChatDetailsModal {
  let res = ShowChatDetailsModal(
    create_rw_signal(false),
    create_rw_signal(UiMessage::default()),
    create_rw_signal(String::new()),
  );
  provide_context(res);
  res
}

#[derive(Clone)]
pub struct ChatState {
  pub active_chat: RwSignal<Option<Uuid>>,
  pub edit_chat: RwSignal<Option<EditChat>>,
}

pub fn create_chat_state() {
  let res = ChatState {
    active_chat: create_rw_signal(None),
    edit_chat: create_rw_signal(None),
  };
  provide_context(res);
}

#[derive(Clone)]
pub struct ChatResourceContext {
  pub resource: ChatResource,
  pub create_chat: ChatCreateAction,
  pub delete_chat: ChatDeleteAction,
  pub update_title: ChatUpdateTitleAction,
  pub submit_goal: OnGoalSubmit,
}

pub fn create_chat_resource() -> ChatResource {
  let create_chat: ChatCreateAction = create_server_action::<CreateChat>();
  let delete_chat = create_server_action::<DeleteChat>();
  let update_chat_title = create_server_action::<UpdateChatTitle>();
  let on_goal_submit = create_action(|_input| async move { Ok::<_, ServerFnError>(()) });
  let res = create_resource(
    move || {
      (
        create_chat.version().get(),
        delete_chat.version().get(),
        update_chat_title.version().get(),
        on_goal_submit.version().get(),
      )
    },
    move |_| get_chats(),
  );
  provide_context(ChatResourceContext {
    resource: res,
    create_chat,
    delete_chat,
    update_title: update_chat_title,
    submit_goal: on_goal_submit,
  });
  res
}

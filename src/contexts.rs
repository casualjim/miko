use leptos::*;
use uuid::Uuid;
use web_sys::File;

use crate::{
  models::{Chat, EditChat},
  routes::chats::{get_chats, CreateChat, DeleteChat, UpdateChatTitle},
};

pub type ChatResource = Resource<(usize, usize, usize), Result<Vec<Chat>, ServerFnError>>;
pub type ChatCreateAction = Action<CreateChat, Result<(), ServerFnError>>;
pub type ChatDeleteAction = Action<DeleteChat, Result<(), ServerFnError>>;
pub type ChatUpdateTitleAction = Action<UpdateChatTitle, Result<(), ServerFnError>>;

#[derive(Clone, Copy)]
pub struct ShowFileModal(pub RwSignal<bool>, pub WriteSignal<Option<File>>);

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
pub struct ChatResourceContext(
  pub ChatResource,
  pub ChatCreateAction,
  pub ChatDeleteAction,
  pub ChatUpdateTitleAction,
);

pub fn create_chat_resource() -> ChatResource {
  let create_chat: ChatCreateAction = create_server_action::<CreateChat>();
  let delete_chat = create_server_action::<DeleteChat>();
  let update_chat_title = create_server_action::<UpdateChatTitle>();
  let res = create_resource(
    move || {
      (
        create_chat.version().get(),
        delete_chat.version().get(),
        update_chat_title.version().get(),
      )
    },
    move |_| get_chats(),
  );
  provide_context(ChatResourceContext(
    res,
    create_chat,
    delete_chat,
    update_chat_title,
  ));
  res
}

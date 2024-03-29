use gloo_net::eventsource::futures::EventSource;
use leptos::{
  html::{Div, Label},
  logging::log,
  *,
};
use leptos_use::{use_drop_zone, UseDropZoneReturn};
use phosphor_leptos::{File as PhosphorFile, *};
use uuid::Uuid;
use web_sys::{js_sys, Event, File, HtmlInputElement};

use crate::{api, models::UploadedFile, routes::files::get_files, ChatResource, ShowFileModal};

#[component]
#[allow(unused_variables)]
pub fn Workspace(chat_id: ReadSignal<Option<Uuid>>, chats: ChatResource) -> impl IntoView {
  let ShowFileModal(show_file_modal, set_selected_file) = expect_context();
  let (files_to_upload, set_files_to_upload) = create_signal::<Vec<File>>(vec![]);
  let (files, set_files) = create_signal::<Vec<UploadedFile>>(vec![]);
  let (active_chat, set_active_chat) = create_signal(None);

  create_effect(move |_| {
    if let Some(chat_id) = chat_id() {
      if let Some(active_id) = active_chat() {
        if active_id != chat_id {
          set_files.update(|v| v.clear());
          set_active_chat.update(|v| *v = Some(chat_id));
        }
      } else {
        set_active_chat.update(|v| *v = Some(chat_id));
      }
    } else {
      set_files.update(|v| v.clear());
      set_active_chat.update(|v| *v = None);
    }
  });

  let upload_action = create_action(move |(chat_id, files): &(Uuid, Vec<File>)| {
    let chat_id = *chat_id;
    let files = files.to_vec();
    async move {
      if files.is_empty() {
        return;
      }
      api::upload_file::<()>(chat_id, files).await;
      set_files_to_upload.update(|v| v.clear());
    }
  });

  let upload_changed = upload_action.value();
  let (is_watching, set_is_watching) = create_signal(false);
  create_effect(move |_| {
    if upload_changed().is_some() && !is_watching() {
      if let Some(chat_id) = chat_id() {
        let api_url = format!("/api/v1/workspace/{}/watch", chat_id);
        let mut source = EventSource::new(&api_url).unwrap();
        let mut sub = source.subscribe("message").unwrap();

        on_cleanup(move || {
          source.close();
          set_is_watching.update(|v| *v = false);
        });
        set_is_watching.update(|v| *v = true);

        use futures::StreamExt;
        spawn_local(async move {
          let files = get_files(chat_id).await.unwrap_or_default();
          set_files.update(|v| *v = files);

          while let Some(event) = sub.next().await {
            if let Ok((_, event)) = event {
              if let Some(data) = event.data().as_string() {
                let file: UploadedFile = serde_json::from_str(&data).unwrap();
                set_files.update(|v| {
                  if !v.iter().any(|f| f.file_name == file.file_name) {
                    v.push(file)
                  }
                });
              }
            }
          }
        });
      }
    }
  });

  create_effect(move |_| {
    let chat_id = chat_id();
    let files = files_to_upload();

    if let Some(chat_id) = chat_id {
      upload_action.dispatch((chat_id, files));
    }
  });

  let select_file = move |file: UploadedFile| {
    set_selected_file(Some(file));
    show_file_modal.set(true);
  };

  let open_file_picker = create_rw_signal(false);

  view! {
    <Show when=move || chat_id().is_some()>

      <div class="p-2">
        <div class="flex w-full items-center justify-between space-x-1 px-2 text-neutral-content p-1">
          <div class="text-xs uppercase tracking-widest text-[currentColor]">"Current workspace"</div>
          <div class="flex items-center space-x-1">
            <FileDialogOpener
              id="titlefiles"
              size="18"
              class="hover:cursor-pointer hover:text-accent"
              set_files=set_files_to_upload
              weight=IconWeight::Bold
              chat_id
              open_dialog=open_file_picker
            />
          </div>
        </div>

        <div class="relative h-full max-h-[24vh] overflow-y-auto [scrollbar-gutter:stable]">
          <Suspense fallback=move || {
              view! { <div class="skeleton h-24 w-full"></div> }
          }>
            <Show
              when=move || { !files().is_empty() }
              fallback=move || view! { <EmptyWorkspace set_files=set_files_to_upload chat_id/> }
            >
              <WorkspaceFiles files select_file set_files=set_files_to_upload/>
            </Show>
          </Suspense>
        </div>

      </div>
    </Show>
  }
}

#[component]
#[allow(unused_variables)]
fn EmptyWorkspace(
  chat_id: ReadSignal<Option<Uuid>>,
  set_files: WriteSignal<Vec<File>>,
) -> impl IntoView {
  let drop_zone_ref = create_node_ref::<Div>();
  let UseDropZoneReturn {
    is_over_drop_zone,
    files,
  } = use_drop_zone(drop_zone_ref);

  create_effect(move |_| {
    let files = files();
    if !files.is_empty() {
      set_files.update(|v| v.extend(files));
    }
  });

  let open_dialog = create_rw_signal(false);

  view! {
    <div
      node_ref=drop_zone_ref
      class="mt-1 flex cursor-pointer flex-col items-center justify-center space-y-2 rounded-lg border-2 border-dashed text-neutral-content border-neutral-content p-7 text-center transition-colors duration-300 hover:cursor-pointer hover:text-accent hover:border-accent"
      class:border-accent=is_over_drop_zone
      class:bg-base-300=is_over_drop_zone
      class:text-accent=is_over_drop_zone
      on:click={
          let open_dialog = open_dialog.clone();
          move |_| {
              open_dialog.set(true);
          }
      }
    >
      <FileDialogOpener
        id="emptyfiles"
        size="24"
        class="text-[currentColor] cursor-pointer"
        set_files
        chat_id
        open_dialog
      />
      <p class="leading-regular text-xs text-[currentColor]">
        "You currently have no files in your workspace. Drop or click here to add them."
      </p>
    </div>
  }
}

#[component]
#[allow(unused_variables)]
fn WorkspaceFiles(
  files: ReadSignal<Vec<UploadedFile>>,
  #[prop(into)] select_file: Callback<UploadedFile>,
  set_files: WriteSignal<Vec<File>>,
) -> impl IntoView {
  let drop_zone_ref = create_node_ref::<Div>();
  let UseDropZoneReturn {
    is_over_drop_zone,
    files: dropped_files,
  } = use_drop_zone(drop_zone_ref);

  create_effect(move |_| {
    let files = dropped_files();
    if !files.is_empty() {
      set_files.update(|v| v.extend(files));
    }
  });
  view! {
    <div
      node_ref=drop_zone_ref
      class="h-full space-y-1 rounded-lg border-2 border-solid border-neutral p-[6px] transition-all duration-100 ease-in-out"
      class:border-accent=is_over_drop_zone
      class:bg-base-950=is_over_drop_zone
      class:text-accent=is_over_drop_zone
    >
      <For each=files key=|f| f.file_name.clone() let:file>
        <div
          class="flex w-full cursor-pointer items-center space-x-2 rounded p-1 text-sm text-neutral-content transition-colors duration-300"
          on:click={
              let file = file.clone();
              move |_| {
                  let file = file.clone();
                  log!("selected file: {:?}", file);
                  select_file(file);
              }
          }
        >

          <FileIcon file_type=file.mime_type/>
          <div class="w-full overflox-x-hidden text-ellipsis whitespace-nowrap">{file.file_name}</div>
        </div>
      </For>
    </div>
  }
}

#[component]
pub(super) fn FileDialogOpener(
  id: &'static str,
  size: &'static str,
  class: &'static str,
  chat_id: ReadSignal<Option<Uuid>>,
  set_files: WriteSignal<Vec<File>>,
  open_dialog: RwSignal<bool>,
  #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
) -> impl IntoView {
  let label = create_node_ref::<Label>();

  create_effect(move |_| {
    let label = label.get().unwrap();
    if open_dialog() {
      label.click();
    }
  });

  let on_change = Callback::new(move |ev: Event| {
    let target = event_target::<HtmlInputElement>(&ev);
    let files = target
      .files()
      .map(|f| js_sys::Array::from(&f).to_vec())
      .unwrap_or_default();

    if files.is_empty() {
      return;
    }
    set_files.update(|v| v.extend(files.into_iter().map(File::from)));
    open_dialog.set(false);
  });

  view! {
    <div>
      <form
        action=move || {
            format!("/api/v1/workspace/{}", chat_id().as_ref().map(|id| id.to_string()).unwrap_or_default())
        }

        method="post"
        enctype="multipart/form-data"
      >
        <label node_ref=label for=id>
          <FilePlus size class weight/>
        </label>
        <input id=id name="file" type="file" on:change=on_change style="display:none;"/>
      </form>
    </div>
  }
}

#[component]
fn FileIcon(file_type: String) -> impl IntoView {
  // TODO: build a better icon system, one that generates svg icons for all the mime types

  let parts = file_type.split('/').collect::<Vec<_>>();
  match (parts[0], parts[1]) {
    (_, "pdf") => view! { <FilePdf size="18"/> },
    (_, "csv") => view! { <FileCsv size="18"/> },
    (_, "xls" | "xlsx") => view! { <FileXls size="18"/> },
    (_, "doc" | "docx") => view! { <FileDoc size="18"/> },
    (_, "jpg" | "jpeg") => view! { <FileJpg size="18"/> },
    (_, "png") => view! { <FilePng size="18"/> },
    (_, "zip") => view! { <FileZip size="18"/> },
    ("text", _) => view! { <FileText size="18"/> },
    ("image", _) => view! { <FileImage size="18"/> },
    ("audio", _) => view! { <FileAudio size="18"/> },
    ("video", _) => view! { <FileVideo size="18"/> },
    _ => view! { <PhosphorFile size="18"/> },
  }

  // let ext = name.split('.').last().unwrap_or_default();

  // let class = format!("mime-icon ico-{category} ico-{detail}");
  // view! {
  //   <div class=class>
  //     <div class="ico"></div>
  //   </div>
  // }
  // let ext_len = detail.len();
  // let file_size = file.size();

  // let font_size = move || Math::max(12f64 - 1f64 * ext_len as f64, 7f64);
  // let tracking_widest = move || ext_len <= 4;
  // let bottom = move || if tracking_widest() { 2f64 } else { 1f64 };

  // view! {
  //   <div class="relative h-5 w-5 max-w-[1.25rem]">
  //     <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
  //       <path
  //         d="M4.5 10.7671V4.01709C4.5 3.81818 4.57902 3.62741 4.71967 3.48676C4.86032 3.34611 5.05109 3.26709 5.25 3.26709H14.25M14.25 3.26709L19.5 8.51709M14.25 3.26709V8.51709H19.5M19.5 8.51709V10.7671"
  //         stroke="currentColor"
  //         strokeWidth="2"
  //         strokeLinecap="round"
  //         strokeLinejoin="round"
  //       />
  //     </svg>
  //     <div
  //       class="absolute left-1/2 -translate-x-1/2 transform text-center text-[currentColor] font-bold uppercase leading-none"
  //       class:tracking-widest=tracking_widest
  //       class:traking-wider=move || !tracking_widest()
  //       style:fontSize=font_size
  //       style:bottom=bottom
  //     >
  //     </div>
  //   </div>
  // }
}

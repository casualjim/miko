use leptos::{html::Div, logging::log, *};
use phosphor_leptos::{File as PhosphorFile, *};
use uuid::Uuid;
use web_sys::{js_sys, DragEvent, Event, File, FormData, HtmlFormElement, HtmlInputElement};

use crate::{api, ChatResource, ShowFileModal};

#[component]
pub fn Workspace(chat_id: ReadSignal<Option<Uuid>>, chats: ChatResource) -> impl IntoView {
  let ShowFileModal(show_file_modal, set_selected_file) = expect_context();

  let drop_zone_ref = create_node_ref::<Div>();
  let (files, set_files) = create_signal::<Vec<File>>(vec![]);

  let prevent_defaults = move |ev: DragEvent| {
    ev.prevent_default();
    ev.stop_propagation();
  };

  let on_drop = move |ev: DragEvent| {
    ev.prevent_default();
    ev.stop_propagation();

    if let Some(data_transfer) = ev.data_transfer() {
      let files = data_transfer
        .files()
        .map(|f| js_sys::Array::from(&f).to_vec())
        .unwrap_or_default()
        .into_iter()
        .map(File::from);
      set_files.update(|v| v.extend(files));
    }
  };

  let select_file = move |file: File| {
    set_selected_file(Some(file));
    show_file_modal.set(true);
  };

  view! {
    <div class="p-2">
      <div class="flex w-full items-center justify-between space-x-1 px-2 text-neutral-content p-1">
        <div class="text-xs uppercase tracking-widest text-[currentColor]">"Current workspace"</div>
        <div class="flex items-center space-x-1">
          <FileDialogOpener
            id="titlefiles"
            size="18"
            class="hover:cursor-pointer hover:text-primary"
            set_files
            weight=IconWeight::Bold
            chat_id
          />
        </div>
      </div>

      <div
        class="relative h-full max-h-[24vh] overflow-y-auto [scrollbar-gutter:stable]"
        on:dragenter=prevent_defaults
        on:dragover=prevent_defaults
        on:dragleave=prevent_defaults
        on:drop=on_drop
      >
        <Suspense fallback=move || {
            view! { <div class="skeleton h-24 w-full"></div> }
        }>
          <Show
            when=move || { !files().is_empty() }
            fallback=move || view! { <EmptyWorkspace node_ref=drop_zone_ref set_files chat_id/> }
          >
            <WorkspaceFiles node_ref=drop_zone_ref files select_file/>
          </Show>
        </Suspense>
      </div>

    </div>
  }
}

#[component]
#[allow(unused_variables)]
fn EmptyWorkspace(
  chat_id: ReadSignal<Option<Uuid>>,
  node_ref: NodeRef<Div>,
  set_files: WriteSignal<Vec<File>>,
) -> impl IntoView {
  view! {
    <div
      node_ref=node_ref
      class="mt-1 flex cursor-pointer flex-col items-center justify-center space-y-2 rounded-lg border-2 border-dashed text-neutral-content border-neutral-content p-7 text-center transition-colors duration-300 hover:border-primary hover:bg-base-950 hover:text-primary"
    >
      <FileDialogOpener
        id="emptyfiles"
        size="24"
        class="text-[currentColor] hover:cursor-pointer hover:text-primary"
        set_files
        chat_id
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
  node_ref: NodeRef<Div>,
  files: ReadSignal<Vec<File>>,
  #[prop(into)] select_file: Callback<File>,
) -> impl IntoView {
  view! {
    <div
      node_ref=node_ref
      class="h-full space-y-1 rounded-lg border-2 border-solid border-neutral p-[6px] transition-all duration-100 ease-in-out"
    >
      <For each=files key=|f| f.name() let:file>
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
          <FileIcon file_type=file.type_()/>
          <div class="w-full overflox-x-hidden text-ellipsis whitespace-nowrap">{file.name()}</div>
        </div>
      </For>
    </div>
  }
}

#[component]
fn FileDialogOpener(
  id: &'static str,
  size: &'static str,
  class: &'static str,
  chat_id: ReadSignal<Option<Uuid>>,
  set_files: WriteSignal<Vec<File>>,
  #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
) -> impl IntoView {
  let upload_action = create_action(move |form_data: &FormData| {
    let chat_id = chat_id();
    #[cfg(feature = "hydrate")]
    let form_data = form_data.clone();
    async move {
      #[allow(unused_variables)]
      if let Some(chat_id) = chat_id {
        #[cfg(feature = "hydrate")]
        api::upload_file::<()>(chat_id, form_data.clone()).await;
      }
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
    let form: HtmlFormElement = target.form().unwrap();

    let form_data = FormData::new_with_form(&form).unwrap();
    upload_action.dispatch(form_data);
    set_files.update(|v| v.extend(files.into_iter().map(File::from)));
  });

  view! {
    <div>
      <form action=move || {format!("/api/v1/workspace/{}", chat_id().as_ref().map(|id|id.to_string()).unwrap_or_default())} method="post" enctype="multipart/form-data" >
        <label for=id>
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

use leptos::*;
use phosphor_leptos::{FilePlus, IconWeight};

use crate::models::Chat;

type ChatsResource = Resource<(), Result<Vec<Chat>, ServerFnError>>;

#[component]
#[allow(unused_variables)]
pub fn Workspace(chats: ChatsResource) -> impl IntoView {
  view! {
    <div class="p-2">
      <div class="flex w-full items-center justify-between space-x-1 px-2">
        <div class="text-xs uppercase tracking-widest text-base-500">"Current workspace"</div>
        <div class="flex items-center space-x-1">
          <button class="btn btn-ghost">
            <FilePlus size="18" weight=IconWeight::Bold/>
          </button>
        </div>
      </div>
      <div class="relative h-full max-h-[24vh] overflow-y-auto [scrollbar-gutter:stable]">
        <Suspense fallback=move || {
            view! { <div class="skeleton h-24 w-full"></div> }
        }>
          <div class="mt-1 flex cursor-pointer flex-col items-center justify-center space-y-2 rounded-lg border-2 border-dashed border-base-500 p-7 text-center transition-colors duration-300 hover:border-cyan-500 hover:bg-base-950 hover:text-cyan-500">
            <FilePlus size="24" class="text-[currentColor]"/>
            <p class="leading-regular text-xs text-base-500">
              "You currently have no files in your workspace. Drop or click here to add them."
            </p>
          </div>
        </Suspense>
      </div>

    </div>
  }
}

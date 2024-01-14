use leptos::*;
use phosphor_leptos::{GearSix, IconWeight, SignOut, UserCirclePlus};

use crate::models::CurrentUser;

#[component]
pub fn AccountDropdown(show_logout: RwSignal<bool>) -> impl IntoView {
  let user = expect_context::<ReadSignal<CurrentUser>>();
  let is_authenticated = move || user().is_authenticated();
  view! {
    <Show when=is_authenticated fallback=move || view! { <GuestDropdown/> }>
      <UserDropdown user=user show_logout=show_logout/>
    </Show>
  }
}

#[component]
fn UserDropdown(user: ReadSignal<CurrentUser>, show_logout: RwSignal<bool>) -> impl IntoView {
  let picture = move || user().picture();
  let name = move || user().name();
  let email = move || user().email();
  view! {
    <div class="dropdown dropdown-top">
      <div
        tabindex="0"
        role="button"
        class="inline-flex w-full -translate-x-2 transform cursor-pointer items-center space-x-2 rounded-lg p-2 transition-colors hover:bg-base-800"
      >
        <div class="flex items-center  space-x-2">
          <Show
            when=move || { picture().is_some() }
            fallback=move || view! { <div className="h-8 w-8 min-w-[2rem] rounded-full bg-cyan-600"></div> }
          >
            <img src=picture() width="32" height="32" class="h-8 w-8 min-w-[2rem] rounded-full bg-cyan-600"/>
          </Show>
          <div class="w-full max-w-[124px] space-y-1 overflow-x-hidden">
            <div class="w-full overflow-hidden text-ellipsis whitespace-nowrap text-sm font-semibold leading-none">
              {name()}
            </div>
            <div class="w-full overflow-hidden text-ellipsis whitespace-nowrap text-[11px] leading-none text-gray-400">
              {email()}
            </div>
          </div>
        </div>
      </div>
      <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">
        <li>
          <GearSix size="16" weight=IconWeight::Bold/>
          <div class="leading-none">"Account Settings"</div>
        </li>
        <li on:click=move |_| { show_logout.set(true) }>
          <SignOut size="16" weight=IconWeight::Bold/>
          <div class="leading-none">"Sign Out"</div>
        </li>
      </ul>
    </div>
  }
}

#[component]
fn GuestDropdown() -> impl IntoView {
  view! {
    <div class="dropdown dropdown-top">
      <div
        tabindex="0"
        role="button"
        class="inline-flex w-full -translate-x-2 transform cursor-pointer items-center space-x-2 rounded-lg p-2 transition-colors hover:bg-base-800"
      >
        <Avatar size=32/>
        <div class="text-white">"Guest"</div>
      </div>
      <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">
        <li>
          <UserCirclePlus size="16" weight=IconWeight::Bold/>
          <a rel="external" href="/oauth/start">
            "Sign In"
          </a>
        </li>
      </ul>
    </div>
  }
}

#[component]
pub fn Avatar(size: usize, #[prop(optional)] class: &'static str) -> impl IntoView {
  view! {
    <img
      src="images/guest.svg"
      class=move || format!("rounded-full {}", class)
      width=size
      height=size
      alt="User Avatar"
      style:width=size
      style:height=size
    />
  }
}

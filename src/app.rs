pub mod handlers;
pub mod state;
use cfg_if::cfg_if;
use leptos::{logging::log, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{
  storage::{use_local_storage, use_local_storage_with_options, StringCodec, UseStorageOptions},
  use_media_query,
};
use uuid::Uuid;

pub const LEPTOS_OUTPUT_NAME: &str = env!("LEPTOS_OUTPUT_NAME");

// pub use state::*;
use crate::{
  components::{layout::*, modals::*},
  create_chat_details_state, create_chat_resource, create_chat_state,
  error_template::{AppError, ErrorTemplate},
  models::{CurrentUser, User},
  pages::*,
  routes::authn::*,
  ShowChatDetailsModal, ShowFileModal,
};

cfg_if! {
  if #[cfg(feature="ssr")] {
    use sqlx::PgPool;
    use state::AppState;
    use handlers::AuthSession;

    pub fn pool() -> Result<PgPool, ServerFnError> {
        use_context::<PgPool>()
            .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
    }

    pub fn auth() -> Result<AuthSession, ServerFnError> {
        use_context::<AuthSession>()
            .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))
    }

    pub fn app_state() -> Result<AppState, ServerFnError> {
        use_context::<AppState>()
            .ok_or_else(|| ServerFnError::ServerError("App state missing.".into()))
    }
  }
}

#[component]
pub fn App() -> impl IntoView {
  let logout = create_server_action::<Logout>();

  let user = create_resource(
    move || logout.version().get(),
    |_| async { get_user().await.ok().flatten() },
  );
  let (current_user, set_current_user) = create_signal(CurrentUser::default());
  provide_context(current_user);

  create_chat_state();
  create_chat_resource();

  let show_logout_modal = create_rw_signal(false);
  let show_file_modal = create_rw_signal(false);
  let (selected_file, set_selected_file) = create_signal(None);
  provide_context(ShowFileModal(show_file_modal, set_selected_file));

  let ShowChatDetailsModal(show_chat_details, chat_details_message, chat_status) =
    create_chat_details_state();

  // let getcurrentuser = create_memo(move |_| user.get().flatten());

  // create_effect(move |_| {
  //   let current_user = getcurrentuser();
  //   set_current_user.update(|v| *v = CurrentUser(current_user));
  // });

  let (is_dark, set_is_dark, _) = use_local_storage::<bool, StringCodec>("miko-dark-mode");

  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  let pick_theme = move |is_dark| {
    if is_dark {
      "night"
    } else {
      "pastel"
    }
  };

  let on_toggle_theme = move |_| {
    set_is_dark.update(|value| *value = !*value);
  };

  let (chat_id, set_chat_id) = create_signal(None);

  view! {
    <Html lang="en" class="h-full tracking-wide" attr:data-theme=move || pick_theme(is_dark()) />

    <Stylesheet id="leptos" href=format!("/pkg/{}.css", LEPTOS_OUTPUT_NAME)/>

    <Link rel="icon" href="/images/happy-egg.svg"/>
    <Link rel="mask-icon" href="/images/happy-egg.mono.svg"/>
    <Link rel="apple-touch-icon" href="/images/happy-egg.180px.png"/>
    <Link
      rel="stylesheet"
      href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css"
      integrity="sha512-hasIneQUHlh06VNBe7f6ZcHmeRTLIaQWFd43YriJ0UND19bvYRauxthDg8E4eVNPm9bRUhr5JGeqH7FRFXQu5g=="
      crossorigin="anonymous"
      referrerpolicy="no-referrer"
    />
    <Script
      src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"
      integrity="sha512-D9gUyxqja7hBtkWpPWGt9wfbfaMGVt9gnyCvYa+jojwwPHLCzUm5i8rpk7vD7wNee9bA35eYIjobYPaQuKS1MQ=="
      crossorigin="anonymous"
      referrerpolicy="no-referrer"
    />
    <Script
      src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/bash.min.js"
      integrity="sha512-i0JFeiLhgBAMGfIEVqMQwALhhse1orgl34XyotSUNiNbDIB1qS9IK53sDochCIrwvj4nJ51CsDSOqhGyhZITGg=="
      crossorigin="anonymous"
      referrerpolicy="no-referrer"
    />
    <Script
      src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/javascript.min.js"
      integrity="sha512-H69VMoQ814lKjFuFwLImb4OwoK8Rm8fcvsqZexaxjp/VkJfEnrt5TO7oaOdNlMf/j51QUctfLTe8+rgozW7l2A=="
      crossorigin="anonymous"
      referrerpolicy="no-referrer"
    />
    <Script
      src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/python.min.js"
      integrity="sha512-wW8K3TEH5ZViD4aMPzwPdhXKs/Kb5MAm7qLRd3QliYlHy0u9utSKZsZzqlZAgJ9xxXp81acwnrZVZ8oTfoLG1g=="
      crossorigin="anonymous"
      referrerpolicy="no-referrer"
    />
    <Script
      src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/go.min.js"
      integrity="sha512-wsnZc3vH14xwbbaoAwkar86729DTpz6wx48ABISfmaKLZwP/lm8d7Z+Hmr9JKobAENs0qO/cGounL7LUEg10Pg=="
      crossorigin="anonymous"
      referrerpolicy="no-referrer"
    />
    <Script
      src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/rust.min.js"
      integrity="sha512-vp/BmF+BW+m/wuIgSZQYqoB2Rwz46sD8YnVM51unvyvOcqYOwyP1BMAO4lHfmVpSZ+eeJAPnkk2fv7mdQN5HDw=="
      crossorigin="anonymous"
      referrerpolicy="no-referrer"
    />
    <Script defer="true">"hljs.highlightAll();" "console.log('highlighting code');"</Script>

    // sets the document title
    <Title text="Miko - the helpful robot"/>

    <Body class="h-full"/>

    <Router fallback=|| {
        let mut outside_errors = Errors::default();
        outside_errors.insert_with_default_key(AppError::NotFound);
        view! { <ErrorTemplate outside_errors/> }.into_view()
    }>
      <Show when=|| false>
        <div class="bg-accent text-accent-content border-accent"></div>
        <div class="bg-primary text-primary-content border-primary"></div>
        <div class="bg-secondary text-secondary-content border-secondary"></div>
      </Show>
      <Suspense><UserAuth user set_current_user/></Suspense>
      <Transition fallback=|| view! { <div class="skeleton w-full h-full"></div> }>
        <ErrorBoundary fallback=|errors| view! { <ErrorTemplate errors=errors/> }>
          <LogoutModal logout=logout show_modal=show_logout_modal/>
          <FileModal show_modal=show_file_modal content=selected_file/>
          <ChatDetailsModal show_modal=show_chat_details message=chat_details_message status=chat_status/>
          <SidebarLayoutWithHeader chat_id show_logout=show_logout_modal is_dark on_toggle_theme>
            <MainContent set_chat_id/>
          </SidebarLayoutWithHeader>
        </ErrorBoundary>
      </Transition>
    </Router>
  }
}

#[component]
fn UserAuth(
  user: Resource<usize, Option<User>>,
  set_current_user: WriteSignal<CurrentUser>,
) -> impl IntoView {
  create_effect(move |_| {
    if let Some(user) = user.get() {
      set_current_user.update(|v| *v = CurrentUser(user));
    }
  });
  view! {}
}

#[component]
fn MainContent(set_chat_id: WriteSignal<Option<Uuid>>) -> impl IntoView {
  let user = expect_context::<ReadSignal<CurrentUser>>();

  let is_authenticated = move || user().is_authenticated();

  view! {
    <Routes>
      <Route
        path="/"
        view=move || {
            view! {
              <Show
                when=is_authenticated
                fallback=move || {
                    view! { <AboutPage/> }
                }
              >

                <Outlet/>
              </Show>
            }
        }
      >

        <Route path="about" view=AboutPage/>
        <Route path="" view=move || view! { <ChatPage set_chat_id/> }/>
        <Route path="chat/:id" view=move || view! { <ChatPage set_chat_id/> }/>
      </Route>
    </Routes>
  }
}

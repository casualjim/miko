pub mod handlers;
pub mod state;
use cfg_if::cfg_if;
#[cfg(feature = "hydrate")] use gloo_storage::{LocalStorage, Storage as _};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use uuid::Uuid;

pub const LEPTOS_OUTPUT_NAME: &str = env!("LEPTOS_OUTPUT_NAME");

// pub use state::*;
use crate::{
  components::{layout::*, modals::*},
  create_chat_resource, create_chat_state,
  error_template::{AppError, ErrorTemplate},
  models::CurrentUser,
  pages::*,
  routes::authn::*,
  SelectedFileSetter,
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

  let user = create_resource(move || logout.version().get(), move |_| get_user());
  let (current_user, set_current_user) = create_signal(CurrentUser::default());
  provide_context(current_user);

  create_chat_state();
  create_chat_resource();

  let show_logout_modal = create_rw_signal(false);
  let show_file_modal = create_rw_signal(false);
  let (selected_file, set_selected_file) = create_signal(None);
  provide_context(SelectedFileSetter(set_selected_file));

  let getcurrentuser = create_memo(move |_| {
    user
      .get()
      .map(|user| user.ok().flatten())
      .unwrap_or_default()
  });

  create_effect(move |_| {
    let current_user = getcurrentuser();
    set_current_user(CurrentUser(current_user));
  });

  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  #[allow(unused_variables)]
  let (is_dark, set_is_dark) = create_signal(true);
  cfg_if! { if #[cfg(feature="hydrate")] {
    if let Ok(stored_dark_mode) = LocalStorage::get::<bool>("miko-dark-mode") {
      set_is_dark.update(|value| *value = stored_dark_mode);
    }
  }}

  let dark_mode = move || {
    if is_dark() {
      "night"
    } else {
      "pastel"
    }
  };
  let on_toggle_theme = move |_| {
    set_is_dark.update(|value| *value = !*value);
  };

  let (chat_id, set_chat_id) = create_signal(None);

  cfg_if! { if #[cfg(feature="hydrate")] {
    create_effect(move |_| {
      LocalStorage::set("miko-dark-mode", is_dark.get_untracked()).expect("failed to store dark mode");
    });
  }}

  view! {
    <Html lang="en" class="h-full tracking-wide" attr:data-theme=dark_mode/>

    <Stylesheet id="leptos" href=format!("/pkg/{}.css", LEPTOS_OUTPUT_NAME)/>

    <Link rel="icon" href="/images/happy-egg.svg"/>
    <Link rel="mask-icon" href="/images/happy-egg.mono.svg"/>
    <Link rel="apple-touch-icon" href="/images/happy-egg.180px.png"/>

    // sets the document title
    <Title text="Miko - the helpful robot"/>

    <Body class="h-full"/>

    // content for this welcome page
    <Router fallback=|| {
        let mut outside_errors = Errors::default();
        outside_errors.insert_with_default_key(AppError::NotFound);
        view! { <ErrorTemplate outside_errors/> }.into_view()
    }>
      <LogoutModal logout=logout show_modal=show_logout_modal/>
      <FileModal show_modal=show_file_modal content=selected_file/>
      <SidebarLayoutWithHeader chat_id show_logout=show_logout_modal is_dark on_toggle_theme>
        <Transition fallback=|| {
            view! { <div class="skeleton w-full h-full"></div> }
        }>
          <ErrorBoundary fallback=|errors| view! { <ErrorTemplate errors=errors/> }>
            <MainContent set_chat_id/>
          </ErrorBoundary>
        </Transition>
      </SidebarLayoutWithHeader>
    </Router>
  }
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

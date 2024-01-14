pub mod handlers;
pub mod state;
use cfg_if::cfg_if;
#[cfg(feature = "hydrate")] use gloo_storage::{LocalStorage, Storage as _};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// pub use state::*;
use crate::{
  components::layout::*,
  components::modals::*,
  error_template::{AppError, ErrorTemplate},
  models::CurrentUser,
  pages::*,
  routes::authn::*,
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

  let show_logout_modal = create_rw_signal(false);

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

  cfg_if! { if #[cfg(feature="hydrate")] {
    create_effect(move |_| {
      LocalStorage::set("miko-dark-mode", is_dark.get_untracked()).expect("failed to store dark mode");
    });
  }}

  view! {
    <Html lang="en" class="h-full font-sans font-light bg-base-900 text-white tracking-wide" attr:data-theme=dark_mode/>

    <Stylesheet id="miko" href="/pkg/miko.css"/>

    <Link rel="icon" href="/images/happy-egg.svg"/>
    <Link rel="mask-icon" href="/images/happy-egg.mono.svg"/>
    <Link rel="apple-touch-icon" href="/images/happy-egg.180px.png"/>

    // sets the document title
    <Title text="Miko - the helpful robot"/>

    <Body/>

    <div class="h-full">
      // content for this welcome page
      <Router fallback=|| {
          let mut outside_errors = Errors::default();
          outside_errors.insert_with_default_key(AppError::NotFound);
          view! { <ErrorTemplate outside_errors/> }.into_view()
      }>
        <LogoutModal logout=logout show_modal=show_logout_modal/>
        <SidebarLayoutWithHeader show_logout=show_logout_modal>
          <Transition fallback=|| {
              view! { <div class="skeleton w-full h-full"></div> }
          }>
            <ErrorBoundary fallback=|errors| view! { <ErrorTemplate errors=errors/> }>
              <MainContent/>
            </ErrorBoundary>
          </Transition>
        </SidebarLayoutWithHeader>
      </Router>
    </div>
  }
}

#[component]
fn MainContent() -> impl IntoView {
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

        <Route path="/about" view=AboutPage/>
        <Route path="/" view=HomePage/>
      </Route>

    </Routes>
  }
}

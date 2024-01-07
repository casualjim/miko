pub mod handlers;
pub mod state;
use cfg_if::cfg_if;
#[cfg(feature = "hydrate")] use gloo_events::EventListener;
#[cfg(feature = "hydrate")] use gloo_storage::{LocalStorage, Storage as _};
use leptos::{
  html::{Dialog, Div},
  *,
};
use leptos_meta::*;
use leptos_router::*;
#[cfg(feature = "hydrate")] use wasm_bindgen::JsCast as _;
#[cfg(feature = "hydrate")] use web_sys::Node;

// pub use state::*;
use crate::{
  app::handlers::*,
  components::{
    layout::*,
    logout::{LogoutModal, ModalAction},
  },
  error_template::{AppError, ErrorTemplate},
  models::CurrentUser,
  pages::*,
};

#[component]
pub fn App() -> impl IntoView {
  let logout = create_server_action::<Logout>();

  let user = create_resource(move || logout.version().get(), move |_| get_user());
  let (current_user, set_current_user) = create_signal(CurrentUser::default());
  provide_context(current_user);

  let (show_close_modal, set_show_close_modal) = create_signal(false);

  let getvalue = move || show_close_modal.get();
  let getcurrentuser = move || {
    user
      .get()
      .map(|user| user.ok().flatten())
      .unwrap_or_default()
  };
  let show_logout_modal = create_memo(move |_| getvalue());

  create_effect(move |_| {
    let current_user = getcurrentuser();
    set_current_user(CurrentUser(current_user));
  });

  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  let (is_dark, _set_is_dark) = create_signal(true);
  cfg_if! { if #[cfg(feature="hydrate")] {
    if let Ok(stored_dark_mode) = LocalStorage::get::<bool>("miko-dark-mode") {
      _set_is_dark.update(|value| *value = stored_dark_mode);
    }
  }}

  let dark_mode = move || {
    if is_dark() {
      "night"
    } else {
      "pastel"
    }
  };

  let modal_ref = create_node_ref::<Div>();

  cfg_if! { if #[cfg(feature="hydrate")] {
    create_effect(move |_| {
      LocalStorage::set("miko-dark-mode", is_dark()).expect("failed to store dark mode");
    });

    create_effect(move |_| {

        EventListener::new(&leptos_dom::document(), "keyup", move |event| {
          let keyboard_event = event.clone()
                                .dyn_into::<leptos::ev::KeyboardEvent>()
                                .unwrap();
          if show_logout_modal() && keyboard_event.key_code() == 27 {
            set_show_close_modal.set(false);
          }
        }).forget();

        EventListener::new(&leptos_dom::document(), "mouseup", move |event| {
          let mouse_event = event.clone().dyn_into::<leptos::ev::MouseEvent>().unwrap();
          if let Some(modal_ref) = modal_ref.get_untracked() {
            if let Some(target) = mouse_event.target() {
                let target = target.dyn_ref::<Node>();
                if !modal_ref.contains(target) {
                  set_show_close_modal.set(false);
                }
            }
          }
        }).forget();

    });
  }}

  view! {
      <Html lang="en" attr:data-theme=dark_mode/>

      <Stylesheet id="miko" href="/pkg/miko.css"/>

      <Link rel="icon" href="/images/happy-egg.svg"/>
      <Link rel="mask-icon" href="/images/happy-egg.mono.svg"/>
      <Link rel="apple-touch-icon" href="/images/happy-egg.180px.png"/>
      <Body class="h-full"/>

      // sets the document title
      <Title text="Miko - the helpful robot"/>

      // content for this welcome page
      <Router fallback=|| {
          let mut outside_errors = Errors::default();
          outside_errors.insert_with_default_key(AppError::NotFound);
          view! { <ErrorTemplate outside_errors/> }.into_view()
      }>
          <LogoutModal
              logout=logout
              show_modal_state=ModalAction(show_logout_modal, set_show_close_modal)
              modal_ref=modal_ref
          />
          <SidebarLayoutWithHeader show_modal=set_show_close_modal>
              <MainContent logout=logout/>
          </SidebarLayoutWithHeader>
      </Router>
  }
}

// #[component]
// pub fn AppRows() -> impl IntoView {
//   let logout = create_server_action::<Logout>();

//   let user = create_resource(move || logout.version().get(), move |_| get_user());

//   // Provides context that manages stylesheets, titles, meta tags, etc.
//   provide_meta_context();

//   let (is_dark, set_is_dark) = create_signal(true);
//   cfg_if! { if #[cfg(feature="hydrate")] {
//     if let Ok(stored_dark_mode) = LocalStorage::get::<bool>("miko-dark-mode") {
//       set_is_dark.update(|value| *value = stored_dark_mode);
//     }
//   }}

//   let dark_mode = move || {
//     if is_dark() {
//       "night"
//     } else {
//       "pastel"
//     }
//   };

//   cfg_if! { if #[cfg(feature="hydrate")] {
//     create_effect(move |_| {
//       LocalStorage::set("miko-dark-mode", is_dark()).expect("failed to store dark mode");
//     });
//   }}

//   view! {
//       <Html lang="en" attr:data-theme=dark_mode/>

//       <Stylesheet id="miko" href="/pkg/miko.css"/>

//       <Body class="flex flex-col h-screen w-screen overflow-hidden"/>

//       // sets the document title
//       <Title text="Miko - the helpful robot"/>

//       // content for this welcome page
//       <Router fallback=|| {
//           let mut outside_errors = Errors::default();
//           outside_errors.insert_with_default_key(AppError::NotFound);
//           view! { <ErrorTemplate outside_errors/> }.into_view()
//       }>
//           <Header
//               is_dark=is_dark
//               user=user
//               logout=logout
//               on_toggle_theme=move |_| set_is_dark.update(|value| *value = !*value)
//           />
//           <SidebarLayout sidebar=move || view! { <Sidebar/> }>
//               <Transition fallback=move || {
//                   view! { <div class="skeleton w-full h-full"></div> }
//               }>

//                   <MainContent logout=logout user=user/>

//               </Transition>
//           </SidebarLayout>
//           <Footer/>

//       </Router>
//   }
// }

#[component]
fn MainContent(logout: Action<Logout, Result<(), ServerFnError>>) -> impl IntoView {
  let user = expect_context::<ReadSignal<CurrentUser>>();
  let is_authenticated = move || matches!(user.get(), CurrentUser(Some(_)));
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

mod logout;

use leptos::*;
pub use logout::LogoutModal;

#[derive(Debug, Default, Clone)]
pub struct ModalsContext {
  pub show_logout_modal: RwSignal<bool>,
  pub show_signin_modal: RwSignal<bool>,
  pub show_settings_modal: RwSignal<bool>,
  pub show_welcome_modal: RwSignal<bool>,
}

use leptos::*;
use leptos_router::*;

use crate::{components::modals::Modal, routes::authn::Logout};

#[component]
pub fn LogoutModal(show_modal: RwSignal<bool>, logout: LogoutAction) -> impl IntoView {
  let close_modal = move |_| show_modal.set(false);
  view! {
    <Modal id="logoutModal" show_modal=show_modal>
      <p>"Are you sure you want to log out?"</p>
      <div class="modal-action">
        <ActionForm action=logout>
          <button type="submit" class="btn btn-primary" on:click=close_modal>
            "Log Out"
          </button>
        </ActionForm>
      </div>
    </Modal>
  }
}

pub type LogoutAction = Action<Logout, Result<(), ServerFnError>>;

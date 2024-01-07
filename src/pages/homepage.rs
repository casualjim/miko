use leptos::*;

use crate::components::{
  daisyui::{Card, Panel},
  layout::CenteredLayout,
};

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
  // Creates a reactive value to update the button
  let (count, set_count) = create_signal(0);
  let on_click = move |_| set_count.update(|count| *count += 1);

  view! {
      <CenteredLayout>
          <Panel>
              <Card>
                  <article>
                      <h1>"Welcome to Miko!"</h1>
                      <button on:click=on_click>"Click Me Preease: " {count}</button>
                  </article>
              </Card>
          </Panel>
      </CenteredLayout>
  }
}

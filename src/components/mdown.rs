use leptos::*;

#[component]
pub fn Markdown(
  #[prop(into)] content: MaybeSignal<String>,
  #[prop(optional, into)] class: MaybeSignal<&'static str>,
) -> impl IntoView {
  let html = move || markdown::to_html(&content());
  view! { <div prop:class=class prop:inner_html=html></div> }
}

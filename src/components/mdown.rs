use leptos::{logging::log, *};
use markdown::Options;
#[component]
pub fn Markdown(
  #[prop(into)] content: MaybeSignal<String>,
  #[prop(optional, into)] class: MaybeSignal<&'static str>,
) -> impl IntoView {
  let html = move || {
    let content = content();
    let mut opts = Options::default();
    opts.compile.allow_dangerous_html = true;

    markdown::to_html_with_options(&content, &opts).unwrap_or_default()
  };
  view! { <div class=class inner_html=html></div> }
}

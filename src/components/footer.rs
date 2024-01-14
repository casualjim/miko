use leptos::{html::p, *};

#[component]
pub fn Footer() -> impl IntoView {
  let footer_text = p().inner_html("&copy; 2024 - Made with ❤️ by <a class=\"link link-accent-content\" href=\"https://github.com/casualjim\" target=\"_blank\">@casualjim</a>");
  view! {
    <footer class="footer footer-center p-4 bg-base-300 text-base-content">
      <div>{footer_text}</div>
    </footer>
  }
}

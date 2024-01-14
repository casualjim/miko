use leptos::*;

#[component]
pub fn Card(children: Children) -> impl IntoView {
  view! {
    <div class="card bg-base-200 shadow-md w-full">
      <div class="card-body">{children()}</div>
    </div>
  }
}

#[component]
pub fn Panel(children: Children) -> impl IntoView {
  view! { <div class="flex flex-col items-center justify-start h-[90%]">{children()}</div> }
}

#[component]
pub fn Textarea(label: &'static str) -> impl IntoView {
  view! {
    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">{label}</span>
      </label>
      <input/>
    </div>
  }
}

#[component]
pub fn FormInput(
  label: &'static str,
  name: &'static str,
  input_type: &'static str,
  error_message: Option<&'static str>,
) -> impl IntoView {
  view! {
    <div class="form-control w-full max-w-xs">
      <label class="label">
        <span class="label-text">{label}</span>
      </label>
      <input name=name type=input_type class="input input-bordered"/>
      {error_message
          .map(|emsg| {
              Some(
                  view! {
                    <label class="label">
                      <span class="label-text-alt text-error">{emsg}</span>
                    </label>
                  },
              )
          })}

    </div>
  }
}

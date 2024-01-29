use std::rc::Rc;

use leptos::*;
use wasm_bindgen::JsValue;
use web_sys::{Blob, FilePropertyBag};

#[component]
pub fn ExamplePrompts<F: Fn(String) + 'static>(on_click: F) -> impl IntoView {
  let on_click = Rc::new(Box::new(on_click));
  let example_prompts = ExamplePrompt::examples();
  let prompts = move || {
    let pp = example_prompts();
    pp.clone().into_iter().map(Rc::new).enumerate()
  };
  let click_handler = {
    let on_click = on_click.clone();
    Rc::new(Box::new(move |prompt: Rc<ExamplePrompt>| {
      on_click(prompt.prompt.to_string());
    }))
  };
  view! {
    <div class="flex flex-col items-center space-y-3">
      <h2 class="w-full text-center font-normal">
        "Not sure where to start? " <span class="text-sm md:text-base">"Try asking one of these:"</span>
      </h2>
      {" "}
      <For
        each=prompts
        key=move |(idx, _)| *idx
        children=move |(_, prompt)| {
            view! {
              <div
                class="m-1 cursor-pointer rounded-lg  border-2 p-2.5 text-xstransition-all duration-300 ease-in-out"
                on:click={
                    let prompt = prompt.clone();
                    let click_handler = click_handler.clone();
                    move |_| click_handler(prompt.clone())
                }
              >

                {prompt.prompt}
              </div>
            }
        }
      />

    </div>
  }
}

const FILE1_CSV: &str = include_str!("file1.csv");
const FILE2_CSV: &str = include_str!("file2.csv");

fn str_to_file(name: &str, content: &str) -> Result<web_sys::File, JsValue> {
  let mut property_bag = FilePropertyBag::new();
  property_bag.type_("text/csv");

  let blob_parts = web_sys::js_sys::Array::new();
  blob_parts.push(&JsValue::from_str(content));

  let blob = Blob::new_with_blob_sequence(&blob_parts)?;

  let file_parts = web_sys::js_sys::Array::new();
  file_parts.push(&blob);
  web_sys::File::new_with_blob_sequence_and_options(&file_parts, name, &property_bag)
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ExamplePrompt {
  prompt: &'static str,
  files: Option<Vec<web_sys::File>>,
}

impl ExamplePrompt {
  pub fn new(prompt: &'static str, files: Option<Vec<web_sys::File>>) -> Self {
    Self { prompt, files }
  }

  pub fn examples() -> Signal<Vec<Self>> {
    Signal::derive(move || {
      vec![
        Self::new(
          "How much was spent on utilities within these CSVs? Write the result to a file.",
          Some(vec![
            str_to_file("file1.csv", FILE1_CSV).unwrap(),
            str_to_file("file2.csv", FILE2_CSV).unwrap(),
          ]),
        ),
        Self::new(
          "How many people were born in the last US election year? Write the results to a file.",
          None,
        ),
        Self::new(
          "Who are the two hosts of the Latent Space podcast? Write their names to a file.",
          None,
        ),
      ]
    })
  }
}

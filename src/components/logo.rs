use leptos::*;

#[component]
pub fn Logo<'a>(
  #[prop(optional)] class: Option<&'a str>,
  #[prop(optional)] logo_class: Option<&'a str>,
) -> impl IntoView {
  let clazz = class.map(|v| v.to_string()).unwrap_or_default();
  let logo_clazz = logo_class.map(|v| v.to_string()).unwrap_or_default();
  view! {
    <div class=move || { format!("flex {}", clazz) }>
      <svg
        height="50px"
        width="50px"
        version="1.1"
        id="Layer_1"
        xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink"
        viewBox="0 0 502.4 502.4"
        xml:space="preserve"
        fill="none"
        class=move || { format!("h-auto w-full {}", logo_clazz) }
      >

        <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
        <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
        <g id="SVGRepo_iconCarrier">
          <path
            style="fill:#E56505;"
            d="M91.2,290.4c0-140.8,76-290.4,160-290.4s160,148.8,160,290.4s-76,172.8-160,172.8 S91.2,431.2,91.2,290.4z"
          ></path>
          <path
            style="fill:#F97906;"
            d="M251.2,463.2c-83.2,0-160-32-160-172.8S168,0,251.2,0s160,148.8,160,290.4"
          ></path>
          <path style="fill:#FF9100;" d="M251.2,463.2c-83.2,0-160-32-160-172.8S168,0,251.2,0"></path>
          <ellipse style="fill:#EAD7A7;" cx="251.2" cy="490.4" rx="104" ry="12"></ellipse>
          <g>
            <polyline
              style="fill:#FFBC00;"
              points="91.2,315.2 148.8,315.2 184.8,350.4 187.2,350.4 187.2,350.4 235.2,349.6 235.2,349.6 188,344.8 155.2,310.4 155.2,310.4 "
            ></polyline>
            <polyline
              style="fill:#FFBC00;"
              points="411.2,315.2 353.6,315.2 317.6,350.4 315.2,350.4 315.2,350.4 267.2,349.6 267.2,349.6 314.4,344.8 347.2,310.4 347.2,310.4 "
            ></polyline>
            <path
              style="fill:#FFBC00;"
              d="M410.4,291.2L410.4,291.2c0.8-0.8,1.6-0.8,1.6-1.6c0-93.6-32-184.8-80-239.2v212l30.4,29.6h48V291.2 z"
            ></path>
          </g>
          <g>
            <path
              style="fill:#E56505;"
              d="M410.4,292L410.4,292c0.8-0.8,0.8-1.6,0.8-2.4c0-93.6-32-189.6-80-244v213.6"
            ></path>
            <path
              style="fill:#E56505;"
              d="M92.8,291.2L92.8,291.2c-0.8-0.8-1.6-0.8-1.6-1.6c0-93.6,32-177.6,80-232.8v204.8l-29.6,29.6H92.8z"
            ></path>
          </g>
          <path style="fill:#FFBC00;" d="M92,292L92,292c0-0.8-0.8-1.6-0.8-2.4c0-93.6,32-189.6,80-244v213.6"></path>
          <circle style="fill:#E56505;" cx="454.4" cy="157.6" r="40.8"></circle>
          <g>
            <path style="fill:#F97906;" d="M454.4,198.4c-22.4,0-40.8-18.4-40.8-40.8s18.4-40.8,40.8-40.8"></path>
            <circle style="fill:#F97906;" cx="454.4" cy="216" r="9.6"></circle>
          </g>
          <polyline
            style="fill:#FFBC00;"
            points="451.2,227.2 451.2,267.2 427.2,267.2 427.2,299.2 411.2,299.2 "
          ></polyline>
          <ellipse
            transform="matrix(0.0789 0.9969 -0.9969 0.0789 627.9552 -172.9957)"
            style="fill:#FF9100;"
            cx="407.597"
            cy="253.328"
            rx="40.001"
            ry="13.6"
          ></ellipse>
          <ellipse
            transform="matrix(0.0788 0.9969 -0.9969 0.0788 631.4185 -177.6087)"
            style="fill:#FFBC00;"
            cx="411.81"
            cy="252.845"
            rx="24"
            ry="5.6"
          ></ellipse>
          <g>
            <circle style="fill:#FCE6F1;" cx="377.6" cy="252" r="3.2"></circle>
            <circle style="fill:#FCE6F1;" cx="364.8" cy="256" r="3.2"></circle>
            <circle style="fill:#FCE6F1;" cx="352" cy="259.2" r="3.2"></circle>
          </g>
          <path
            style="fill:#FFBC00;"
            d="M475.2,143.2c0,2.4-1.6,4-4,4h-32c-2.4,0-4-1.6-4-4l0,0c0-2.4,1.6-4,4-4h32 C473.6,139.2,475.2,140.8,475.2,143.2L475.2,143.2z"
          ></path>
          <circle style="fill:#F97906;" cx="48" cy="157.6" r="40.8"></circle>
          <path style="fill:#E56505;" d="M48,198.4c22.4,0,40.8-18.4,40.8-40.8S70.4,116.8,48,116.8"></path>
          <circle style="fill:#F97906;" cx="48" cy="216" r="9.6"></circle>
          <polyline style="fill:#FFBC00;" points="51.2,227.2 51.2,267.2 75.2,267.2 75.2,299.2 91.2,299.2 "></polyline>
          <ellipse
            transform="matrix(0.0789 -0.9969 0.9969 0.0789 -165.1524 328.4951)"
            style="fill:#F97906;"
            cx="95.193"
            cy="253.622"
            rx="40.001"
            ry="13.6"
          ></ellipse>
          <ellipse
            transform="matrix(0.0788 -0.9969 0.9969 0.0788 -169.5547 323.7074)"
            style="fill:#FFBC00;"
            cx="90.375"
            cy="253.597"
            rx="24"
            ry="5.6"
          ></ellipse>
          <g>
            <circle style="fill:#E5337C;" cx="124.8" cy="252" r="3.2"></circle>
            <circle style="fill:#E5337C;" cx="137.6" cy="256" r="3.2"></circle>
            <circle style="fill:#E5337C;" cx="150.4" cy="259.2" r="3.2"></circle>
          </g>
          <g>
            <path
              style="fill:#FFBC00;"
              d="M67.2,143.2c0-2.4-1.6-4-4-4h-32c-2.4,0-4,1.6-4,4l0,0c0,2.4,1.6,4,4,4h32 C65.6,147.2,67.2,145.6,67.2,143.2L67.2,143.2z"
            ></path>
            <path
              style="fill:#FFBC00;"
              d="M212.8,125.6c0,14.4-11.2,35.2-25.6,35.2c-13.6,0-25.6-20.8-25.6-35.2c0-13.6,11.2-25.6,25.6-25.6 C201.6,100,212.8,111.2,212.8,125.6z"
            ></path>
          </g>
          <circle style="fill:#E5337C;" cx="188" cy="125.6" r="25.6"></circle>
          <path style="fill:#9B0B4D;" d="M169.6,107.2c9.6-9.6,25.6-9.6,36,0s9.6,25.6,0,36"></path>
          <circle style="fill:#00FFF2;" cx="188" cy="125.6" r="14.4"></circle>
          <path style="fill:#16D8D8;" d="M188,140c-8,0-14.4-6.4-14.4-14.4s6.4-14.4,14.4-14.4"></path>
          <g>
            <circle style="fill:#FCE6F1;" cx="176.8" cy="114.4" r="8.8"></circle>
            <circle style="fill:#FCE6F1;" cx="202.4" cy="138.4" r="4"></circle>
          </g>
          <path
            style="fill:#FFBC00;"
            d="M340,125.6c0,14.4-11.2,35.2-25.6,35.2c-13.6,0-25.6-20.8-25.6-35.2c0-13.6,11.2-25.6,25.6-25.6 C328.8,100,340,111.2,340,125.6z"
          ></path>
          <circle style="fill:#E5337C;" cx="314.4" cy="125.6" r="25.6"></circle>
          <path style="fill:#9B0B4D;" d="M296.8,107.2c9.6-9.6,25.6-9.6,36,0c9.6,9.6,9.6,25.6,0,36"></path>
          <circle style="fill:#00FFF2;" cx="314.4" cy="125.6" r="14.4"></circle>
          <path style="fill:#16D8D8;" d="M314.4,140c-8,0-14.4-6.4-14.4-14.4s6.4-14.4,14.4-14.4"></path>
          <g>
            <circle style="fill:#FCE6F1;" cx="303.2" cy="114.4" r="8.8"></circle>
            <circle style="fill:#FCE6F1;" cx="328.8" cy="138.4" r="4"></circle>
          </g>
          <g>
            <polygon style="fill:#9B0B4D;" points="267.2,160.8 251.2,176.8 235.2,160.8 251.2,160 "></polygon>
            <circle style="fill:#9B0B4D;" cx="251.2" cy="14.4" r="3.2"></circle>
            <circle style="fill:#9B0B4D;" cx="251.2" cy="28.8" r="3.2"></circle>
            <circle style="fill:#9B0B4D;" cx="251.2" cy="44" r="3.2"></circle>
            <circle style="fill:#9B0B4D;" cx="251.2" cy="58.4" r="3.2"></circle>
            <circle style="fill:#9B0B4D;" cx="251.2" cy="72.8" r="3.2"></circle>
          </g>
        </g>
      </svg>
    </div>
  }
}

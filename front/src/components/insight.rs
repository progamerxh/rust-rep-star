use dioxus::prelude::*;

#[component]
pub fn InsightCard(content: String) -> Element {
    rsx! {
        div { class: "border p-4 rounded shadow-md mb-4",
            p { class: "text-gray-700", "{content}" }
        }
    }
}

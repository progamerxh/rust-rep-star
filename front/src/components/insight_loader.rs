use dioxus::prelude::*;

#[component]
pub fn InsightLoader() -> Element {
    rsx! {
        div { class: "border p-4 rounded shadow-md mb-4",
            div { class: "animate-pulse flex space-x-4",
                div { class: "flex-1 space-y-6 py-1",
                    div { class: "h-2 bg-gray-300 rounded" }
                    div { class: "space-y-3",
                        div { class: "grid grid-cols-3 gap-4",
                            div { class: "h-2 bg-gray-300 rounded col-span-2" }
                            div { class: "h-2 bg-gray-300 rounded col-span-1" }
                        }
                        div { class: "h-2 bg-gray-300 rounded" }
                    }
                }
            }
        }
    }
}

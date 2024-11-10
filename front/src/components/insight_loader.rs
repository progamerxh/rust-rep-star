use dioxus::prelude::*;

#[component]
pub fn InsightLoader() -> Element {
    rsx! {
        div { class: "p-4 shadow-md mb-4 [background:linear-gradient(45deg,#bdbdbd,theme(colors.gray.200)_10%,#bdbdbd)_padding-box,conic-gradient(from_var(--border-angle),theme(colors.gray.200/.48)_80%,_theme(colors.gray.500)_86%,_theme(colors.gray.300)_90%,_theme(colors.gray.500)_94%,_theme(colors.gray.600/.48))_border-box] rounded-2xl border border-transparent animate-border",
            div { class: "animate-pulse flex space-x-4",
                div { class: "flex-1 space-y-6 py-1",
                    div { class: "h-2 bg-gray-500 rounded" }
                    div { class: "space-y-3",
                        div { class: "grid grid-cols-3 gap-4",
                            div { class: "h-2 bg-gray-500 rounded col-span-2" }
                            div { class: "h-2 bg-gray-500 rounded col-span-1" }
                        }
                        div { class: "h-2 bg-gray-500 rounded" }
                    }
                }
            }
        }
    }
}

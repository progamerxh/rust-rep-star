use dioxus::prelude::*;

#[component]
pub fn InsightCard(content: String) -> Element {
    rsx! {
        div { class: "p-4 shadow-md mb-4 [background:linear-gradient(45deg,#bdbdbd,theme(colors.gray.200)_10%,#bdbdbd)_padding-box,conic-gradient(from_var(--border-angle),theme(colors.gray.200/.48)_80%,_theme(colors.gray.500)_86%,_theme(colors.gray.300)_90%,_theme(colors.gray.500)_94%,_theme(colors.gray.600/.48))_border-box] rounded-2xl border border-transparent animate-border",
            p { class: "text-gray-700", "{content}" }
        }
    }
}

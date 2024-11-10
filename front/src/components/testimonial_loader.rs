use dioxus::prelude::*;
use rand::random;

#[component]
pub fn TestimonialLoader() -> Element {
    rsx! {
        div { class: "grid grid-cols-2 md:grid-cols-3 gap-4",
            for _ in 0..3 {
                div { class: "grid gap-4",
                    for _ in 0..5 {
                        // random height
                        div { class: "border p-4 rounded shadow-md mb-4 w-[280px]",
                            div { class: "animate-pulse flex space-x-4",
                                div { class: "flex-1 space-y-6 py-1",
                                    div { class: "h-2 bg-gray-300 rounded" }
                                    div { class: "space-y-3",
                                        div { class: "grid grid-cols-3 gap-4",
                                            div { class: "h-2 bg-gray-300 rounded col-span-2" }
                                            div { class: "h-2 bg-gray-300 rounded col-span-1" }
                                        }
                                        for _ in 0..(random::<u8>() % 4) {
                                            div { class: "h-2 bg-gray-300 rounded" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

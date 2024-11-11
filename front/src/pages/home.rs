use crate::{layouts::main::MainLayout, Route};
use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        MainLayout {
            div { class: "flex flex-col items-center justify-center h-screen",
                h1 { class: "text-4xl font-bold mb-4", "Welcome to the Rep Star!" }
                p { class: "text-lg mb-4", "Manage your testimonials with ease." }
                Link {
                    to: Route::ManageTestimonialPage {},
                    class: "animate-pulse bg-gray-800 text-white px-4 py-2 rounded m-2",
                    {},
                    "Manage Page"
                }
                Link {
                    to: Route::EmbedTestimonialPage {},
                    class: "bg-gray-200 text-gray-800 px-4 py-2 rounded m-2",
                    {},
                    "Embed Page"
                }
            }
        }
    }
}

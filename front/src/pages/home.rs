use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-screen",
            h1 {
                class: "text-4xl font-bold mb-4",
                "Welcome to the Testimonial Management App"
            }
            p {
                class: "text-lg mb-4",
                "Manage your testimonials with ease."
            }
            Link {
                to: Route::TestimonialsPage  {},
                class: "bg-blue-500 text-white px-4 py-2 rounded m-2",
                "View Testimonials"
            }
            Link {
                to: Route::AddTestimonialPage {},
                class: "bg-green-500 text-white px-4 py-2 rounded m-2",
                "Add a Testimonial"
            }
        }
    }
}

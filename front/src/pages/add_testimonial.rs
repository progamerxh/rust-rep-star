use crate::components::add_testimonial::AddTestimonial;
use dioxus::prelude::*;

#[component]
pub fn AddTestimonialPage() -> Element {
    rsx! {
        div {
            class: "p-4",
            h1 {
                class: "text-2xl font-bold mb-4",
                "Add a Testimonial"
            }
            AddTestimonial {}
        }
    }
}

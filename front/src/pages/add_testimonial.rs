use crate::{components::add_testimonial::AddTestimonial, layouts::main::MainLayout};
use dioxus::prelude::*;

#[component]
pub fn AddTestimonialPage() -> Element {
    rsx! {
        MainLayout {
            h1 { class: "text-2xl font-bold mb-4", "Add a Testimonial" }
            AddTestimonial {}
        }
    }
}

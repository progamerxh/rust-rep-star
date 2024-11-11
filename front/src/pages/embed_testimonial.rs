use dioxus::prelude::*;
use shared::queries::TestimonialQueries;

use crate::{
    components::{
        add_testimonial::AddTestimonial, testimonial_list::TestimonialList,
        testimonial_loader::TestimonialLoader,
    },
    layouts::main::MainLayout,
    queries::testimonials::get_testimonials,
};

#[component]
pub fn EmbedTestimonialPage() -> Element {
    let mut testimonials = use_resource(move || get_testimonials(TestimonialQueries { q: None }));

    let TestimonialList = match &*testimonials.read() {
        Some(Ok(res)) => {
            rsx! {
                div {
                    TestimonialList { testimonials: res.clone(), cols: 1, tracking: true }
                }
            }
        }
        Some(Err(_)) => {
            rsx! { "An error occurred while fetching stories " }
        }
        None => {
            rsx! {
                TestimonialLoader {}
            }
        }
    };

    rsx! {
        MainLayout {
            div { class: "flex flex-col items-center justify-center space-y-4 mt-4",
                div { class: "w-full p-4 bg-gray-50 shadow-lg rounded-lg mt-4",
                    h1 { class: "text-2xl font-bold mb-2", "Add a Testimonial" }
                    AddTestimonial {
                        on_submit: move |_| {
                            testimonials.restart();
                        }
                    }
                }
                div { class: "w-full flex flex-col items-center justify-center space-y-4 mt-4",
                    div { class: "p-4 bg-gray-50 shadow-lg rounded-lg", {TestimonialList } }
                }
            }
        }
    }
}

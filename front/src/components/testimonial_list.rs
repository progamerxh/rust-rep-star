use crate::components::testimonial::Testimonial;
use dioxus::prelude::*;

#[component]
pub fn TestimonialList(testimonials: Vec<shared::models::Testimonial>) -> Element {
    rsx! {
        div { class: "testimonial-list",
            for t in testimonials {
                Testimonial { content: t.content.clone(), rating: t.rating, date: t.created_at }
            }
        }
    }
}

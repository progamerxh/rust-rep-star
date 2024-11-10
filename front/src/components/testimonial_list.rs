use crate::components::testimonial::Testimonial;
use dioxus::prelude::*;

#[component]
pub fn TestimonialList(testimonials: Vec<shared::models::Testimonial>) -> Element {
    rsx! {
        div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
            for t in testimonials {
                Testimonial { content: t.content.clone(), rating: t.rating, date: t.created_at }
            }
        }
    }
}

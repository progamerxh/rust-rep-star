use crate::components::testimonial::Testimonial;
use dioxus::prelude::*;

#[component]
pub fn TestimonialList(testimonials: Vec<shared::models::Testimonial>) -> Element {
    // Chunk the testimonials into groups of 4
    // To display them in a Mansory layout
    let chunked_testimonials = testimonials.chunks(4).collect::<Vec<_>>();

    rsx! {
        div { class: "grid grid-cols-2 md:grid-cols-3 gap-4",
            for chunk in chunked_testimonials.iter() {
                div { class: "grid gap-4",
                    for t in chunk.iter() {
                        Testimonial { content: t.content.clone(), rating: t.rating, date: t.created_at }
                    }
                }
            }
        }
    }
}

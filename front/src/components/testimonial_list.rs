use crate::{
    components::testimonial::Testimonial, layouts::metric_tracking_wrapper::MetricTrackingWrapper,
};
use dioxus::prelude::*;

#[component]
pub fn TestimonialList(
    testimonials: Vec<shared::models::Testimonial>,
    cols: usize,
    tracking: Option<bool>,
) -> Element {
    // Chunk the testimonials into groups of 4
    // To display them in a Mansory layout
    let chunked_testimonials = testimonials.chunks(cols).collect::<Vec<_>>();

    rsx! {
        div { class: format!("grid grid-cols-{} gap-4", cols),
            for chunk in chunked_testimonials.iter() {
                div { class: "grid gap-4",
                    for t in chunk.iter() {
                        if tracking.unwrap_or(false) {
                            MetricTrackingWrapper { index: t.created_at.timestamp_millis(),
                                Testimonial { content: t.content.clone(), rating: t.rating, date: t.created_at }
                            }
                        } else {
                            Testimonial { content: t.content.clone(), rating: t.rating, date: t.created_at }
                        }
                    }
                }
            }
        }
    }
}

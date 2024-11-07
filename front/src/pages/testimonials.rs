use crate::components::testimonial_list::TestimonialList;
use dioxus::prelude::*;
use shared::models::Testimonial;

#[component]
pub fn TestimonialsPage() -> Element {
    let testimonials: Vec<Testimonial> = vec![Testimonial {
        content: "I had a great experience!".to_string(),
        rating: 5.0,
        created_at: "2024-11-07T09:50:19.585Z"
            .parse()
            .expect("failed to parse date"),
        updated_at: "2024-11-07T09:50:19.585Z"
            .parse()
            .expect("failed to parse date"),
        user_id: Some(uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()),
        id: uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
    }];

    rsx! {
        div {
            class: "p-4",
            h1 {
                class: "text-3xl font-bold",
                "Testimonials"
            }
            TestimonialList { testimonials: testimonials}
        }
    }
}

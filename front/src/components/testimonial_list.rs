#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::testimonial::Testimonial;

#[component]
pub fn TestimonialList(testimonials: Vec<shared::models::Testimonial>) -> Element {
    let testimonials_rendered = testimonials.iter().map(|t| {
        rsx! {
            Testimonial {
               content: t.content.clone(),
               rating: t.rating,
               date: t.created_at
        }}
    });

    rsx! {
        div {
            class: "testimonial-list",
             {testimonials_rendered}
        }
    }
}

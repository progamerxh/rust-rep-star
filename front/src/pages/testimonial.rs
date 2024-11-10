use crate::components::insight::InsightCard;
use crate::components::testimonial_list::TestimonialList;
use crate::queries::insights::get_insights;
use crate::queries::testimonials::get_testimonials;
use dioxus::prelude::*;

#[component]
pub fn TestimonialsPage() -> Element {
    let testimonials = use_resource(move || get_testimonials());
    let insights = use_resource(move || get_insights());

    let InsightCards = match &*insights.read_unchecked() {
        Some(Ok(res)) => {
            rsx! {
                div { class: "p-4",
                    h1 { class: "text-3xl font-bold", "Insights" }
                    for insight in res {
                        InsightCard { content: insight.message.clone() }
                    }
                }
            }
        }
        Some(Err(_)) => {
            rsx! { "An error occurred while fetching insights " }
        }
        None => {
            rsx! { "Loading items" }
        }
    };

    // check if the future is resolved
    match &*testimonials.read_unchecked() {
        Some(Ok(res)) => {
            // if it is, render the stories
            rsx! {
                div { class: "p-4",
                    h1 { class: "text-3xl font-bold", "Testimonials" }
                    {InsightCards},
                    TestimonialList { testimonials: res.clone() }
                }
            }
        }
        Some(Err(_)) => {
            // if there was an error, render the error
            rsx! { "An error occurred while fetching stories " }
        }
        None => {
            // if the future is not resolved yet, render a loading message
            rsx! { "Loading items" }
        }
    }
}

use crate::components::insight::InsightCard;
use crate::components::testimonial_list::TestimonialList;
use crate::components::time_duration_buttons::TimeDurationButtons;
use crate::queries::insights::get_insights;
use crate::queries::testimonials::get_testimonials;
use dioxus::prelude::*;

#[component]
pub fn TestimonialsPage() -> Element {
    let mut selected_duration = use_signal(|| "day".to_owned());
    let testimonials = use_resource(move || get_testimonials());
    let mut insights = use_resource(move || get_insights(selected_duration.read().to_string()));

    let Insight = match &*insights.read() {
        Some(Ok(res)) => {
            rsx! {
                div { class: "p-4",
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
            rsx! {
                div { class: "p-4", "Insights are loading" }
            }
        }
    };

    match &*testimonials.read_unchecked() {
        Some(Ok(res)) => {
            rsx! {
                div { class: "flex flex-col items-center justify-center space-y-4",
                    div { class: "p-4 bg-white shadow-lg rounded-lg w-full",
                        p { class: "text-xl font-bold mb-2", "What our customers say" }
                        TimeDurationButtons {
                            on_select: move |value| {
                                selected_duration.set(value);
                                insights.clear();
                            }
                        }
                        div { class: "p-4",
                            {
                                Insight
                            }
                        }
                    }
                    div { class: "p-4 bg-white shadow-lg rounded-lg",
                        TestimonialList { testimonials: res.clone() }
                    }
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

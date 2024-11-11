use std::time::Duration;

use crate::components::insight::InsightCard;
use crate::components::insight_loader::InsightLoader;
use crate::components::testimonial_list::TestimonialList;
use crate::components::testimonial_loader::TestimonialLoader;
use crate::components::time_duration_buttons::TimeDurationButtons;
use crate::layouts::main::MainLayout;
use crate::queries::insights::get_insights;
use crate::queries::testimonials::get_testimonials;
use crate::utils::use_debounce;
use dioxus::prelude::*;
use shared::queries::TestimonialQueries;

#[component]
pub fn ManageTestimonialPage() -> Element {
    let mut query_string = use_signal(|| "".to_owned());

    let mut selected_duration = use_signal(|| "day".to_owned());
    let mut testimonials = use_resource(move || {
        get_testimonials(TestimonialQueries {
            q: Some(query_string.read().clone()),
        })
    });
    let mut insights = use_resource(move || get_insights(selected_duration.read().to_string()));

    let mut debounce_query = use_debounce(Duration::from_millis(500), move |text| {
        query_string.set(text);
        testimonials.clear();
    });

    let InsightBox = match &*insights.read() {
        Some(Ok(res)) => {
            rsx! {
                div { class: "p-4 ",
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
                div { class: "p-4", InsightLoader {} }
            }
        }
    };

    let TestimonialList = match &*testimonials.read() {
        Some(Ok(res)) => {
            rsx! {
                div {
                    TestimonialList { testimonials: res.clone(), cols: 3 }
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
                div { class: "p-4 bg-gray-50 shadow-lg rounded-lg w-full",
                    p { class: "text-xl font-bold mb-2", "What our customers say" }
                    TimeDurationButtons {
                        on_select: move |value| {
                            selected_duration.set(value);
                            insights.clear();
                        }
                    }
                    div { class: "p-4", {InsightBox} }
                }
                div { class: "p-4 bg-gray-50 shadow-lg rounded-lg w-full",
                    p { class: "text-xl font-bold mb-2", "Search Testimonials" }
                    input {
                        class: "w-full p-2 border rounded",
                        r#type: "text",
                        placeholder: "Search...",
                        oninput: move |event| {
                            let value = event.value().clone();
                            debounce_query.action(value);
                        }
                    }
                }
                div { class: "p-4 bg-gray-50 shadow-lg rounded-lg", {TestimonialList } }
            }
        }
    }
}

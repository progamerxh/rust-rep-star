use crate::components::insight::InsightCard;
use crate::components::insight_loader::InsightLoader;
use crate::components::testimonial_list::TestimonialList;
use crate::components::testimonial_loader::TestimonialLoader;
use crate::components::time_duration_buttons::TimeDurationButtons;
use crate::layouts::main::MainLayout;
use crate::queries::insights::get_insights;
use crate::queries::testimonials::get_testimonials;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use shared::queries::TestimonialQueries;

#[component]
pub fn ManageTestimonialPage() -> Element {
    let mut query_input = use_signal(|| "".to_string());
    let mut q = use_signal(|| "".to_string());

    let mut selected_duration = use_signal(|| "day".to_string());
    let mut testimonials = use_resource(move || {
        get_testimonials(TestimonialQueries {
            q: Some(q.read().clone()),
        })
    });
    let mut insights = use_resource(move || get_insights(selected_duration.read().to_string()));

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
                    p { class: "text-lg font-bold mb-2", "What our customers say in the last" }
                    TimeDurationButtons {
                        on_select: move |value| {
                            selected_duration.set(value);
                            insights.clear();
                        }
                    }
                    div { class: "p-4", {InsightBox} }
                }
                form {
                    class: "p-4 bg-gray-50 shadow-lg rounded-lg w-full flex gap-4",
                    onsubmit: move |_| {
                        q.set(query_input.read().clone());
                        testimonials.clear();
                    },
                    button { class: "p-2 bg-gray-500  text-white rounded ml-2", "Search" }
                    input {
                        class: "w-full p-2 border rounded",
                        r#type: "text",
                        value: "{query_input}",
                        oninput: move |e| query_input.set(e.value()),
                        placeholder: "Wanna search for a feedback?"
                    }
                }
                div { class: "p-4 bg-gray-50 shadow-lg rounded-lg", {TestimonialList } }
            }
        }
    }
}

use chrono::{DateTime, Utc};
use dioxus::prelude::*;

use crate::layouts::metric_tracking_wrapper::MetricTrackingWrapper;

#[component]
pub fn Testimonial(content: String, rating: f64, date: DateTime<Utc>) -> Element {
    rsx! {
        MetricTrackingWrapper { index: date.timestamp_millis(),
            div { class: "border h-auto max-w-full rounded-lg p-4 shadow-md",
                div { class: "flex",
                    if rating > 0.0 {
                        // Round the rating to the nearest half star
                        {(0..(rating / 2.0).round() as usize).map(|_| rsx! {
                            span { class: "text-yellow-500", "★" }
                        })}
                    } else {
                        {(0..5).map(|_| rsx! {
                            span { class: "text-gray-500", "★" }
                        })}
                    }
                }
                p { class: "text-md", "{content}" }
                span { class: "text-gray-500 text-sm", "{date}" }
            }
        }
    }
}

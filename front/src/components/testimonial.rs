use chrono::{DateTime, Utc};
use dioxus::prelude::*;

use crate::layouts::metric_tracking_wrapper::MetricTrackingWrapper;

#[component]
pub fn Testimonial(content: String, rating: f64, date: DateTime<Utc>) -> Element {
    rsx! {
        MetricTrackingWrapper { index: date.timestamp_millis(),
            div { class: "border p-4 rounded shadow-md mb-4",
                h3 { class: "text-md", "{content}" }
                div { class: "flex",
                    if rating > 0.0 {
                        {(0..rating.round() as usize).map(|_| rsx! {
                            span { class: "text-yellow-500", "★" }
                        })}
                    } else {
                        {(0..5).map(|_| rsx! {
                            span { class: "text-gray-500", "★" }
                        })}
                    }
                }
                span { class: "text-gray-500 text-sm", "{date}" }
            }
        }
    }
}

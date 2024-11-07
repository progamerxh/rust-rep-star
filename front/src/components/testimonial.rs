#![allow(non_snake_case)]

use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[component]
pub fn Testimonial(content: String, rating: f64, date: DateTime<Utc>) -> Element {
    rsx! {
        div {
            class: "border p-4 rounded shadow-md mb-4",
            h3 {
                class: "font-bold text-lg",
                "{content}"
            }
            p {
                class: "text-gray-700",
                "{rating}"
            }
            span {
                class: "text-gray-500 text-sm",
                "{date}"
            }
        }
    }
}

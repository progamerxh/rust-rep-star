use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use shared::models::CreateMetric;
use uuid::Uuid;
use web_sys::js_sys::Date;

use crate::queries::metrics::create_metric;

#[derive(PartialEq, Clone, Props)]
pub struct MetricTrackingWrapperProps {
    children: Element,
    index: i64,
}

pub fn MetricTrackingWrapper(props: MetricTrackingWrapperProps) -> Element {
    let mut hovered_at = use_signal::<Option<f64>>(|| None);

    let track_mouse_time = move |time: f64| {
        spawn(async move {
            if let Err(e) = create_metric(CreateMetric {
                metric_type_id: Uuid::parse_str("9ff7d3b0-11d5-450b-b0bf-92b312578fcb").unwrap(),
                value: time,
            })
            .await
            {
                info!("Failed to create metric: {:?}", e);
            }

            // Reset the form fields after submission
            hovered_at.set(None);
        });
        ()
    };

    rsx! {
        div {
            onmouseover: move |_| {
                if hovered_at.read().is_none() {
                    hovered_at.set(Some(Date::now()));
                }
            },
            onmouseleave: move |_| {
                if hovered_at.read().is_some() {
                    let time = Date::now() - hovered_at.read().unwrap();
                    track_mouse_time(time);
                }
            },
            {props.children}
        }
    }
}

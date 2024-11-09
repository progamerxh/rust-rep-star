use crate::queries::testimonials::create_testimonial;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use shared::models::CreateTestimonial;

#[component]
pub fn AddTestimonial() -> Element {
    let mut content = use_signal(|| String::new());
    let mut rating = use_signal(|| 0.0);

    let handle_submit = move |_: Event<FormData>| {
        spawn(async move {
            if let Err(e) = create_testimonial(CreateTestimonial {
                content: content.read().clone(),
                rating: *rating.read(),
                user_id: None,
            })
            .await
            {
                info!("Failed to create testimonial: {:?}", e);
            }

            // Reset the form fields after submission
            content.set(String::new());
            rating.set(0.0);
        });
        ()
    };

    rsx! {
        form { class: "p-4", onsubmit: handle_submit,
            div { class: "mb-4",
                label { class: "block text-gray-700", "Content:" }
                textarea {
                    class: "border rounded w-full py-2 px-3 text-gray-700",
                    value: "{content}",
                    oninput: move |e| content.set(e.value()),
                    placeholder: "write your testimonial here"
                }
            }
            div { class: "mb-4",
                label { class: "block text-gray-700", "Rating:" }
                input {
                    class: "border rounded w-full py-2 px-3 text-gray-700",
                    value: "{rating}",
                    r#type: "number",
                    oninput: move |e| rating.set(e.value().parse().unwrap_or(0.0)),
                    placeholder: "Enter testimonial content"
                }
            }
            button { class: "bg-blue-500 text-white px-4 py-2 rounded", "Submit" }
        }
    }
}

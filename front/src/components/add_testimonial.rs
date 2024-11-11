use crate::queries::testimonials::create_testimonial;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use shared::models::{CreateTestimonial, Testimonial};

#[component]
pub fn AddTestimonial(on_submit: Option<EventHandler<Testimonial>>) -> Element {
    let mut content = use_signal(|| String::new());
    let mut rating = use_signal(|| 0.0);

    let handle_submit = move |_: Event<FormData>| {
        spawn(async move {
            // if let Err(e) = create_testimonial(CreateTestimonial {
            //     content: content.read().clone(),
            //     rating: *rating.read() * 2.0,
            //     user_id: None,
            //     created_at: None,
            // })
            // .await
            // {
            //     info!("Failed to create testimonial: {:?}", e);
            // }

            let res = create_testimonial(CreateTestimonial {
                content: content.read().clone(),
                rating: *rating.read() * 2.0,
                user_id: None,
                created_at: None,
            })
            .await;

            match res {
                Ok(t) => {
                    if let Some(handler) = on_submit {
                        handler.call(t);
                    }
                }
                Err(e) => info!("Failed to create testimonial: {:?}", e),
            }

            // Reset the form fields after submission
            content.set(String::new());
            rating.set(0.0);
        });
        ()
    };

    rsx! {
        form { class: "h-full p-4", onsubmit: handle_submit,
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
                {(1..=5).map(|i| rsx! {
                    span {
                        class: "cursor-pointer",
                        onclick: move |_| rating.set(i as f64),
                        if *rating.read() >= i as f64 {
                            i { class: "text-yellow-500", "★" }
                        } else {
                            i { class: "text-gray-400", "★" }
                        }
                    }
                })}
            }
            button { class: "bg-blue-500 text-white px-4 py-2 rounded", "Submit" }
        }
    }
}

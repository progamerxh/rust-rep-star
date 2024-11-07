#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn AddTestimonial() -> Element {
    let mut content = use_signal(|| String::new());
    let mut rating = use_signal(|| 0.0);

    let on_submit = move |_event: Event<FormData>| {
        // Handle the submission logic here, e.g., send data to an API or update state
        // Reset the form fields after submission
        content.set(String::new());
        rating.set(0.0);
    };

    rsx! {
        form {
            class: "p-4",
            onsubmit: on_submit,
            div {
                class: "mb-4",
                label {
                    class: "block text-gray-700",
                    "Author:"
                }
                input {
                    class: "border rounded w-full py-2 px-3 text-gray-700",
                    r#type: "text",
                    value: "{content}",
                    oninput: move |e| content.set(e.value()),
                    placeholder: "Enter author name"
                }
            }
            div {
                class: "mb-4",
                label {
                    class: "block text-gray-700",
                    "Testimonial:"
                }
                textarea {
                    class: "border rounded w-full py-2 px-3 text-gray-700",
                    value: "{rating}",
                    oninput: move |e| content.set(e.value()),
                    placeholder: "Enter testimonial content"
                }
            }
            button {
                class: "bg-blue-500 text-white px-4 py-2 rounded",
                "Submit"
            }
        }
    }
}

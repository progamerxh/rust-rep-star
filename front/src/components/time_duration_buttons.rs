use dioxus::prelude::*;

const BUTTON_CLASS_DEFAULT: &'static str = "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
const BUTTON_CLASS_ACTIVE: &'static str = "bg-white text-foreground shadow";

#[derive(PartialEq, Clone, Props)]
pub struct TimeDurationButtonsProps {
    on_select: EventHandler<String>,
}

#[component]
pub fn TimeDurationButtons(props: TimeDurationButtonsProps) -> Element {
    let TimeDurationButtonsProps { on_select } = props;
    let mut selected_button = use_signal(|| "day".to_owned());

    let mut handle_click = move |value: String| {
        let value_clone = value.clone();
        selected_button.set(value_clone);
        on_select(value);
    };

    let class_name = |value: String| {
        format!(
            "{} {}",
            BUTTON_CLASS_DEFAULT,
            if *selected_button.read() == value {
                BUTTON_CLASS_ACTIVE
            } else {
                ""
            }
        )
    };

    rsx! {
        div { class: "inline-flex h-9 items-center justify-center rounded-lg bg-gray-200 p-1 text-muted-foreground",
            button {
                class: class_name("day".to_owned()),
                onclick: move |_| { handle_click("day".to_owned()) },
                "Day"
            }
            button {
                class: class_name("week".to_owned()),
                onclick: move |_| { handle_click("week".to_owned()) },
                "Week"
            }
            button {
                class: class_name("month".to_owned()),
                onclick: move |_| { handle_click("month".to_owned()) },
                "Month"
            }
            button {
                class: class_name("year".to_owned()),
                onclick: move |_| { handle_click("year".to_owned()) },
                "Year"
            }
        }
    }
}

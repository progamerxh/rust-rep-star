use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct MainLayoutProps {
    children: Element,
}

pub fn MainLayout(props: MainLayoutProps) -> Element {
    rsx! {
        div { class: "h-full w-full bg-gray-100 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]",
            header { class: "flex flex-row bg-opacity-5 backdrop-blur-sm p-4 rounded-lg shadow-lg sticky top-0 z-10",
                h1 { "Rep Star" }
                nav {
                    ul { class: "flex space-x-4 flex-row",
                        li {
                            a { href: "/", "Home" }
                        }
                        li {
                            a { href: "/page", "Page" }
                        }
                    }
                }
            }
            section {
                // Do not wider than 3xl
                div { class: "max-w-4xl mx-auto", {props.children} }
            }
            footer {
                p { "Footer" }
            }
        }
    }
}

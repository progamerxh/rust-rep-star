#![allow(non_snake_case)]

use crate::pages::add_testimonial::AddTestimonialPage;
use crate::pages::home::HomePage;
use crate::pages::testimonials::TestimonialsPage;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod components;
mod pages;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    HomePage {},
    #[route("/testimonials")]
    TestimonialsPage {},
    #[route("/add-testimonial")]
    AddTestimonialPage {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

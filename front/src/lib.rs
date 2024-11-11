use dioxus::prelude::*;

use crate::pages::{
    embed_testimonial::EmbedTestimonialPage, home::HomePage,
    manage_testimonial::ManageTestimonialPage,
};

mod components;
mod layouts;
mod pages;
mod queries;
mod utils;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    HomePage {},
    #[route("/manage-testimonials")]
    ManageTestimonialPage {},
    #[route("/embed-testimonial")]
    EmbedTestimonialPage {},
}

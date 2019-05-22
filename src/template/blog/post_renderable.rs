use maud::Markup;
use crate::data::models::{TextSection, Heading, Image};

pub trait PostRenderable {
    fn render(&self) -> Markup;
}

impl PostRenderable for Heading {
    fn render(&self) -> Markup {
        match self.heading_size {
            1 => html! { h1 { a name=(format!("h-{}", self.id)) { (self.heading_text) } } },
            2 => html! { h2 { a name=(format!("h-{}", self.id)) { (self.heading_text) } } },
            _ => html! { h3 { a name=(format!("h-{}", self.id)) { (self.heading_text) } } },
        }
    }
}

impl PostRenderable for Image {
    fn render(&self) -> Markup {
        html! {
            p { (format!("[Image: {}]", self.image_name)) }
        }
    }
}

impl PostRenderable for TextSection {
    fn render(&self) -> Markup {
        html! {
            p { (self.section_text) }
        }
    }
}
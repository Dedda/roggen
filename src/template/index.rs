use maud::Markup;
use crate::template::page;
use crate::template::elements::Link;

pub fn index(language: String) -> Markup {
    page(language, &Link::new("roggen", "/"),
    html! {
        h1 { "roggen" }
    })
}
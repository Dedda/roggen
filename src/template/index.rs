use maud::Markup;
use crate::template::page;
use crate::template::elements::Link;

pub fn index() -> Markup {
    page(&Link::new("roggen", "/"),
    html! {
        h1 { "roggen" }
    })
}
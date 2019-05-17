use maud::Markup;
use crate::template::page;

pub fn index() -> Markup {
    page(html! {
        h1 { "roggen" }
    })
}
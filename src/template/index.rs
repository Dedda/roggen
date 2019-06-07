use crate::template::elements::Link;
use crate::template::page;
use maud::Markup;

pub fn index(language: String) -> Markup {
    page(
        language,
        &Link::new("roggen", "/"),
        html! {
            h1 { "roggen" }
        },
    )
}

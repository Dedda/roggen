use maud::Markup;

use crate::template;

pub fn moto(page: Option<String>) -> Markup {
    template::page(html! {
        @if let Some(page) = page {
            h1 { (page) }
        } @else {
            (home())
        }
    })
}

fn home() -> Markup {
    html! {

    }
}
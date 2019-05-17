use rocket::error::LaunchError;
use maud::Markup;

use crate::template;
use rocket_contrib::serve::StaticFiles;
use std::borrow::Cow;
use rocket::http::RawStr;

#[get("/")]
fn index() -> Markup {
    template::index::index()
}

#[get("/blog/<name>?<page>")]
fn blog<'a>(name: &RawStr, page: Option<&RawStr>) -> Markup {
    let page = match page {
        Some(raw) => Some(raw.to_string()),
        None => None,
    };
    match name.as_str() {
        "moto" => template::blog::moto::moto(page),
        name => html! {
                h1 { "Blog " (name) " not found!" }
            },
    }
}

pub fn ignite() -> LaunchError{
    rocket::ignite()
        .mount("/css", StaticFiles::from("web/css"))
        .mount("/js", StaticFiles::from("web/js"))
        .mount("/", routes![index, blog])
        .launch()
}
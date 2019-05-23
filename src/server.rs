use rocket::error::LaunchError;
use maud::Markup;

use crate::{template, get_lang};
use rocket_contrib::serve::StaticFiles;
use rocket::http::Cookies;
use crate::template::blog::mount_blogs;

#[get("/")]
fn index(cookie: Cookies) -> Markup {
    template::index::index(get_lang(cookie))
}

pub fn ignite() -> LaunchError{
    let rocket = rocket::ignite()
        .mount("/css", StaticFiles::from("web/css"))
        .mount("/js", StaticFiles::from("web/js"));
    let rocket = mount_blogs(rocket);
    rocket.mount("/", routes![index]).launch()
}
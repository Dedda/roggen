use maud::Markup;
use rocket::error::LaunchError;

use crate::template::blog::mount_blogs;
use crate::{get_lang, template};
use rocket::http::Cookies;
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index(cookie: Cookies) -> Markup {
    template::index::index(get_lang(cookie))
}

pub fn ignite() -> LaunchError {
    let rocket = rocket::ignite()
        .mount("/css", StaticFiles::from("web/css"))
        .mount("/js", StaticFiles::from("web/js"));
    let rocket = mount_blogs(rocket);
    let rocket = crate::img::mount(rocket);
    rocket.mount("/", routes![index]).launch()
}

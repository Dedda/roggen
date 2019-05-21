use rocket::error::LaunchError;
use maud::Markup;

use crate::template;
use rocket_contrib::serve::StaticFiles;
use crate::template::blog::mount_blogs;

#[get("/")]
fn index() -> Markup {
    template::index::index()
}

pub fn ignite() -> LaunchError{
    let rocket = rocket::ignite()
        .mount("/css", StaticFiles::from("web/css"))
        .mount("/js", StaticFiles::from("web/js"));
    let rocket = mount_blogs(rocket);
    rocket.mount("/", routes![index]).launch()
}
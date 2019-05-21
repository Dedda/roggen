use maud::Markup;
use rocket::Rocket;

use crate::template::blog::embed_blog_contents;
use crate::template::elements::Link;

static ROOT: &'static str = "/blog/roggen";

lazy_static! {
    static ref ROGGEN_TITLE: Link = Link::new("roggen", ROOT);
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount(ROOT, routes![index])
}

#[get("/")]
fn index() -> Markup {
    embed_blog_contents(&ROGGEN_TITLE,
        html! {
        h1 { "Index" }
    })
}
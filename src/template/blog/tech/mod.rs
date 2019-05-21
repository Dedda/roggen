use maud::Markup;
use rocket::Rocket;

use crate::template::blog::embed_blog_contents;
use crate::template::elements::Link;

static ROOT: &'static str = "/blog/tech";

lazy_static! {
    static ref TECH_TITLE: Link = Link::new("Tech", ROOT);
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount(ROOT, routes![index])
}

#[get("/")]
fn index() -> Markup {
    embed_blog_contents(&TECH_TITLE,
        html! {
        h1 { "Index" }
    })
}
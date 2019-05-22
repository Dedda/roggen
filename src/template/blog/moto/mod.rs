use maud::Markup;
use rocket::Rocket;

use crate::template::blog::{blog_home, embed_blog_contents};
use crate::template::elements::Link;
use crate::data::read::{load_posts, load_post};
use crate::template::blog::post::render_post;

static ROOT: &'static str = "/blog/moto";

lazy_static! {
    static ref MOTO_TITLE: Link = Link::new("Moto", ROOT);
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount(ROOT, routes![index, post])
}

#[get("/")]
fn index() -> Markup {
    blog_home(&MOTO_TITLE, &load_posts("moto"))
}

#[get("/<id>")]
fn post(id: i32) -> Markup {
    let contents = match load_post(id) {
        Some(post) => render_post(&post),
        None => html! {
            h1 { "Post not found!" }
        }
    };
    embed_blog_contents(&MOTO_TITLE, contents)
}
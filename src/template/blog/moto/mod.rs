use maud::Markup;
use rocket::Rocket;

use crate::template::blog::{blog_home, embed_blog_contents};
use crate::template::elements::Link;

static ROOT: &'static str = "/blog/moto";

lazy_static! {
    static ref MOTO_TITLE: Link = Link::new("Moto", ROOT);
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount(ROOT, routes![index])
}

#[get("/")]
fn index() -> Markup {
    blog_home(&MOTO_TITLE, vec![])
}

pub fn moto(page: Option<String>) -> Markup {
    embed_blog_contents(&MOTO_TITLE,
                        html! {
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
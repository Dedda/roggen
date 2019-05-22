use crate::data::models::{Post, Heading, BelongsToPost};
use maud::Markup;
use crate::template::blog::post_renderable::PostRenderable;
use crate::data::read::{load_headings, load_images, load_texts};

trait PostPart: PostRenderable + BelongsToPost {}
impl<T> PostPart for T where T: PostRenderable + BelongsToPost {}

pub fn render_post(post: &Post) -> Markup {
    let headings: Vec<Heading> = load_headings(post.id);
    let images = load_images(post.id);
    let texts = load_texts(post.id);
    let mut parts: Vec<Box<&PostPart>> = vec![];
    for h in headings.iter() {
        parts.push(Box::new(h));
    }
    for i in images.iter() {
        parts.push(Box::new(i));
    }
    for t in texts.iter() {
        parts.push(Box::new(t));
    }
    parts.sort_by(|a, b| a.index().partial_cmp(&b.index()).unwrap());
    html! {
        div class="row" {
            div class="col-sm-3 blog-sidebar" {
                ol {
                    @for heading in &headings {
                        li {
                            a href=(format!("#h-{}", heading.id)) { (heading.heading_text) }
                        }
                    }
                }
            }
            div class="col-sm-9 blog-main" {
                div class="blog-post" {
                    @for part in &parts {
                        (part.render())
                    }
                }
            }
        }
    }
}

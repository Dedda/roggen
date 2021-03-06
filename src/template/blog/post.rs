use crate::data::models::{BelongsToPost, Heading, Post};
use crate::data::read::{load_headings, load_images, load_texts};
use crate::i18n::i18n;
use crate::template::blog::post_renderable::PostRenderable;
use maud::Markup;

trait PostPart: PostRenderable + BelongsToPost {}
impl<T> PostPart for T where T: PostRenderable + BelongsToPost {}

pub fn render_post(language: String, post: &Post) -> Markup {
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
    let published = match post.published {
        Some(p) => p.format("%m/%d/%Y %H:%M:%S UTC").to_string(), //6/29/2011 4:52:48 PM
        None => "".to_string(),
    };
    let pub_text = i18n(&language, "published").or_else("published");
    html! {
        div class="jumbotron" {
            div { h1 { (post.title) } }
            div { small { (format!("{}: ", pub_text)) span data-published=(published) {} } }
        }
        div class="row" {
            (sidebar(&headings))
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

fn sidebar(headings: &Vec<Heading>) -> Markup {
    html! {
        div class="col-sm-3 blog-sidebar" {
            div class="sidebar-module" {
                ol class="post-section-list" {
                    @for heading in headings {
                        li {
                            a href=(format!("#{}", heading_id(&heading))) { (heading.heading_text) }
                        }
                    }
                }
            }
        }
    }
}

pub fn heading_id(heading: &Heading) -> String {
    format!("h-{}", heading.id)
}

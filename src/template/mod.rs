use maud::Markup;
use crate::template::elements::Link;

pub mod blog;
pub mod elements;
pub mod index;

pub fn page(title: &Link, contents: Markup) -> Markup {
    html! {
        html lang="de" {
            head {
                (head())
            }
            body {
                script src="/js/jquery-1.11.3.min.js" {}
                script src="/js/bootstrap.min.js" {}
                (header(title))
                (body_main(contents))
                (footer())
            }
        }
    }
}

fn head() -> Markup {
    html! {
        meta charset="utf-8" {}
        meta name="viewport" content="width=device-width, initial-scale=1" {}
        link href="/css/bootstrap.min.css" rel="stylesheet" {}
        link href="/css/base.css" rel="stylesheet" {}
    }
}

fn header(title: &Link) -> Markup {
    struct NavPage {
        path: &'static str,
        title: &'static str,
    }
    let pages = vec![
        NavPage {
            path: "/",
            title: "Home",
        }
    ];
    html! {
        nav {
            nav id="navigation-bar" class="navbar navbar-default navbar-inverse navbar-fixed-top" role="navigation" {
                div class="container" {
                    div class="navbar-header" {
                        button type="button" class="navbar-toggle" data-toggle="collapse" data-target="#navbarCollapse" {
                            span class="sr-only" { "Toggle navigation" }
                            @for _page in &pages {
                                span class="icon-bar" {}
                            }
                        }
                        a class="navbar-brand" href=(title.href) { (title.title) }
                    }
                    div id="navbarCollapse" class="collapse navbar-collapse" {
                        ul class="nav navbar-nav" {
                            @for page in pages {
                                li {
                                    a class="nav-link" href=(page.path) { (page.title) }
                                }
                            }
                            li class="nav-item dropdown" {
                                a class="nav-link dropdown-toggle" href="#" id="blogsDropdown" role="button" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false" {
                                    "Blogs"
                                }
                                ul class="dropdown-menu" aria-labelledby="blogsDropdown" {
                                    li { a class="dropdown-item" href="/blog/moto" { "Moto" } }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn body_main(contents: Markup) -> Markup{
    html! {
        div class="container" {
            div class="jumbotron" {
                (contents)
            }
        }
    }
}

fn footer() -> Markup {
    html! {
    }
}
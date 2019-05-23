#[derive(Clone)]
pub struct Link {
    pub title: String,
    pub href: String,
}

impl Link {
    pub fn new<S>(title: S, href: S) -> Link where S: ToString {
        Link {
            title: title.to_string(),
            href: href.to_string(),
        }
    }

    pub fn for_blog_name<S>(name: S) -> Link where S: ToString {
        Link {
            title: name.to_string(),
            href: format!("/blog/{}", name.to_string().to_ascii_lowercase()),
        }
    }
}
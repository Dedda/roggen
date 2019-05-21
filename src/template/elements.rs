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
}
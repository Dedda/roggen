use maud::Markup;

pub fn language_selector(selected: &str) -> Markup {
    html! {
        li class="nav-item dropdown" {
            a class="nav-link dropdown-toggle" href="#" id="langDropdown" role="button" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false" {
                (icon(selected))
                (lang(selected))
            }
            ul class="dropdown-menu" aria-labelledBy="langDropdown" {
                li { a class="dropdown-item" href="#" data-toggle-language="en" { (icon("en")) (lang("en")) } }
                li { a class="dropdown-item" href="#" data-toggle-language="de" { (icon("de")) (lang("de")) } }
            }
        }
    }
}

fn icon(lang: &str) -> Markup {
    html! {
        span class=(format!("flag-icon flag-icon-{}", icon_name(lang))) {}
    }
}

fn icon_name(lang: &str) -> String {
    match lang {
        "en" => "us".to_string(),
        x => x.to_string(),
    }
}

fn lang(code: &str) -> &'static str {
    match code {
        "de" => " Deutsch",
        _ => " English"
    }
}
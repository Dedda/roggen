use std::fmt::{Display, Error, Formatter};
use uebersetzt::{Config, TranslationResult, Uebersetzt};

lazy_static! {
    static ref UE: Uebersetzt = Uebersetzt::new(Config::new(
        "./i18n".to_string(),
        vec!["en".to_string(), "de".to_string(),]
    ));
}

pub struct I18N {
    val: Option<String>,
}

impl I18N {
    fn with_value(value: String) -> I18N {
        I18N { val: Some(value) }
    }
    fn empty() -> I18N {
        I18N { val: None }
    }

    pub fn or_else<S>(&self, fallback: S) -> String
    where
        S: ToString,
    {
        match &self.val {
            Some(v) => v.clone(),
            _ => fallback.to_string(),
        }
    }
}

impl Display for I18N {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&self.or_else(""))
    }
}

pub fn i18n(lang: &str, key: &str) -> I18N {
    match UE.get(lang, key) {
        TranslationResult::Ok(s) => I18N::with_value(s),
        _ => I18N::empty(),
    }
}

//! Search bar

use seed::{prelude::*, *};

#[derive(Clone)]
pub struct Mdl {
    pub search_term: String,
    pub placeholder: Option<String>,
    pub attrs: Attrs,
    pub clear_label: String,
}

impl Default for Mdl {
    fn default() -> Self {
        Self {
            search_term: String::new(),
            placeholder: Some("What are you looking for? (# for tags)".to_string()),
            attrs: attrs! {},
            clear_label: "clear".to_string(),
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    Search(String),
    Clear,
}

pub fn view(mdl: &Mdl) -> Node<Msg> {
    div![
        &mdl.attrs,
        input![
            attrs! {
                At::Value => mdl.search_term;
                At::Type => "search";
            },
            if let Some(p) = &mdl.placeholder {
                attrs! {
                    At::Placeholder => p;
                }
            } else {
                attrs! {}
            },
            input_ev(Ev::Input, Msg::Search)
        ],
        if !mdl.search_term.is_empty() {
            button![&mdl.clear_label, simple_ev(Ev::Click, Msg::Clear)]
        } else {
            empty!()
        }
    ]
}

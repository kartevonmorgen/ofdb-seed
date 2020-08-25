//! Login form
//!
//! # Example
//!
//! ```rust
//! use ofdb_seed::components::login;
//! use seed::{*, prelude::*};
//!
//! let mdl = login::Mdl {
//!    is_submitting: false,
//!    attrs: class!["login-form"],
//! };
//! let login = login::view(&mdl);
//!  ```

use seed::{prelude::*, *};

// TODO
// - Support i18n
// - Show error messages
#[derive(Clone)]
pub struct Mdl {
    pub email: String,
    pub password: String,
    pub is_submitting: bool,
    pub attrs: Attrs,
}

impl Default for Mdl {
    fn default() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            is_submitting: false,
            attrs: attrs! {},
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    EmailChanged(String),
    PasswordChanged(String),
    Submit,
}

pub fn view(mdl: &Mdl) -> Node<Msg> {
    form![
        attrs! {
            At::Action => "javascript:void(0);";
        },
        &mdl.attrs,
        fieldset![
            label![
                "E-Mail",
                input![
                    input_ev(Ev::Input, Msg::EmailChanged),
                    attrs! {
                        At::Type => "email";
                        At::Name => "email";
                        At::Required => true.as_at_value();
                        At::Value => mdl.email;
                        At::Disabled => mdl.is_submitting.as_at_value();
                    }
                ]
            ],
            label![
                "Password",
                input![
                    input_ev(Ev::Input, Msg::PasswordChanged),
                    attrs! {
                        At::Type => "password";
                        At::Name => "password";
                        At::Required => true.as_at_value();
                        At::Value => mdl.password;
                        At::Disabled => mdl.is_submitting.as_at_value();
                    }
                ]
            ]
        ],
        input![
            simple_ev(Ev::Click, Msg::Submit),
            attrs! {
                At::Value => "login";
                At::Type => "submit";
                At::Disabled => mdl.is_submitting.as_at_value();
            }
        ]
    ]
}

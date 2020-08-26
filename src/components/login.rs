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
#[derive(Clone)]
pub struct Mdl {
    pub email: String,
    pub password: String,
    pub is_submitting: bool,
    pub attrs: Attrs,
    pub errors: Errors,
}

#[derive(Clone, Default)]
pub struct Errors {
    pub form: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl Default for Mdl {
    fn default() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            is_submitting: false,
            attrs: attrs! {},
            errors: Default::default(),
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
        if let Some(msg) = &mdl.errors.form {
            div![class!["error"], msg]
        } else {
            empty!()
        },
        fieldset![
            legend!["Login"],
            label![
                span!["E-Mail:"],
                input![
                    input_ev(Ev::Input, Msg::EmailChanged),
                    attrs! {
                        At::Type => "email";
                        At::Name => "email";
                        At::Placeholder => "Enter e-mail address";
                        At::Required => true.as_at_value();
                        At::Value => mdl.email;
                        At::Disabled => mdl.is_submitting.as_at_value();
                    }
                ],
                if let Some(msg) = &mdl.errors.email {
                    div![class!["error"], msg]
                } else {
                    empty!()
                }
            ],
            label![
                span!["Password:"],
                input![
                    input_ev(Ev::Input, Msg::PasswordChanged),
                    attrs! {
                        At::Type => "password";
                        At::Name => "password";
                        At::Placeholder => "Enter password";
                        At::Required => true.as_at_value();
                        At::Value => mdl.password;
                        At::Disabled => mdl.is_submitting.as_at_value();
                    }
                ],
                if let Some(msg) = &mdl.errors.password {
                    div![class!["error"], msg]
                } else {
                    empty!()
                }
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

use ofdb_seed::{components::login, Api};
use seed::{prelude::*, *};

#[derive(Clone)]
struct Mdl {
    api: Api,
    page: Page,
}

#[derive(Clone)]
enum Page {
    Home,
    Login(login::Mdl),
}

impl Default for Mdl {
    fn default() -> Self {
        Self {
            api: Api::new("https://api.ofdb.io/v0".to_string()),
            page: Page::Login(Default::default()),
        }
    }
}

enum Msg {
    Login(login::Msg),
    LoginResult(fetch::Result<()>),
    Logout,
    LogoutResult(fetch::Result<()>),
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Mdl {
    Mdl::default()
}

fn update(msg: Msg, mdl: &mut Mdl, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Login(msg) => match &mut mdl.page {
            Page::Login(login_mdl) => match msg {
                login::Msg::EmailChanged(email) => {
                    login_mdl.email = email;
                }
                login::Msg::PasswordChanged(pw) => {
                    login_mdl.password = pw;
                }
                login::Msg::Submit => {
                    login_mdl.is_submitting = true;
                    let email = login_mdl.email.clone();
                    let password = login_mdl.password.clone();
                    let api = mdl.api.clone();
                    orders.perform_cmd(async move {
                        Msg::LoginResult(api.post_login(email, password).await)
                    });
                }
            },
            _ => unreachable!(),
        },
        Msg::LoginResult(res) => {
            match res {
                Ok(()) => {
                    mdl.page = Page::Home;
                }
                Err(err) => {
                    // TODO: show err
                    seed::error!(err);
                }
            }
        }
        Msg::Logout => {
            let api = mdl.api.clone();
            orders.perform_cmd(async move { Msg::LogoutResult(api.post_logout().await) });
        }
        Msg::LogoutResult(res) => {
            match res {
                Ok(()) => {
                    mdl.page = Page::Login(Default::default());
                }
                Err(err) => {
                    // TODO: show err
                    seed::error!(err);
                }
            }
        }
    }
}

fn view(mdl: &Mdl) -> Node<Msg> {
    match &mdl.page {
        Page::Home => div![
            h1!["OpenFairDB"],
            p!["You are logged in"],
            button![ev(Ev::Click, |_| Msg::Logout), "logout"]
        ],
        Page::Login(mdl) => div![
            h1!["OpenFairDB Login"],
            login::view(&mdl).map_msg(Msg::Login)
        ],
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

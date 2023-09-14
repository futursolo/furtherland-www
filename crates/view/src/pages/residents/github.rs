use atoms::{ErrorState, TokenState};
use bounce::prelude::*;
use components::Main;
use serde::{Deserialize, Serialize};

use crate::api::{Bridge, ExchangeTokenInput, ExchangeTokenMutation};
use crate::prelude::*;

#[derive(Serialize, Deserialize)]
struct OauthContinueQuery {
    next: String,
    code: String,
}

#[styled_component(OauthContinue)]
pub(crate) fn oauth_continue() -> Html {
    let exchange_token = Bridge::use_mutation::<ExchangeTokenMutation>();
    let set_error = use_atom_setter::<ErrorState>();
    let set_token = use_atom_setter::<TokenState>();
    let history = use_navigator().unwrap_throw();

    let location = use_location().unwrap_throw();

    use_effect_with_deps(
        move |_| {
            match location.query::<OauthContinueQuery>() {
                Ok(m) => {
                    let input = ExchangeTokenInput { code: m.code };
                    let next = m.next;

                    spawn_local(async move {
                        let result = exchange_token.run(input).await;

                        match result {
                            Ok(m) => {
                                set_token(TokenState {
                                    inner: Some(m.content.access_token.clone()),
                                });

                                let path = if next.starts_with('/') {
                                    next.as_str()
                                } else {
                                    "/"
                                };

                                let route =
                                    AppRoute::recognize(path).unwrap_or(AppRoute::HomeRedirect);

                                history.push(&route);
                            }
                            Err(_e) => {
                                set_error(ErrorKind::Server.into());
                            }
                        }
                    });
                }
                Err(_e) => set_error(ErrorKind::Server.into()),
            }

            || {}
        },
        (),
    );

    html! {
        <Main>
            <div>
                {"Please wait..."}
            </div>
        </Main>
    }
}

use atoms::{ErrorState, TokenState};
use bounce::prelude::*;
use bounce::query::use_mutation;
use components::Main;
use serde::{Deserialize, Serialize};

use crate::api::ExchangeTokenMutation;
use crate::prelude::*;

#[derive(Serialize, Deserialize)]
struct OauthContinueQuery {
    next: String,
    code: String,
}

#[styled_component(OauthContinue)]
pub(crate) fn oauth_continue() -> Html {
    let exchange_token = use_mutation::<ExchangeTokenMutation>();
    let set_error = use_atom_setter::<ErrorState>();
    let set_token = use_atom_setter::<TokenState>();
    let history = use_navigator().unwrap_throw();

    let location = use_location().unwrap_throw();

    use_effect_with_deps(
        move |_| {
            match location.query::<OauthContinueQuery>() {
                Ok(m) => {
                    let input = messages::AccessTokenInput { code: m.code };
                    let next = m.next;

                    spawn_local(async move {
                        let result = exchange_token.run(input).await;

                        match result {
                            Ok(m) => {
                                set_token(TokenState {
                                    inner: Some(m.content.access_token.clone()),
                                });

                                let path = next
                                    .parse::<reqwest::Url>()
                                    .ok()
                                    .and_then(|url| {
                                        if let Some(m) = url.host() {
                                            if Some(m.to_string())
                                                != window().location().host().ok()
                                            {
                                                None
                                            } else {
                                                Some(url)
                                            }
                                        } else {
                                            Some(url)
                                        }
                                    })
                                    .map(|m| m.path().to_string())
                                    .unwrap_or_else(|| "/".to_string());

                                let route =
                                    AppRoute::recognize(&path).unwrap_or(AppRoute::HomeRedirect);

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

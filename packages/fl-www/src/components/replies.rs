use crate::api::{CurrentResidentQuery, RepliesQuery, RepliesQueryInput};
use crate::prelude::*;
use crate::utils::is_ssr;
use atoms::TokenState;
use components::{Author, AuthoringResident, Placeholder, PlaceholderKind, Textarea};

use bounce::query::use_query_value;
use bounce::*;
use serde::{Deserialize, Serialize};
use web_sys::HtmlTextAreaElement;
use yew_feather::github::Github;

#[derive(Properties, PartialEq, Debug)]
struct ReplyProps {
    content: messages::Reply,
}

#[styled_component(Reply)]
fn reply(props: &ReplyProps) -> Html {
    let resident = AuthoringResident::Other(props.content.resident.clone());
    let create_date = props.content.created_at.naive_local().date();
    let create_time = props.content.created_at.naive_local().time();

    html! {
        <div class={css!(
            r#"
                width: 100%;
            "#
        )}>
            <Author
                author={resident}
                date={create_date}
                time={create_time}
            />
            <div class={css!(
                r#"
                    width: 100%;
                    padding-top: 20px;
                    padding-bottom: 20px;
                "#
            )}>
                {props.content.content.clone()}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Debug)]
pub(crate) struct RepliesProps {
    pub slug: String,
}

#[styled_component(RepliesLoading)]
fn replies_loading() -> Html {
    let theme = use_theme();

    html! {
        <>
            <div class={css!(
                r#"
                    display: flex;
                    width: 100%;
                    padding-bottom: 20px;

                    flex-direction: row;
                    align-items: center;
                    justify-content: flex-start;
                "#,
            )}>
                <Placeholder
                    height="50px"
                    width="50px"
                    kind={PlaceholderKind::Circle}
                    set_data_status={false}
                />
                <div class={css!(
                    r#"
                        display: flex;
                        flex-direction: column;
                        justify-content: space-around;
                        height: 50px;

                        padding-left: 10px;
                        padding-right: 10px;
                    "#,
                )}>
                    <Placeholder
                        height={theme.font_size.default.to_string()}
                        width="200px"
                        set_data_status={false}
                    />
                    <Placeholder
                        height={theme.font_size.secondary.to_string()}
                        width="130px"
                        set_data_status={false}
                    />
                </div>
            </div>
            <div class={css!("
                margin-bottom: 10px;
                width: 100%;
                height: 100px;
            ")}>
                <Placeholder height="1rem" width="100%" set_data_status={false} />
            </div>
        </>
    }
}

#[styled_component(NoReplies)]
fn no_replies() -> Html {
    use_language();
    let theme = use_theme();

    html! {
        <div class={css!(r#"
            flex-grow: 1;

            display: flex;
            flex-direction: row;
            justify-content: space-around;
            align-items: center;
        "#)}>
            <div class={css!(
                r#"
                    color: ${colour};
                    font-size: 2rem;
                "#,
                colour = css_var!(theme.colour.text.hint)
            )}>{fl!("no-comments")}</div>
        </div>
    }
}

#[styled_component(RepliesContent)]
fn replies_content(props: &RepliesProps) -> Html {
    let lang = use_language();
    let set_error = use_atom_setter::<ErrorState>();

    let replies = use_query_value::<RepliesQuery>(
        RepliesQueryInput {
            slug: props.slug.clone(),
            lang,
        }
        .into(),
    );

    let replies = match replies.result() {
        None => return html! {<RepliesLoading />},
        Some(Ok(m)) => m,
        Some(Err(_e)) => {
            return {
                set_error(ErrorKind::Server.into());

                html! {<RepliesLoading />}
            }
        }
    };

    if replies.content.replies.is_empty() {
        return html! {<NoReplies />};
    }

    let replies = replies
        .content
        .replies
        .iter()
        // TODO: show all of the contents to mod residents.
        .filter(|m| m.approved.unwrap_or(false))
        .map(|m| html! {<Reply content={m.clone()} />});

    html! { <div class={css!("width: 100%;")}>{for replies}</div> }
}

#[derive(Serialize, Deserialize, Debug)]
struct OAuthQuery {
    client_id: &'static str,
    redirect_uri: String,
}

#[derive(Properties, PartialEq, Clone)]
struct NewReplyAreaProps {
    resident: messages::Resident,
}

#[styled_component(NewReplyArea)]
fn new_reply_area(props: &NewReplyAreaProps) -> Html {
    use_language();

    let set_token_state = use_atom_setter::<TokenState>();

    let NewReplyAreaProps { resident } = props.clone();
    let theme = use_theme();

    let reply_value = use_state_eq(|| "".to_string());

    let on_reply_input = {
        let reply_value = reply_value.clone();

        Callback::from(move |e: InputEvent| {
            let target = e.target_unchecked_into::<HtmlTextAreaElement>();

            reply_value.set(target.value());
        })
    };

    let signout = {
        Callback::from(move |_| {
            set_token_state(TokenState { inner: None });
        })
    };

    html! {
        <div
            class={css!(r#"
                width: 100%;
            "#)}
        >
            <Author author={AuthoringResident::Other(Some(resident))} />
            <Textarea
                value={(*reply_value).clone()}
                oninput={on_reply_input}
                class={classes!(css!(r#"
                    width: 100%;
                    height: 200px;
                "#))}
            />
            <div class={css!(
                r#"
                    width: 100%;
                    display: flex;
                    flex-direction: row;
                    justify-content: flex-end;
                    align-items: center;
                    padding-top: 15px;

                    @media ${sm_down} {
                        flex-direction: column;
                        justify-content: center;
                        align-items: flex-end;
                    }
                "#,
                sm_down = theme.breakpoint.sm.down(),
            )}>
                <div
                    class={css!(
                        r#"
                            padding-right: 15px;

                            color: ${sec_colour};


                            @media ${sm_down} {
                                padding-right: 0;
                                padding-bottom: 15px;
                            }
                        "#,
                        sm_down = theme.breakpoint.sm.down(),
                        sec_colour = theme.colour.text.secondary,
                    )}
                >
                    {fl!("comment-supports-markdown")}
                </div>
                <div
                    class={css!(
                        r#"
                            display: flex;
                            flex-direction: row;
                            align-items: center;
                            justify-content: flex-end;
                        "#
                    )}
                >
                    <div
                        onclick={signout}
                        class={css!(
                            r#"
                                background-color: ${bg_colour};
                                height: 40px;
                                text-align: center;
                                display: flex;
                                flex-direction: row;
                                justify-content: space-around;
                                align-items: center;
                                border-radius: 80px;
                                padding-left: 20px;
                                padding-right: 20px;
                                font-size: 1rem;
                                font-weight: bold;
                                color: ${colour};
                                pointer: default;

                                transition: 0.2s background-color;

                                :hover {
                                    background-color: ${bg_hover_colour};
                                }
                            "#,
                            colour = css_var!(theme.colour.text.primary),
                            bg_colour = css_var!(theme.colour.secondary),
                            bg_hover_colour = css_var!(theme.colour.secondary_hover),
                        )}
                    >
                        {fl!("signout")}
                    </div>
                    <div class={css!(r#"width: 15px;"#)} />
                    <div
                        class={css!(
                            r#"
                                background-color: ${bg_colour};
                                height: 40px;
                                text-align: center;
                                display: flex;
                                flex-direction: row;
                                justify-content: space-around;
                                align-items: center;
                                border-radius: 80px;
                                padding-left: 20px;
                                padding-right: 20px;
                                font-size: 1rem;
                                font-weight: bold;
                                color: white;
                                pointer: default;

                                transition: 0.2s background-color;

                                :hover {
                                    background-color: ${bg_hover_colour};
                                }
                            "#,
                            bg_colour = css_var!(theme.colour.primary),
                            bg_hover_colour = css_var!(theme.colour.primary_hover),
                        )}
                    >
                        {fl!("comment")}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[styled_component(NewReply)]
pub(crate) fn new_reply() -> Html {
    let current_resident = use_query_value::<CurrentResidentQuery>(().into());
    let set_error = use_atom_setter::<ErrorState>();

    let navigate_to_github = Callback::from(|_| {
        let queries = OAuthQuery {
            client_id: option_env!("FL_WWW_GITHUB_CLIENT_ID").unwrap_throw(),
            redirect_uri: format!(
                "http://localhost:9741/residents/github/continue?next={}",
                window().location().href().unwrap_throw()
            ),
        };

        let next_url = format!(
            "https://github.com/login/oauth/authorize?{}",
            serde_urlencoded::ser::to_string(queries).unwrap_throw(),
        );

        window().location().set_href(&next_url).unwrap_throw();
    });

    let comment_area = match current_resident.result() {
        None => html! {<RepliesLoading />},
        Some(Ok(m)) => match &m.content {
            Some(ref m) => html! {<NewReplyArea resident={m.clone()} />},
            None => html! {
                <div class={css!(
                    r#"
                        width: 100%;
                        height: 100px;

                        display: flex;
                        justify-content: space-around;
                        align-items: center;
                        flex-direction: row;
                    "#
                )}>
                    <div
                        class={css!(
                            r#"
                                background-color: rgb(36, 40, 46);
                                border-radius: 80px;
                                height: 40px;
                                padding-left: 20px;
                                padding-right: 20px;

                                display: inline-flex;
                                justify-content: center;
                                align-items: center;
                                flex-direction: row;

                                color: white;
                                font-weight: bold;
                                cursor: pointer;
                                font-size: 1rem;

                                transition: 0.2s background-color;

                                :hover {
                                    background-color: rgb(78, 82, 87);
                                }
                            "#
                        )}
                        onclick={navigate_to_github}
                    >
                        <Github size={15} />
                        <div class={css!("width: 5px;")} />
                        {fl!("signin-github")}
                    </div>
                </div>
            },
        },
        Some(Err(_e)) => {
            set_error(ErrorKind::Server.into());

            html! {<RepliesLoading />}
        }
    };

    html! {
        <div class={css!("width: 100%;")}>
            <h2 class={css!("width: 100%;")}>{fl!("new-comment")}</h2>
            {comment_area}
        </div>
    }
}

#[styled_component(Replies)]
pub(crate) fn replies(props: &RepliesProps) -> Html {
    use_language();

    let show_content = use_state(|| false);

    {
        let show_content = show_content.clone();
        use_effect_with_deps(
            move |_| {
                if !is_ssr() {
                    show_content.set(true);
                }

                || {}
            },
            (),
        );
    }

    let content = if !*show_content {
        html! {<RepliesLoading />}
    } else {
        html! {<RepliesContent slug={props.slug.clone()} />}
    };

    html! {
        <div class={css!(r#"
            min-height: 150px;
            width: 100%;

            display: flex;
            flex-direction: column;
            justify-content: space-around;
            align-items: center;
        "#)}>
            <h1 class={css!("width: 100%;")}>{fl!("comments")}</h1>
            {content}
            <NewReply />
        </div>
    }
}

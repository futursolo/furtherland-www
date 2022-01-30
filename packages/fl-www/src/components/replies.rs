use crate::api::{RepliesQuery, RepliesQueryInput};
use crate::prelude::*;
use crate::utils::is_ssr;
use components::{Placeholder, PlaceholderKind};

use bounce::*;
use bounce_query::use_query_value;

#[derive(Properties, PartialEq, Debug)]
struct ReplyProps {
    content: messages::Reply,
}

#[styled_component(Reply)]
fn reply(props: &ReplyProps) -> Html {
    html! {}
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
                height: 200px;
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

    html! {"hello?"}
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
        </div>
    }
}

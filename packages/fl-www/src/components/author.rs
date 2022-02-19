use chrono::{NaiveDate, NaiveTime};

use crate::prelude::*;

static MY_AVATAR_URL: Cow<'static, str> =
    Cow::Borrowed("https://www.gravatar.com/avatar/0dd494a963ae648caebe34288b664ca6?s=200");

static MP_AVATAR_URL: Cow<'static, str> =
    Cow::Borrowed("https://www.gravatar.com/avatar/a07f70e42724f28eddfb2bf1ece6fe9a?s=200&d=mp");

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum AuthoringResident {
    Default,
    Other(Option<messages::Resident>),
}

// Both Date time Optional time are local to browser if possible.
#[derive(Properties, Clone, PartialEq)]
pub(crate) struct AuthorProps {
    pub date: NaiveDate,
    #[prop_or_default]
    pub time: Option<NaiveTime>,

    pub author: AuthoringResident,
}

#[styled_component(Author)]
pub(crate) fn author(props: &AuthorProps) -> Html {
    let date_str = props.date.format("%Y-%m-%d");
    // notice the leading space for time.
    let time_str = props
        .time
        .as_ref()
        .map(|m| Cow::from(m.format(" %H:%M:%S").to_string()))
        .unwrap_or_else(|| "".into());

    let theme = use_theme();
    use_language();

    let (resident_name, resident_avatar_url) = match &props.author {
        AuthoringResident::Default => (fl!("my-name"), MY_AVATAR_URL.clone()),
        AuthoringResident::Other(Some(m)) => {
            if m.id == 11693215 {
                (fl!("my-name"), MY_AVATAR_URL.clone())
            } else {
                (m.name.to_string(), Cow::from(m.avatar_url.to_owned()))
            }
        }
        AuthoringResident::Other(None) => (fl!("removed-resident"), MP_AVATAR_URL.clone()),
    };

    html! {
        <div class={css!(
            r#"
                display: flex;
                width: 100%;
                padding-bottom: 10px;

                flex-direction: row;
                align-items: center;
                justify-content: flex-start;
            "#,
        )}>
            <div class={css!(
                r#"
                    background-image: url(${avatar_url});
                    height: 50px;
                    width: 50px;
                    border-radius: 50px;
                    background-repeat: no-repeat;
                    background-size: cover;
                "#,
                avatar_url = resident_avatar_url
            )}></div>
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
                <div class={css!(
                    r#"
                        font-size: ${font_size};
                        color: ${colour};
                    "#,
                    font_size = &theme.font_size.default,
                    colour = css_var!(theme.colour.text.primary),
                )}>{resident_name}</div>
                <div class={css!(
                    r#"
                        font-size: ${font_size};
                        color: ${colour};
                    "#,
                    font_size = &theme.font_size.secondary,
                    colour = css_var!(theme.colour.text.secondary),
                )}>{date_str}{time_str}</div>
            </div>
        </div>
    }
}

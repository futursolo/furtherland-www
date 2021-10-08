use chrono::NaiveDate;

use crate::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct WritingInfoProps {
    pub date: NaiveDate,
}

#[styled_component(WritingInfo)]
pub(crate) fn writing_info(props: &WritingInfoProps) -> Html {
    let date_str = props.date.format("%Y-%m-%d");
    let theme = use_theme();

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
                avatar_url = "https://www.gravatar.com/avatar/0dd494a963ae648caebe34288b664ca6"
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
                    colour = theme.colour.text.primary,
                )}>{fl!("my-name")}</div>
                <div class={css!(
                    r#"
                        font-size: ${font_size};
                        color: ${colour};
                    "#,
                    font_size = &theme.font_size.secondary,
                    colour = theme.colour.text.secondary,
                )}>{date_str}</div>
            </div>
        </div>
    }
}

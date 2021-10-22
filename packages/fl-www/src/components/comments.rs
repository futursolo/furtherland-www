use crate::prelude::*;

#[styled_component(Comments)]
pub(crate) fn comments() -> Html {
    let theme = use_theme();
    use_language();

    html! {
        <div class={css!(r#"
            height: 150px;
            width: 100%;

            display: flex;
            flex-direction: column;
            justify-content: space-around;
            align-items: center;
        "#)}>
            <h1 class={css!("width: 100%;")}>{fl!("comments")}</h1>
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
                )}>{"Coming Soon..."}</div>
            </div>
        </div>
    }
}

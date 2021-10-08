use crate::prelude::*;

#[styled_component(Content)]
pub(crate) fn content() -> Html {
    let theme = use_theme();

    html! {
        <div class={css!(r#"
            height: 1px;
            width: 100%;

            flex-grow: 1;

            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: space-around;
        "#)}>
            <div class={css!(
                r#"
                font-size: 5rem;

                @media ${lg_down} {
                    font-size: 4rem;
                }

                @media ${md_down} {
                    font-size: 3rem;
                }

                @media ${sm_down} {
                    font-size: 2rem;
                }
                "#,
                lg_down = theme.breakpoint.lg.down(),
                md_down = theme.breakpoint.md.down(),
                sm_down = theme.breakpoint.sm.down(),
            )}>{fl!("default-title")}</div>
         </div>
    }
}

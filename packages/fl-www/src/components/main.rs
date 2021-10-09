use crate::prelude::*;

#[styled_component(Main)]
pub(crate) fn main(props: &ChildrenProps) -> Html {
    let children = props.children.clone();
    let theme = use_theme();
    let route = use_app_route();

    let min_height_str = match route {
        AppRoute::HomeZh | AppRoute::HomeEn => "calc(100vh - 160px)",
        _ => "auto",
    };

    html! {
        <main class={css!(
            r#"
                display: flex;
                width: 100%;
                flex-grow: 1;
                padding-top: 20px;
                padding-bottom: 20px;
                min-height: ${min_height_str};

                flex-direction: column;
                align-items: center;
                justify-content: flex-start;
            "#,
            min_height_str = min_height_str,
        )}>
            <div class={css!(
                r#"
                    width: calc(100% - 40px);
                    max-width: ${md_width};
                "#,
                md_width = theme.breakpoint.md.width_str(),
            )}>
                {children}
            </div>
        </main>
    }
}

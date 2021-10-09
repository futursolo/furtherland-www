use chrono::{Datelike, Local};

use crate::prelude::*;

use components::FlexSpace;
use hooks::use_render_event;

#[styled_component(Footer)]
pub(crate) fn footer() -> Html {
    let year = Local::now().year();
    use_language();

    let theme = use_theme();

    let is_vertical = theme.breakpoint.md.matches_down();

    use_render_event(&window(), "resize");
    use_render_event(&window(), "orientationchange");

    html! {
        <footer class={css!(
            r#"
                height: 100px;
                width: 100%;

                display: flex;
                flex-direction: row;
                justify-content: space-around;
                align-items: center;

                box-sizing: border-box;
            "#
        )}>
            <div class={css!(
                r#"
                    max-width: calc(${md_width} - 40px);
                    width: 100%;

                    display: flex;
                    flex-direction: row;
                    justify-content: space-around;
                    align-items: center;

                    @media ${md_down} {
                        flex-direction: column;
                    }
                "#,
                md_down = theme.breakpoint.md.down(),
                md_width = theme.breakpoint.md.width_str()
            )}>
                <div>
                    {format!("© {} ", year)}
                    {fl!("default-title")}
                </div>
                {if is_vertical { html!{<FlexSpace />} } else { html!{} }}
                <div class={css!(
                    r#"
                        color: ${colour};
                    "#,
                    colour = theme.colour.text.secondary,
                )}>
                    {fl!("footer-copy")}
                </div>
            </div>
        </footer>
    }
}

use styling::use_media_query;
use web_sys::{ScrollBehavior, ScrollIntoViewOptions};
use yew_feather::chevron_down::ChevronDown;

use crate::prelude::*;

#[styled_component(HomeContent)]
pub(crate) fn home_content() -> Html {
    let theme = use_theme();
    use_language();

    let is_small = use_media_query(&theme.breakpoint.sm.down());

    let scroll_to_main = use_state(|| {
        |_| {
            if let Some(m) = document().query_selector("nav").ok().flatten() {
                m.scroll_into_view_with_scroll_into_view_options(
                    ScrollIntoViewOptions::new().behavior(ScrollBehavior::Smooth),
                );
            }
        }
    });

    let down_arrow_size = if is_small { 80 } else { 100 };
    let area_size = if is_small { 120 } else { 150 };

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
            )}>
                {fl!("default-title")}
            </div>
            <div class={css!(r#"height: ${area_size}px;"#, area_size = area_size)}>
                <div
                    class={css!(r#"
                        cursor: pointer;
                        box-sizing: border-box;
                    "#)}
                    onclick={*scroll_to_main}
                >
                    <ChevronDown size={down_arrow_size} />
                </div>
            </div>
        </div>
    }
}

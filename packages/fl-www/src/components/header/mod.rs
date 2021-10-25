use crate::prelude::*;

mod content;
mod home_content;
mod item;
mod lang_toggle;
mod theme_toggle;

use components::{FlexSpace, Link};
use content::Content;
use home_content::HomeContent;
use hooks::{use_event, use_viewport_height};
use item::Item;
use lang_toggle::LangToggle;
use styling::Colour;
use styling::{use_media_query, use_style};
use theme_toggle::ThemeToggle;
use utils::get_scroll_y;

#[derive(Properties, PartialEq, Debug)]
pub(crate) struct HeaderLinksProps {
    nav_colour: Colour,
}

#[styled_component(HeaderBackgound)]
pub(crate) fn header_background() -> Html {
    let route = use_app_route();
    let theme = use_theme();

    let home_class = match route {
        AppRoute::Home { .. } => "currently-home",
        _ => "",
    };

    html! {
        <div class={classes!(
            css!(
                r#"
                    position: absolute;

                    height: 200px;
                    width: 100%;

                    &.currently-home {
                        height: 100vh;
                    }

                    @media ${md_up} {
                        height: 300px;

                        &.currently-home {
                            height: 100vh;
                        }
                    }
                "#,
                md_up = theme.breakpoint.md.up(),
            ), home_class
        )}>
            <picture>
                <source srcset="/assets/images/background.webp" type="image/webp" />
                <img src="/assets/images/background.jpg" class={css!(r#"
                    height: 100%;
                    width: 100%;

                    object-fit: cover;
                    object-position: top right;
                "#)} />
            </picture>
        </div>
    }
}

#[function_component(HeaderLinks)]
pub(crate) fn header_links(props: &HeaderLinksProps) -> Html {
    let lang = use_language();
    let theme = use_theme();
    let home_colour = theme.colour.primary.with_opacity(0.9);
    let nav_colour = props.nav_colour;

    let is_mobile = use_media_query(&theme.breakpoint.sm.down());

    let home_text = match (is_mobile, lang) {
        (false, Language::Chinese) => fl!("default-title"),
        _ => fl!("home"),
    };

    let home_route = AppRoute::Home { lang };

    html! {
        <>
            <Link to={home_route} colour={nav_colour}>
                <Item colour={home_colour}>{home_text}</Item>
            </Link>
            <Link to={AppRoute::Page { lang, slug: "about".to_string() }} colour={nav_colour}>
                <Item colour={Colour::from_rgba(255, 242, 66, 0.9)}>{fl!("about")}</Item>
            </Link>
            <Link to={AppRoute::Page { lang, slug: "links".to_string() }} colour={nav_colour}>
                <Item colour={Colour::from_rgba(230, 117, 92, 0.9)}>{fl!("friendly-links")}</Item>
            </Link>
        </>
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum NavPosition {
    Top,
    Bottom,
    InPlace,
}

#[styled_component(Header)]
pub(crate) fn header() -> Html {
    let theme = use_theme();
    let route = use_app_route();
    let viewport_height = use_viewport_height();
    // let browser = BrowserKind::detect();

    use_language();

    let header_ref: NodeRef = use_ref(NodeRef::default).borrow_mut().clone();

    let nav_pos = use_state_eq(|| NavPosition::InPlace);

    let header_is_home = use_state_eq(|| false);

    let nav_pos_clone = nav_pos.clone();
    let header_ref_clone = header_ref.clone();
    let sync_header_height = move || {
        if let Some(m) = header_ref_clone.cast::<HtmlElement>() {
            let y_pos = get_scroll_y();

            let header_height = match window()
                .get_computed_style(&m)
                .ok()
                .flatten()
                .and_then(|m| m.get_property_value("height").ok())
                // Get Header height in px.
                .and_then(|m| m.splitn(2, "px").next().and_then(|m| m.parse::<u32>().ok()))
            {
                Some(header_height) => header_height,
                None => {
                    nav_pos_clone.set(NavPosition::InPlace);
                    return;
                }
            };

            // If y position > header height - 60px,
            // then NavPosition should be top.
            let pos = if y_pos
                .map(|m| header_height > 60 && m >= header_height - 60)
                .unwrap_or(false)
            {
                NavPosition::Top
            } else if y_pos
                // If header height > viewport height,
                // and header height - viewport height > y position,
                // then NavPosition should be bottom.
                .map(|m| (header_height as i64 - viewport_height as i64, m as i64))
                .map(|(diff, y_pos)| diff > 0 && y_pos < diff)
                .unwrap_or(false)
            {
                NavPosition::Bottom
            } else {
                NavPosition::InPlace
            };

            nav_pos_clone.set(pos);
        }
    };

    let sync_header_height_clone = sync_header_height.clone();
    use_event(&window(), "scroll", move |_| sync_header_height_clone());
    let sync_header_height_clone = sync_header_height.clone();
    use_event(&window(), "orientationchange", move |_| {
        sync_header_height_clone()
    });

    use_effect_with_deps(
        move |_| {
            sync_header_height();
            || {}
        },
        *header_is_home,
    );

    let header_style = use_style!(
        r#"
            /* display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: flex-end; */
            width: 100%;
            color: white;
            height: 200px;

            /*
            background-image: url(${bg_url});
            background-repeat: no-repeat;
            background-size: cover;
            background-position: top right;
            */

            box-shadow: 0 0 10px 5px rgba(0, 0, 0, 0.3);

            &.fl-header-no-shadow {
                box-shadow: none;
            }

            &.currently-home {
                height: 100vh;
            }

            @media ${md_up} {
                height: 300px;

                &.currently-home {
                    height: 100vh;
                }
            }
        "#,
        md_up = theme.breakpoint.md.up(),
        // bg_url = bg_url,
    );

    let nav_style = use_style!(
        r#"
            height: 60px;
            width: 100%;

            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: center;

            background-color: ${bg_colour_transparent};

            transition: background-color 0.3s;

            box-sizing: border-box;

            padding-left: env(safe-area-inset-left);
            padding-right: env(safe-area-inset-right);


            /*
            position: sticky;
            position: -webkit-sticky;
            bottom: 0;
            */

            &.fl-nav-fixed-top {
                top: 0;
                position: fixed;

                background-color: ${bg_colour};

                color: ${text_colour};

                box-shadow: 0 0 10px 5px rgba(0, 0, 0, 0.3);
            }
        "#,
        bg_colour_transparent = theme.colour.background.component.with_opacity(0.0),
        bg_colour = css_var!(theme.colour.background.component),
        text_colour = css_var!(theme.colour.text.primary),
    );

    let mut header_classes = vec![header_style.get_class_name().to_owned()];
    let mut nav_classes = vec![nav_style.get_class_name().to_owned()];

    let nav_pos = *nav_pos;

    if nav_pos == NavPosition::Top {
        header_classes.push("fl-header-no-shadow".to_string());
        nav_classes.push("fl-nav-fixed-top".to_string());
    } else if nav_pos == NavPosition::Bottom {
        nav_classes.push("fl-nav-fixed-bottom".to_string());
    }

    let content = match route {
        AppRoute::Home { .. } => {
            header_is_home.set(true);
            header_classes.push("currently-home".to_string());
            html! {<HomeContent />}
        }
        _ => {
            header_is_home.set(false);
            html! {<Content />}
        }
    };

    let nav_colour = if nav_pos == NavPosition::Top {
        theme.colour.text.primary
    } else {
        Colour::from_rgb(255, 255, 255)
    };

    html! {
        <header class={classes!(header_classes)} ref={header_ref}>
            <HeaderBackgound />
            <div class={css!(
                r#"
                    background-color: ${bg_colour};
                    transition: background-color 0.3s;

                    width: 100%;
                    height: 100%;

                    position: relative;
                    z-index: 10;
                "#,
                bg_colour = css_var!(theme.colour.background.header)
            )}>
                <div class={css!(
                    r#"
                        height: 35%;
                        width: 100%;
                    "#,
                )}/>
                <div
                    class={css!(
                        r#"
                            height: 65%;
                            width: 100%;

                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            justify-content: flex-end;

                            position: sticky;
                            position: -webkit-sticky;
                            bottom: 0;
                        "#
                    )}
                >
                    {content}
                    <nav class={classes!(nav_classes)}>
                        <HeaderLinks nav_colour={nav_colour} />
                        <FlexSpace />
                        <LangToggle colour={nav_colour} />
                        <ThemeToggle />
                    </nav>
                </div>
            </div>
        </header>
    }
}

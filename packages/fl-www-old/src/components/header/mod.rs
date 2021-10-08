use crate::prelude::*;

mod content;
mod home_content;
mod item;
mod lang_toggle;
mod theme_toggle;

use components::{FlexSpace, Link};
use content::Content;
use home_content::HomeContent;
use hooks::use_event;
use i18n::Language;
use item::Item;
use lang_toggle::LangToggle;
use styling::Colour;
use styling::{use_style, ThemeKind};
use theme_toggle::ThemeToggle;
use utils::get_scroll_y;

#[styled_component(Header)]
pub(crate) fn header() -> Html {
    let theme = use_theme();
    let home_colour = theme.colour.primary.with_opacity(0.9);

    let lang = use_language();

    let header_ref: NodeRef = use_ref(|| NodeRef::default()).borrow_mut().clone();

    let background_colour = match theme.kind() {
        ThemeKind::Light => Colour::from_rgba(0, 0, 0, 0.3),
        ThemeKind::Dark => Colour::from_rgba(0, 0, 0, 0.5),
    };

    let primary_text_color = &theme.colour.text.primary;

    let fixed_nav = use_state(|| false);

    let fixed_nav_clone = fixed_nav.clone();
    let header_ref_clone = header_ref.clone();
    let sync_header_height = move || {
        if let Some(m) = header_ref_clone.cast::<HtmlElement>() {
            match window()
                .get_computed_style(&m)
                .ok()
                .flatten()
                .and_then(|m| m.get_property_value("height").ok())
                .and_then(|m| m.splitn(2, "px").next().and_then(|m| m.parse::<u32>().ok()))
            {
                Some(header_height) => {
                    let current_fixed_nav = get_scroll_y()
                        .map(|m| header_height > 60 && m >= header_height - 60)
                        .unwrap_or(false);

                    fixed_nav_clone.set(current_fixed_nav);
                }
                None => {
                    fixed_nav_clone.set(false);
                }
            }
        }
    };

    let sync_header_height_clone = sync_header_height.clone();
    use_effect(move || {
        sync_header_height_clone();
        || {}
    });

    use_event(&window(), "scroll", move |_| sync_header_height());

    let header_style = use_style!(
        r#"
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: flex-end;
            width: 100%;
            color: white;
            height: 200px;

            background-image: url(/assets/images/background.jpg);
            background-repeat: no-repeat;
            background-size: cover;
            background-position: top right;

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

            &.fl-nav-fixed {
                position: fixed;
                top: 0;

                background-color: ${bg_colour};

                color: ${text_colour};

                box-shadow: 0 0 10px 5px rgba(0, 0, 0, 0.3);
            }
        "#,
        bg_colour_transparent = theme.colour.background.component.with_opacity(0.0),
        bg_colour = theme.colour.background.component,
        text_colour = primary_text_color,
    );

    let mut header_classes = vec![header_style.get_class_name().to_owned()];
    let mut nav_classes = vec![nav_style.get_class_name().to_owned()];

    if *fixed_nav {
        header_classes.push("fl-header-no-shadow".to_string());
        nav_classes.push("fl-nav-fixed".to_string());
    }

    let content = if let Some(AppRoute::Home { .. }) = AppRoute::current_route() {
        header_classes.push("currently-home".to_string());
        html! {<HomeContent />}
    } else {
        html! {<Content />}
    };

    let home_text = match lang {
        Language::Chinese => fl!("default-title"),
        Language::English => fl!("home"),
    };

    let nav_colour = if *fixed_nav {
        theme.colour.text.primary.clone()
    } else {
        Colour::from_rgb(255, 255, 255)
    };

    html! {
        <header class={classes!(header_classes)}>
            <div
                class={css!(
                    r#"
                    background-color: ${bg_colour};
                    transition: background-color 0.3s;

                    height: 100%;
                    width: 100%;

                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: flex-end;
                    "#,
                    bg_colour = background_colour
                )}
                ref={header_ref}
            >
                {content}
                <nav class={classes!(nav_classes)}>
                    <Link to={AppRoute::Home { lang }} colour={nav_colour.clone()}>
                        <Item colour={home_colour}>{home_text}</Item>
                    </Link>
                    <Link to={AppRoute::About { lang }} colour={nav_colour.clone()}>
                        <Item colour={Colour::from_rgba(255, 242, 66, 0.9)}>{fl!("about")}</Item>
                    </Link>
                    <FlexSpace />
                    <LangToggle colour={nav_colour} />
                    <ThemeToggle />
                </nav>
            </div>
        </header>
    }
}

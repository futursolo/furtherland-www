use gloo::events::EventListener;

use crate::prelude::*;

mod content;
mod home_content;
mod item;
mod lang_toggle;
mod theme_toggle;

use components::{FlexSpace, Link};
use content::Content;
use home_content::HomeContent;
use i18n::Language;
use item::Item;
use lang_toggle::LangToggle;
use store::AppDispatch;
use styling::Colour;
use styling::ThemeKind;
use theme_toggle::ThemeToggle;
use utils::get_scroll_y;

#[derive(PartialEq, Debug)]
pub(crate) enum HeaderMsg {
    SyncHeaderHeight,
}

pub(crate) struct BaseHeader {
    link: ComponentLink<Self>,
    dispatch: AppDispatch,
    header_ref: NodeRef,

    _scroll_listener: EventListener,

    fixed_nav: bool,
}

impl Component for BaseHeader {
    type Message = HeaderMsg;
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, link: ComponentLink<Self>) -> Self {
        let link_clone = link.clone();

        let scroll_listener = EventListener::new(&window(), "scroll", move |_e| {
            link_clone.clone().send_message(HeaderMsg::SyncHeaderHeight);
        });

        Self {
            link,
            dispatch,
            header_ref: NodeRef::default(),

            _scroll_listener: scroll_listener,

            fixed_nav: false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(HeaderMsg::SyncHeaderHeight);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if HeaderMsg::SyncHeaderHeight == msg {
            if let Some(m) = self.header_ref.cast::<HtmlElement>() {
                match window()
                    .get_computed_style(&m)
                    .ok()
                    .flatten()
                    .and_then(|m| m.get_property_value("height").ok())
                    .and_then(|m| m.splitn(2, "px").next().and_then(|m| m.parse::<u32>().ok()))
                {
                    Some(header_height) => {
                        let fixed_nav = get_scroll_y()
                            .map(|m| header_height > 60 && m >= header_height - 60)
                            .unwrap_or(false);

                        let changed = fixed_nav != self.fixed_nav;
                        self.fixed_nav = fixed_nav;
                        return changed;
                    }
                    None => {
                        let changed = self.fixed_nav;
                        self.fixed_nav = false;
                        return changed;
                    }
                }
            }
        }

        true
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        let theme = self.dispatch.state().theme.current();
        let home_colour = theme.colour.primary.with_opacity(0.9);

        let lang = self.dispatch.state().i18n.lang.clone();

        let header_ref = self.header_ref.clone();

        let mut classes = vec![self.style_class()];

        let nav_cls = if self.fixed_nav {
            classes.push("fl-header-no-shadow".to_string());
            Some("fl-nav-fixed")
        } else {
            None
        };

        let content = if I18nRoute::current_route()
            .map(|m| m.into_app_route())
            .unwrap_or(AppRoute::Home)
            == AppRoute::Home
        {
            classes.push("currently-home".to_string());
            html! {<HomeContent />}
        } else {
            html! {<Content />}
        };

        let home_text = match lang {
            Language::Chinese => fl!("default-title"),
            Language::English => fl!("home"),
        };

        let nav_colour = if self.fixed_nav {
            theme.colour.text.primary.clone()
        } else {
            Colour::from_rgb(255, 255, 255)
        };

        html! {
            <header class=classes!(classes)>
                <div class="fl-header-container" ref=header_ref>
                    {content}
                    <nav class=nav_cls>
                        <Link to=lang.route_i18n(AppRoute::Home) colour=nav_colour.clone()>
                            <Item colour=home_colour>{home_text}</Item>
                        </Link>
                        <Link to=lang.route_i18n(AppRoute::About) colour=nav_colour.clone()>
                            <Item colour=Colour::from_rgba(255, 242, 66, 0.9)>{fl!("about")}</Item>
                        </Link>
                        <FlexSpace />
                        <LangToggle colour=nav_colour />
                        <ThemeToggle />
                    </nav>
                </div>
            </header>
        }
    }
}

impl YieldStyle for BaseHeader {
    fn style_str(&self) -> Cow<'static, str> {
        let theme = self.dispatch.state().theme.current();
        let theme_kind = self.dispatch.state().theme.current_kind();

        let background_colour = match theme_kind {
            ThemeKind::Light => Colour::from_rgba(0, 0, 0, 0.3),
            ThemeKind::Dark => Colour::from_rgba(0, 0, 0, 0.5),
        };

        let primary_text_color = &theme.colour.text.primary;

        format!(
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

                &.fl-header-no-shadow {{
                    box-shadow: none;
                }}

                .fl-header-container {{
                    background-color: {};
                    transition: background-color 0.3s;

                    height: 100%;
                    width: 100%;

                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: flex-end;
                }}

                nav {{
                    height: 60px;
                    width: 100%;

                    display: flex;
                    flex-direction: row;
                    align-items: center;
                    justify-content: center;

                    background-color: {};

                    transition: background-color 0.3s;
                }}

                .fl-nav-fixed {{
                    position: fixed;
                    top: 0;

                    background-color: {};

                    color: {};

                    box-shadow: 0 0 10px 5px rgba(0, 0, 0, 0.3);
                }}

                &.currently-home {{
                    height: 100vh;
                }}

                {} {{
                    height: 300px;

                    &.currently-home {{
                        height: 100vh;
                    }}
                }}
            "#,
            background_colour,
            theme.colour.background.component.with_opacity(0.0),
            theme.colour.background.component,
            primary_text_color,
            theme.breakpoint.md.up(),
        )
        .into()
    }
}

pub(crate) type Header = WithDispatch<BaseHeader>;

use yew_feather::moon::Moon;
use yew_feather::sun::Sun;

use crate::prelude::*;
use store::{Action, AppDispatch};
use styling::ThemeKind;

pub(crate) struct BaseThemeToggle {
    dispatch: AppDispatch,
}

impl Component for BaseThemeToggle {
    type Message = ();
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { dispatch }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        let theme_kind = self.dispatch.state().theme.current_kind();

        let theme_icon = match &theme_kind {
            ThemeKind::Light => html! {<Moon size=24 />},
            ThemeKind::Dark => html! {<Sun size=24 />},
        };

        let alt_text = match &theme_kind {
            ThemeKind::Light => "Switch to Dark Theme",
            ThemeKind::Dark => "Switch to Light Theme",
        };

        let toggle_theme = self
            .dispatch
            .callback(move |_| Action::SetThemeKind(Some(theme_kind.alternative())));

        html! {
            <div class=self.style() onclick=toggle_theme alt=alt_text>
                {theme_icon}
            </div>
        }
    }
}

impl YieldStyle for BaseThemeToggle {
    fn style_str(&self) -> Cow<'static, str> {
        r#"
            height: 60px;
            width: 60px;

            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: center;
            cursor: pointer;
        "#
        .into()
    }
}

pub(crate) type ThemeToggle = WithDispatch<BaseThemeToggle>;

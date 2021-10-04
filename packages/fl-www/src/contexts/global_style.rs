use crate::prelude::*;

use super::ContextProps;
use stylist::yew::Global;

pub(crate) struct BaseGlobalStyle {
    props: ContextProps,
}

impl Component for BaseGlobalStyle {
    type Message = ();
    type Properties = ContextProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let children = self.props.children.clone();
        let theme = self.props.dispatch.state().theme.current();

        let style_str = format!(
            r#"

            html, body {{
                min-height: 100vh;
                margin: 0;
                background-color: {background_color};
                color: {font_color};
                font-family: {font_family};
                font-size: {font_size};
                transition: background-color 0.3s, color 0.3s;
                -webkit-font-smoothing: antialiased;
                -moz-osx-font-smoothing: grayscale;
            }}
        "#,
            background_color = theme.colour.background.default,
            font_color = theme.colour.text.primary,
            font_family = theme.font_family,
            font_size = theme.font_size.root,
        );
        html! {<><Global css=style_str />{children}</>}
    }
}

pub(crate) type GlobalStyle = WithDispatch<BaseGlobalStyle>;

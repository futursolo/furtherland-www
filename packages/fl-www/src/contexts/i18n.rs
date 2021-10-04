use crate::prelude::*;

use super::ContextProps;

pub(crate) struct BaseI18nProvider {
    props: ContextProps,
}

impl Component for BaseI18nProvider {
    type Message = ();
    type Properties = ContextProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.props.neq_assign(props);

        self.update_html_lang();

        changed
    }

    fn view(&self) -> Html {
        let children = self.props.children.clone();
        html! {<>{children}</>}
    }
}

impl BaseI18nProvider {
    fn update_html_lang(&mut self) {
        let lang = self.props.dispatch.state().i18n.lang.clone();

        let html_element = document()
            .document_element()
            .expect("Failed to get <html /> element.");

        html_element
            .set_attribute("lang", lang.as_str())
            .expect("Failed to set language.");
    }
}

pub(crate) type I18nProvider = WithDispatch<BaseI18nProvider>;

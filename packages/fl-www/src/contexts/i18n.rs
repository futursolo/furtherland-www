use crate::prelude::*;
use i18n::Language;
use utils::Id;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LanguageState {
    lang: Language,
    id: Id,
}

pub(crate) fn use_language() -> Language {
    use_context::<LanguageState>()
        .map(|m| m.lang)
        .unwrap_or_else(Language::detect)
}

#[function_component(I18nProvider)]
pub(crate) fn i18n_provider(props: &ChildrenProps) -> Html {
    let children = props.children.clone();
    use_app_route(); // Refresh when route changes.

    let lang = Language::detect();

    let id = use_state(Id::new);
    let id_clone = id.clone();
    use_effect_with_deps(
        move |lang| {
            let html_element = document()
                .document_element()
                .expect("Failed to get <html /> element.");

            html_element
                .set_attribute("lang", lang.as_str())
                .expect("Failed to set language.");

            lang.activate();
            id_clone.set(Id::new());

            || {}
        },
        lang,
    );

    let state = LanguageState {
        lang,
        id: (*id).clone(),
    };

    html! {<ContextProvider<LanguageState> context={state}>{children}</ContextProvider<LanguageState>>}
}

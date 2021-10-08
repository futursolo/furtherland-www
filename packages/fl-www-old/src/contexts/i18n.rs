use crate::prelude::*;

use hooks::use_event;
use i18n::Language;

pub(crate) fn use_language() -> Language {
    use_context::<Language>().unwrap_or_else(Language::detect)
}

#[function_component(I18nProvider)]
pub(crate) fn i18n_provider(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    let lang = use_state(Language::detect);

    let lang_clone = lang.clone();
    use_event(&window(), "popstate", move |_event| {
        lang_clone.set(Language::detect());
    });

    use_effect_with_deps(
        |lang| {
            let html_element = document()
                .document_element()
                .expect("Failed to get <html /> element.");

            html_element
                .set_attribute("lang", lang.as_str())
                .expect("Failed to set language.");

            lang.activate();

            || {}
        },
        lang.clone(),
    );

    html! {<ContextProvider<Language> context={*lang}>{children}</ContextProvider<Language>>}
}

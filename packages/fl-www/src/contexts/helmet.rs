use std::cell::RefCell;
use std::rc::Rc;

use crate::prelude::*;
use utils::is_ssr;

use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlLinkElement, HtmlMetaElement, HtmlScriptElement};

use yew_side_effect::{SideEffect, SideEffectProvider, SideEffects};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Tag {
    Meta(MetaProps),
    Link(MetaLinkProps),
    Script(ScriptProps),
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub(crate) struct MetaProps {
    pub name: String,
    pub content: String,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub(crate) struct MetaLinkProps {
    pub rel: String,
    pub type_: String,
    pub href: String,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub(crate) struct ScriptProps {
    pub content: String,
    pub type_: String,
}

#[function_component(Meta)]
pub(crate) fn meta(props: &MetaProps) -> Html {
    let value = Rc::new(Tag::Meta(props.to_owned()));

    html! {<SideEffect<Tag> value={value} />}
}

#[function_component(MetaLink)]
pub(crate) fn meta_link(props: &MetaLinkProps) -> Html {
    let value = Rc::new(Tag::Link(props.to_owned()));

    html! {<SideEffect<Tag> value={value} />}
}

#[function_component(Script)]
pub(crate) fn script(props: &ScriptProps) -> Html {
    let value = Rc::new(Tag::Script(props.to_owned()));

    html! {<SideEffect<Tag> value={value} />}
}

pub(crate) fn create_element<T>(tag_name: &str) -> T
where
    T: AsRef<Element> + JsCast,
{
    let element = document().create_element(tag_name).unwrap_throw();

    if is_ssr() {
        element.set_attribute("data-helmet", "prerendered").unwrap();
    }

    JsValue::from(&element).dyn_into::<T>().unwrap_throw()
}

#[function_component(HelmetProvider)]
pub(crate) fn helmet_provider(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    let elements = use_state(|| -> RefCell<Vec<Element>> { RefCell::new(Vec::new()) });

    use_effect_with_deps(
        |_| {
            if !is_ssr() {
                if let Some(m) = document().head() {
                    let m: &Element = m.as_ref();

                    if let Ok(nodes) = m.query_selector_all("[data-helmet='prerendered']") {
                        for i in 0..nodes.length() {
                            if let Some(node) = nodes.get(i) {
                                if let Some(m) = node.parent_node() {
                                    m.remove_child(&node).unwrap();
                                }
                            }
                        }
                    }
                }
            }

            || {}
        },
        (),
    );

    let sync = use_state(move || {
        Rc::new(move |tags: SideEffects<Tag>| {
            let mut elements = elements.borrow_mut();

            for element in elements.iter() {
                if let Some(m) = element.parent_element() {
                    m.remove_child(element).unwrap();
                }
            }

            let mut new_elements: Vec<Element> = Vec::new();

            for tag in tags.iter() {
                match **tag {
                    Tag::Meta(ref meta) => {
                        let element = create_element::<HtmlMetaElement>("meta");

                        element.set_name(&meta.name);
                        element.set_content(&meta.content);

                        let element: &Element = element.as_ref();

                        document().head().unwrap().append_child(element).unwrap();
                        new_elements.push(element.to_owned());
                    }
                    Tag::Link(ref link) => {
                        let element = create_element::<HtmlLinkElement>("link");

                        element.set_href(&link.href);
                        element.set_type(&link.type_);
                        element.set_rel(&link.rel);

                        let element: &Element = element.as_ref();

                        document().head().unwrap().append_child(element).unwrap();
                        new_elements.push(element.to_owned());
                    }
                    Tag::Script(ref script) => {
                        let element = create_element::<HtmlScriptElement>("script");

                        element
                            .set_text(&script.content)
                            .expect_throw("failed to set script content");
                        element.set_type(&script.type_);

                        let element: &Element = element.as_ref();

                        document().head().unwrap().append_child(element).unwrap();
                        new_elements.push(element.to_owned());
                    }
                }
            }

            *elements = new_elements;
        }) as Rc<dyn Fn(SideEffects<Tag>)>
    });

    html! {
        <SideEffectProvider<Tag> on_change={(*sync).clone()}>
            {children}
        </SideEffectProvider<Tag>>
    }
}

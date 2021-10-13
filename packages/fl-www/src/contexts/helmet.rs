use std::cell::RefCell;
use std::rc::Rc;

use crate::prelude::*;

use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlLinkElement, HtmlMetaElement};

use yew_side_effect::{SideEffect, SideEffectProvider, SideEffects};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Tag {
    Meta(MetaProps),
    Link(MetaLinkProps),
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

#[function_component(HelmetProvider)]
pub(crate) fn meta_provider(props: &ChildrenProps) -> Html {
    let children = props.children.clone();

    let elements = use_state(|| -> RefCell<Vec<Element>> { RefCell::new(Vec::new()) });

    let sync = use_state(move || {
        Rc::new(move |tags: SideEffects<Tag>| {
            let mut elements = elements.borrow_mut();

            for element in elements.iter() {
                if let Some(m) = element.parent_element() {
                    m.remove_child(element).unwrap();
                }
            }

            let mut new_elements = Vec::new();

            for tag in tags.iter() {
                match **tag {
                    Tag::Meta(ref meta) => {
                        let element = document()
                            .create_element("meta")
                            .and_then(|m| JsValue::from(&m).dyn_into::<HtmlMetaElement>())
                            .unwrap();

                        element.set_name(&meta.name);
                        element.set_content(&meta.content);

                        let element: &Element = element.as_ref();

                        document().head().unwrap().append_child(element).unwrap();

                        new_elements.push(element.to_owned());
                    }
                    Tag::Link(ref link) => {
                        let element = document()
                            .create_element("link")
                            .and_then(|m| JsValue::from(&m).dyn_into::<HtmlLinkElement>())
                            .unwrap();

                        element.set_href(&link.href);
                        element.set_type(&link.type_);
                        element.set_rel(&link.rel);

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

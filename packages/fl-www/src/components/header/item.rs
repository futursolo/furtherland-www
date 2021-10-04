use std::borrow::Cow;

use crate::prelude::*;
use store::AppDispatch;
use styling::Colour;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct ItemProps {
    #[prop_or_default]
    pub dispatch: AppDispatch,
    pub children: Children,

    pub colour: Colour,
}

impl DispatchPropsMut for ItemProps {
    type Store = store::Store;

    fn dispatch(&mut self) -> &mut AppDispatch {
        &mut self.dispatch
    }
}

pub(crate) struct BaseItem {
    props: ItemProps,
}

impl Component for BaseItem {
    type Message = ();
    type Properties = ItemProps;

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
        html! {
            <div class=self.style()>
                <div class="fl-header-item-text">{self.props.children.clone()}</div>
                <div class="fl-header-item-indicator" />
            </div>
        }
    }
}

impl YieldStyle for BaseItem {
    fn style_str(&self) -> Cow<'static, str> {
        format!(
            r#"
                height: 60px;
                font-size: 1.1rem;
                font-weight: bold;
                padding-left: 15px;
                padding-right: 15px;

                transition: color 0.3s;

                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;

                & .fl-header-item-text {{
                    flex-grow: 1;
                    line-height: 57px;
                }}

                & .fl-header-item-indicator {{
                    height: 3px;
                    width: 0%;
                    transition: width 0.2s ease-out;
                    background-color: {};
                }}

                &:hover .fl-header-item-indicator {{
                    width: 100%;
                }}
            "#,
            self.props.colour
        )
        .into()
    }
}

pub(crate) type Item = WithDispatch<BaseItem>;

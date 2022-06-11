use crate::prelude::*;
use yew::events::InputEvent;
use yew::virtual_dom::AttrValue;

#[derive(Properties, PartialEq, Clone)]
pub struct TextareaProps {
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub oninput: Option<Callback<InputEvent>>,
    #[prop_or_default]
    pub class: Classes,
}

#[styled_component(Textarea)]
pub fn textarea(props: &TextareaProps) -> HtmlResult {
    let TextareaProps {
        value,
        oninput,
        class,
    } = props.clone();

    let theme = use_theme();

    return html! {
        <div
            class={classes!(
                class,
                css!(
                    r#"
                        display: flex;
                        flex-direction: column;
                        align-items: stretch;
                        background-color: ${bg_colour};
                        border-radius: 8px;
                        padding: 15px;
                        box-sizing: border-box;
                        transition: 0.3s background-color;
                    "#,
                    bg_colour=css_var!(theme.colour.background.component),
                )
            )}
        >
            <textarea
                value={value}
                oninput={oninput}

                class={css!(
                    r#"
                        flex-grow: 1;
                        height: 1px;
                        width: 100%;
                        background-color: ${bg_colour};
                        box-sizing: border-box;
                        color: ${colour};
                        border: none;
                        outline: 0;
                        font-size: 1rem;
                        resize: none;
                        transition: 0.3s background-color, 0.3s color;
                    "#,
                    bg_colour=css_var!(theme.colour.background.component),
                    colour=css_var!(theme.colour.text.primary),
                )}
            />
        </div>
    };
}

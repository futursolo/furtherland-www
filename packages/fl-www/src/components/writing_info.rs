use chrono::NaiveDate;

use crate::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct WritingInfoProps {
    #[prop_or_default]
    pub dispatch: AppDispatch,
    pub date: NaiveDate,
}

impl_dispatch_mut!(WritingInfoProps);

pub(crate) struct BaseWritingInfo {
    props: WritingInfoProps,
}

impl Component for BaseWritingInfo {
    type Message = ();
    type Properties = WritingInfoProps;

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
        let date_str = self.props.date.format("%Y-%m-%d");

        html! {
            <div class=self.style()>
                <div class="fl-writing-info-avatar"></div>
                <div class="fl-writing-info-right">
                    <div class="fl-writing-info-author">{fl!("my-name")}</div>
                    <div class="fl-writing-info-date">{date_str}</div>
                </div>
            </div>
        }
    }
}

impl YieldStyle for BaseWritingInfo {
    fn style_str(&self) -> Cow<'static, str> {
        let theme = self.props.dispatch.state().theme.current();

        format!(
            r#"
                display: flex;
                width: 100%;
                padding-bottom: 10px;

                flex-direction: row;
                align-items: center;
                justify-content: flex-start;

                & .fl-writing-info-avatar {{
                    background-image: url(https://www.gravatar.com/avatar/0dd494a963ae648caebe34288b664ca6?s=200&d=mp);
                    height: 50px;
                    width: 50px;
                    border-radius: 50px;
                    background-repeat: no-repeat;
                    background-size: cover;
                }}

                & .fl-writing-info-right {{
                    display: flex;
                    flex-direction: column;
                    justify-content: space-around;
                    height: 50px;

                    padding-left: 10px;
                    padding-right: 10px;
                }}

                & .fl-writing-info-author {{
                    font-size: {};
                    color: {};
                }}

                & .fl-writing-info-date {{
                    font-size: {};
                    color: {};
                }}
            "#,
            theme.font_size.default,
            theme.colour.text.primary,
            theme.font_size.secondary,
            theme.colour.text.secondary,
        )
        .into()
    }
}

pub(crate) type WritingInfo = WithDispatch<BaseWritingInfo>;

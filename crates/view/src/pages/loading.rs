use components::{Main, Placeholder, PlaceholderKind};

use crate::prelude::*;

#[styled_component(Loading)]
pub(crate) fn loading() -> Html {
    use_language();
    let theme = use_theme();

    html! {
        <>
            <Main>
                <div class={css!("
                    margin-top: 0.67em;
                    margin-bottom: 0.67em;
                    font-size: 3rem;
                ")}>
                    <Placeholder height="3rem" width="100%" />
                </div>
                <div class={css!(
                    r#"
                        display: flex;
                        width: 100%;
                        padding-bottom: 20px;

                        flex-direction: row;
                        align-items: center;
                        justify-content: flex-start;
                    "#,
                )}>
                    <Placeholder height="50px" width="50px" kind={PlaceholderKind::Circle} />
                    <div class={css!(
                        r#"
                            display: flex;
                            flex-direction: column;
                            justify-content: space-around;
                            height: 50px;

                            padding-left: 10px;
                            padding-right: 10px;
                        "#,
                    )}>
                        <Placeholder
                            height={theme.font_size.default.to_string()}
                            width="200px"
                        />
                        <Placeholder
                            height={theme.font_size.secondary.to_string()}
                            width="130px"
                        />
                    </div>
                </div>
                <div class={css!("
                    margin-bottom: 10px;
                ")}>
                    <Placeholder height="1rem" width="100%" />
                </div>
                <div class={css!("
                    margin-bottom: 10px;
                ")}>
                    <Placeholder height="1rem" width="100%" />
                </div>
                <div class={css!("
                    margin-bottom: 30px;
                ")}>
                    <Placeholder height="1rem" width="75%" />
                </div>

                <div class={css!("
                    margin-bottom: 10px;
                ")}>
                    <Placeholder height="10rem" width="100%" />
                </div>
            </Main>
        </>
    }
}

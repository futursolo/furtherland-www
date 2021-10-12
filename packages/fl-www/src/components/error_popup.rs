use yew_feather::alert_circle::AlertCircle;

use crate::prelude::*;

#[styled_component(ErrorPopup)]
pub(crate) fn error_popup() -> Html {
    let error = use_error_state();
    use_language();
    let theme = use_theme();

    let error_message = match error.kind() {
        None => return Html::default(),
        Some(ErrorKind::Server) => {
            fl!("error-message-server")
        } // Some(ErrorKind::Unknown) => {
          //     fl!("error-message-unknown")
          // }
    };

    html! {
        <div class={css!(r#"
            width: 100%;
            height: 50px;

            position: fixed;
            bottom: 20px;
            z-index: 1000;

            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            color: white;
        "#)}>
            <div class={css!(
                r#"
                    background-color: ${invalid_bg};
                    border-radius: 5px;
                    padding: 10px;
                    padding-left: 15px;
                    padding-right: 15px;

                    display: flex;
                    flex-direction: row;
                    align-items: center;
                    justify-content: center;
                "#,
                invalid_bg = theme.colour.invalid,
            )}>
                <AlertCircle size={20} />
                <div class={css!(r#"width: 7px;"#)} />
                <div>{error_message}</div>
            </div>
        </div>
    }
}
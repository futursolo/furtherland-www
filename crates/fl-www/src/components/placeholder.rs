use once_cell::sync::Lazy;
use styling::ThemeKind;
use utils::Id;

use crate::prelude::*;

static KEYFRAME_ID: Lazy<Id> = Lazy::new(Id::new);

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum PlaceholderKind {
    Rect,
    Circle,
}

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct Props {
    pub height: String,
    pub width: String,
    #[prop_or(PlaceholderKind::Rect)]
    pub kind: PlaceholderKind,

    #[prop_or(true)]
    pub set_data_status: bool,
}

#[styled_component(Placeholder)]
pub(crate) fn placeholder(props: &Props) -> Html {
    let theme = use_theme();

    let radius = match props.kind {
        PlaceholderKind::Rect => "2px",
        PlaceholderKind::Circle => "50%",
    };

    let wave_colour = match theme.kind() {
        ThemeKind::Light => "rgb(255, 255, 255, 0.7)",
        ThemeKind::Dark => "rgb(255, 255, 255, 0.05)",
    };

    let data_status = props.set_data_status.then(|| "loading");

    html! {
        <div data-status={data_status} class={css!(
            r#"
                height: ${height};
                width: ${width};
                border-radius: ${radius};
                background-color: ${bg_color};
                overflow-x: hidden;
                overflow-y: hidden;
                transition: 0.3s background-color;

                -webkit-mask-image: -webkit-radial-gradient(center, white, black);
            "#,
            height = &props.height,
            width = &props.width,
            radius = radius,
            bg_color = css_var!(theme.colour.background.code),
        )}>
            <div class={css!(
                r#"

                    width: 100%;
                    height: 100%;

                    animation-name: fl-keyframe-${keyframe_id};
                    animation-delay: 0.5s;
                    animation-duration: 1.6s;
                    animation-iteration-count: infinite;
                    animation-timing-during: linear;
                    transform: translateX(-100px);


                    @keyframes fl-keyframe-${keyframe_id} {
                        from {
                            transform: translateX(-100px);
                        }

                        to {
                            transform: translateX(100%);
                        }
                    }
                "#,
                keyframe_id = (*KEYFRAME_ID).to_u64(),
            )}>
                <div class={css!(r#"
                    background-image: linear-gradient(to right, rgba(255, 255, 255, 0), ${wave_colour}, rgba(255, 255, 255, 0));
                    width: 100px;
                    height: 100%;

                    transition: 0.3s background-image;
                "#, wave_colour = wave_colour)} />
            </div>
        </div>
    }
}

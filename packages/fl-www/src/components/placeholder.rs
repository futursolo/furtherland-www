use crate::prelude::*;
use styling::ThemeKind;
use utils::Id;

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
}

#[styled_component(Placeholder)]
pub(crate) fn placeholder(props: &Props) -> Html {
    let theme = use_theme();

    let radius = match props.kind {
        PlaceholderKind::Rect => "2px",
        PlaceholderKind::Circle => "50%",
    };

    let keyframe_id = use_state(Id::new);
    let wave_colour = match theme.kind() {
        ThemeKind::Light => "rgb(255, 255, 255, 0.7)",
        ThemeKind::Dark => "rgb(255, 255, 255, 0.05)",
    };

    html! {
        <div class={css!(
            r#"
                height: ${height};
                width: ${width};
                border-radius: ${radius};
                background-color: ${bg_color};
                overflow-x: hidden;
                overflow-y: hidden;

                -webkit-mask-image: -webkit-radial-gradient(center, white, black);
            "#,
            height = &props.height,
            width = &props.width,
            radius = radius,
            bg_color = theme.colour.background.code,
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


                    @keyframes fl-keyframe-${keyframe_id} {
                        from {
                            transform: translateX(-100px);
                        }

                        to {
                            transform: translateX(100%);
                        }
                    }
                "#,
                keyframe_id = (*keyframe_id).to_u64(),
            )}>
                <div class={css!(r#"
                    background-image: linear-gradient(to right, rgba(255, 255, 255, 0), ${wave_colour}, rgba(255, 255, 255, 0));
                    width: 100px;
                    height: 100%;
                "#, wave_colour = wave_colour)} />
            </div>
        </div>
    }
}

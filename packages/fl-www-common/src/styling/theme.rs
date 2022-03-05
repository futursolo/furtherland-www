use crate::prelude::*;
use once_cell::sync::Lazy;

use styling::{Colour, ThemeKind};

#[derive(Debug, Clone, PartialEq, CssVariables)]
pub struct FontSizes {
    pub root: String,
    pub default: String,
    pub secondary: String,
    pub hint: String,
}

impl Default for FontSizes {
    fn default() -> Self {
        Self {
            root: "14px".to_string(),
            default: "1rem".to_string(),
            secondary: "0.9rem".to_string(),
            hint: "0.8rem".to_string(),
        }
    }
}
// css_var!(theme.colour.primary) -> var(--stylist-[entropy]-theme-colour-primary, #123456)
//
// #[derive(CssVariables)]
// struct Theme {
//     #[css_vars(nested)]
//     colour: Colours,
// }
//
// trait CssVariables {
//     fn entropy() -> &'static str;
//     fn to_css_vars_nested_with_prefix()
//     fn to_css_vars_for(selector: &str) -> StyleSource<'static>;
// }
//
// <Global style={theme.to_css_vars_for("html")} />
#[derive(Debug, Clone, PartialEq)]
pub struct Breakpoint {
    width: u64,
}

impl Breakpoint {
    fn new(width: u64) -> Self {
        Self { width }
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn width_str(&self) -> String {
        format!("{}px", self.width())
    }

    pub fn down(&self) -> String {
        format!("(max-width: {}px)", self.width)
    }

    pub fn up(&self) -> String {
        format!("(min-width: {}px)", self.width)
    }

    pub fn matches_down(&self) -> bool {
        self.width() > Breakpoints::current_width()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Breakpoints {
    pub lg: Breakpoint,
    pub md: Breakpoint,
    pub sm: Breakpoint,
}

impl Breakpoints {
    /// Returns current width of the screen.
    fn current_width() -> u64 {
        window()
            .outer_width()
            .ok()
            .and_then(|m| m.as_f64())
            .map(|m| m as u64)
            .expect("Failed to get screen width.")
    }
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self {
            lg: Breakpoint::new(1280),
            md: Breakpoint::new(960),
            sm: Breakpoint::new(500),
        }
    }
}

#[derive(Debug, Clone, PartialEq, CssVariables)]
pub struct Backgrounds {
    pub invalid: Colour,
    pub component: Colour,
    pub default: Colour,
    pub component_shadow: Colour,

    pub code: Colour,

    pub header: Colour,
}

#[derive(Debug, Clone, PartialEq, CssVariables)]
pub struct TextColours {
    pub primary: Colour,
    pub secondary: Colour,
    pub hint: Colour,
}

#[derive(Debug, Clone, PartialEq, CssVariables)]
pub struct Colours {
    pub primary: Colour,
    pub primary_hover: Colour,

    pub secondary: Colour,
    pub secondary_hover: Colour,

    pub invalid: Colour,

    #[css_vars(nested)]
    pub background: Backgrounds,
    #[css_vars(nested)]
    pub text: TextColours,
}

#[derive(Debug, Clone, PartialEq, CssVariables)]
pub struct Theme {
    pub font_family: String,

    #[css_vars(nested)]
    pub font_size: FontSizes,

    #[css_vars(nested)]
    pub colour: Colours,

    #[css_vars(skipped)]
    pub breakpoint: Breakpoints,
}

impl Theme {
    pub fn light() -> &'static Self {
        static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', \
                'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif"
                .to_string(),
            font_size: FontSizes::default(),

            colour: Colours {
                primary: Colour::from_rgb(92, 184, 230),
                primary_hover: Colour::from_rgb(125, 198, 235),

                secondary: Colour::from_rgb(244, 245, 249),
                secondary_hover: Colour::from_rgb(221, 224, 238),

                invalid: Colour::from_rgb(238, 82, 26),

                background: Backgrounds {
                    invalid: Colour::from_rgb(254, 237, 234),
                    default: Colour::from_rgb(255, 255, 255),
                    component: Colour::from_rgb(244, 245, 249),
                    component_shadow: Colour::from_rgb(150, 150, 150),

                    code: Colour::from_rgb(246, 248, 255),
                    header: Colour::from_rgba(0, 0, 0, 0.3),
                },
                text: TextColours {
                    primary: Colour::from_rgb(0, 0, 0),
                    secondary: Colour::from_rgb(100, 100, 100),
                    hint: Colour::from_rgb(150, 150, 150),
                },
            },
            breakpoint: Breakpoints::default(),
        });

        &LIGHT_THEME
    }

    pub fn dark() -> &'static Self {
        static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', \
                'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif"
                .to_string(),
            font_size: FontSizes::default(),

            colour: Colours {
                primary: Colour::from_rgb(92, 184, 230),
                primary_hover: Colour::from_rgb(125, 198, 235),

                secondary: Colour::from_rgb(50, 50, 50),
                secondary_hover: Colour::from_rgb(75, 75, 75),

                invalid: Colour::from_rgb(238, 82, 26),

                background: Backgrounds {
                    invalid: Colour::from_rgb(73, 38, 32),
                    component: Colour::from_rgb(50, 50, 50),
                    default: Colour::from_rgb(20, 20, 20),
                    component_shadow: Colour::from_rgb(35, 35, 35),

                    code: Colour::from_rgb(41, 48, 66),
                    header: Colour::from_rgba(0, 0, 0, 0.5),
                },
                text: TextColours {
                    primary: Colour::from_rgb(255, 255, 255),
                    secondary: Colour::from_rgb(150, 150, 150),
                    hint: Colour::from_rgb(100, 100, 100),
                },
            },
            breakpoint: Breakpoints::default(),
        });

        &DARK_THEME
    }

    pub fn from_kind(kind: &ThemeKind) -> &'static Self {
        match kind {
            ThemeKind::Dark => Self::dark(),
            ThemeKind::Light => Self::light(),
        }
    }
}

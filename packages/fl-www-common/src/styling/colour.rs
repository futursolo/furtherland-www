use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Colour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: Option<f64>,
}

impl Colour {
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: None,
        }
    }

    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: f64) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: Some(alpha),
        }
    }

    pub fn with_opacity(&self, alpha: f64) -> Self {
        Self {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: Some(alpha),
        }
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.alpha {
            Some(m) => write!(
                f,
                "rgba({}, {}, {}, {:.3})",
                self.red, self.green, self.blue, m
            ),
            None => write!(f, "rgb({}, {}, {})", self.red, self.green, self.blue),
        }
    }
}

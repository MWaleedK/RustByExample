use super::super::AnnotationType;
use crossterm::style::Color;

pub struct Attribute {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl From<AnnotationType> for Attribute {
    fn from(annotation_type: AnnotationType) -> Self {
        match annotation_type {
            AnnotationType::Match => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 211,
                    g: 211,
                    b: 211,
                }),
            },
            AnnotationType::SelectedMatch => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 153,
                }),
            },
            AnnotationType::Number => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 95,
                    b: 71,
                }),
                background: None,
            },
            AnnotationType::Keyword => Self {
                foreground: Some(Color::Rgb {
                    r: 100,
                    g: 149,
                    b: 237,
                }),
                background: None,
            },
            AnnotationType::Type => Self {
                foreground: Some(Color::Rgb {
                    r: 175,
                    g: 255,
                    b: 175,
                }),
                background: None,
            },
            AnnotationType::KnownValue => Self {
                foreground: Some(Color::Rgb {
                    r: 195,
                    g: 177,
                    b: 255,
                }),
                background: None,
            },
            AnnotationType::Char => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 191,
                    b: 0,
                }),
                background: None,
            },
            AnnotationType::LifetimeSpecifier => Self {
                foreground: Some(Color::Rgb {
                    r: 102,
                    g: 205,
                    b: 170,
                }),
                background: None,
            },
            AnnotationType::Match => Self {
                foreground: Some(Color::Rgb {
                    r: 34,
                    g: 139,
                    b: 34,
                }),
                background: None,
            },
            AnnotationType::String => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 179,
                    b: 102,
                }),
                background: None,
            },
        }
    }
}

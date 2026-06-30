use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Navbar,
    Hero,
    Footer,
    Sidebar,
    Card,
    Form,
    Modal,
    Table,
    Pricing,
    Testimonial,
    Cta,
    Feature,
    Faq,
    Contact,
    Auth,
    Dashboard,
    Settings,
    Profile,
    Landing,
    Blog,
    Ecommerce,
    Error,
    Loading,
    Notification,
    Section,       // Generic section
    #[default]
    Other,         // Catch-all
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self)
            .unwrap_or_default()
            .trim_matches('"'))
    }
}

impl FromStr for Category {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(&format!("\"{}\"", s.to_lowercase()))
            .map_err(|_| format!("Unknown category: '{s}'"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum Framework {
    Tailwind,
    #[default]
    Css,
    Scss,
    Shadcn,
    React,
}

impl fmt::Display for Framework {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self)
            .unwrap_or_default()
            .trim_matches('"'))
    }
}

impl FromStr for Framework {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        let normalized = match lower.as_str() {
            "tailwind" | "tailwindcss" => "tailwind",
            "css" | "vanillacss" => "css",
            "scss" | "sass" => "scss",
            "shadcn" | "shadcnui" => "shadcn",
            "react" | "reactjs" | "jsx" | "tsx" => "react",
            other => other,
        };
        serde_json::from_str(&format!("\"{}\"", normalized))
            .map_err(|_| format!("Unknown framework: '{s}'"))
    }
}

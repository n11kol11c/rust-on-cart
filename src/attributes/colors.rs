pub use crate::errors::error::CartError;
use std::fmt::{Display, Formatter, Result};

pub enum CartColor {
    Black(String),
    Red(String),
    Green(String),
    Yellow(String),
    Blue(String),
    Magenta(String),
    Cyan(String),
    White(String),
    BrightBlack(String),
    BrightRed(String),
    BrightGreen(String),
    BrightYellow(String),
    BrightBlue(String),
    BrightMagenta(String),
    BrightCyan(String),
    BrightWhite(String),
    Reset(String),
}

impl Display for CartColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CartColor::Black(val) => write!(f, "\x1b[30m{}", val),
            CartColor::Red(val) => write!(f, "\x1b[31m{}", val),
            CartColor::Green(val) => write!(f, "\x1b[32m{}", val),
            CartColor::Yellow(val) => write!(f, "\x1b[33m{}", val),
            CartColor::Blue(val) => write!(f, "\x1b[34m{}", val),
            CartColor::Magenta(val) => write!(f, "\x1b[35m{}", val),
            CartColor::Cyan(val) => write!(f, "\x1b[36m{}", val),
            CartColor::White(val) => write!(f, "\x1b[37m{}", val),
            CartColor::BrightBlack(val) => write!(f, "\x1b[90m{}", val),
            CartColor::BrightRed(val) => write!(f, "\x1b[91m{}", val),
            CartColor::BrightGreen(val) => write!(f, "\x1b[92m{}", val),
            CartColor::BrightYellow(val) => write!(f, "\x1b[93m{}", val),
            CartColor::BrightBlue(val) => write!(f, "\x1b[94m{}", val),
            CartColor::BrightMagenta(val) => write!(f, "\x1b[95m{}", val),
            CartColor::BrightCyan(val) => write!(f, "\x1b[96m{}", val),
            CartColor::BrightWhite(val) => write!(f, "\x1b[97m{}", val),
            CartColor::Reset(val) => write!(f, "\x1b[0m{}", val),
        }
    }
}

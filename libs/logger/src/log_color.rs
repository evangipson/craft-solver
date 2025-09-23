const RESET_COLOR: &str = "\x1b";

/// [`LogColor`] represents colors that log messages can use.
pub enum LogColor {
    /// [`LogColor::BrightBlue`] represents a bright shade of blue.
    BrightBlue,
    /// [`LogColor::BrightGreen`] represents a bright shade of green.
    BrightGreen,
    /// [`LogColor::BrightRed`] represents a bright shade of red.
    BrightRed,
    /// [`LogColor::BrightYellow`] represents a bright shade of yellow.
    BrightYellow,
    /// [`LogColor::Blue`] represents a medium shade of blue.
    Blue,
    /// [`LogColor::Green`] represents a medium shade of green.
    Green,
    /// [`LogColor::Red`] represents a medium shade of red.
    Red,
    /// [`LogColor::Yellow`] represents a medium shade of yellow.
    Yellow,
    /// [`LogColor::White`] represents a bright shade of white.
    White,
    /// [`LogColor::Grey`] represents a dark shade of white.
    Grey,
}

/// Implement [`ToString`] for [`LogColor`].
impl ToString for LogColor {
    fn to_string(&self) -> String {
        match self {
            LogColor::Green => format!("{RESET_COLOR}[32m"),
            LogColor::BrightGreen => format!("{RESET_COLOR}[32;1m"),
            LogColor::Blue => format!("{RESET_COLOR}[34m"),
            LogColor::BrightBlue => format!("{RESET_COLOR}[34;1m"),
            LogColor::Yellow => format!("{RESET_COLOR}[33m"),
            LogColor::BrightYellow => format!("{RESET_COLOR}[33;1m"),
            LogColor::Red => format!("{RESET_COLOR}[31m"),
            LogColor::BrightRed => format!("{RESET_COLOR}[31;1m"),
            LogColor::White => format!("{RESET_COLOR}[37;0m"),
            LogColor::Grey => format!("{RESET_COLOR}[37;2m"),
        }
    }
}

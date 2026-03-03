use std::fmt;

use crate::model::KeyboardLang;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Key {
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,

    // numbers
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,

    // symbols
    IsoExtra,
    Grave,
    Hyphen,
    Equal,
    LeftBracket,
    RightBracket,
    Backslash,
    SemiColon,
    Apostrophe,
    Comma,
    Period,
    QuestionMark,

    // modifiers
    Esc,
    Tab,
    CapsLock,
    LeftShift,
    LeftCtrl,
    LeftSuper,
    LeftAlt,
    Spacebar,
    RightAlt,
    RightSuper,
    RightCtrl,
    RightShift,
    Return,
    Backspace,
    Insert,
    Home,
    PgUp,
    Delete,
    End,
    PgDown,

    Command,

    // arrows
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,

    // F keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // misc
    Separator,
    PrintScreen,
    ScrollLock,
    PauseBreak,

    // Numpad
    NumLock,
    Div,
    Mul,
    Minus,
    Plus,
    Decimal,
    NumpadEnter,
    NumpadZero,
    NumpadOne,
    NumpadTwo,
    NumpadThree,
    NumpadFour,
    NumpadFive,
    NumpadSix,
    NumpadSeven,
    NumpadEight,
    NumpadNine,
}

impl Key {
    pub fn label(&self, lang: &KeyboardLang) -> String {
        match lang {
            KeyboardLang::US => self.to_string(),
            // On Windows DE, the keyboard driver (KBDGR.DLL) remaps VK_OEM codes
            // so each physical position sends a DIFFERENT VK code than on US.
            // The labels here match Key variants to what the DE physical position
            // actually produces via its remapped VK code.
            KeyboardLang::DE => match self {
                Key::Backslash => String::from("^"),             // ^-pos: VK_OEM_5 -> BackSlash
                Key::LeftBracket => String::from("\u{00DF}"),    // ß-pos: VK_OEM_4 -> LeftBracket
                Key::RightBracket => String::from("\u{00B4}"),   // ´-pos: VK_OEM_6 -> RightBracket
                Key::SemiColon => String::from("\u{00FC}"),      // ü-pos: VK_OEM_1 -> Semicolon
                Key::Equal => String::from("+*"),                // +-pos: VK_OEM_PLUS -> Equal
                Key::QuestionMark => String::from("#'"),         // #-pos: VK_OEM_2 -> Slash
                Key::Grave => String::from("\u{00F6}"),          // ö-pos: VK_OEM_3 -> Grave
                Key::Apostrophe => String::from("\u{00E4}"),     // ä-pos: VK_OEM_7 (same)
                Key::Comma => String::from(",;"),                // same VK
                Key::Period => String::from(".:"),               // same VK
                Key::Hyphen => String::from("-_"),               // --pos: VK_OEM_MINUS -> Hyphen
                Key::IsoExtra => String::from("<>"),             // same VK
                _ => self.to_string(),
            },
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Key::CapsLock => write!(f, "Caps"),
            Key::Backspace => write!(f, "Back"),
            Key::LeftShift => write!(f, "Shift"),
            Key::LeftCtrl => write!(f, "Ctrl"),
            Key::LeftSuper => write!(f, "Super"),
            Key::LeftAlt => write!(f, "Alt"),
            Key::LeftBracket => write!(f, "{{["),
            Key::RightShift => write!(f, "Shift"),
            Key::RightCtrl => write!(f, "Ctrl"),
            Key::RightSuper => write!(f, "Super"),
            Key::RightAlt => write!(f, "Alt"),
            Key::RightBracket => write!(f, "}}]"),
            Key::Backslash => write!(f, "|\\"),
            Key::SemiColon => write!(f, ":;"),
            Key::Apostrophe => write!(f, "\"'"),
            Key::Comma => write!(f, "<,"),
            Key::Period => write!(f, ">."),
            Key::QuestionMark => write!(f, "?/"),
            Key::One => write!(f, "1"),
            Key::Two => write!(f, "2"),
            Key::Three => write!(f, "3"),
            Key::Four => write!(f, "4"),
            Key::Five => write!(f, "5"),
            Key::Six => write!(f, "6"),
            Key::Seven => write!(f, "7"),
            Key::Eight => write!(f, "8"),
            Key::Nine => write!(f, "9"),
            Key::Zero => write!(f, "0"),
            Key::Hyphen => write!(f, "-"),
            Key::Equal => write!(f, "+"),
            Key::Grave => write!(f, "~`"),
            Key::ArrowUp => write!(f, "↑"),
            Key::ArrowDown => write!(f, "↓"),
            Key::ArrowLeft => write!(f, "←"),
            Key::ArrowRight => write!(f, "→"),
            Key::PauseBreak => write!(f, "P/B"),
            Key::PrintScreen => write!(f, "Prn"),
            Key::ScrollLock => write!(f, "Lck"),
            Key::IsoExtra => write!(f, "<>"),
            Key::Separator => write!(f, ""),
            Key::NumLock => write!(f, "NLck"),
            Key::Div => write!(f, "/"),
            Key::Mul => write!(f, "*"),
            Key::Minus => write!(f, "-"),
            Key::Plus => write!(f, "+"),
            Key::Decimal => write!(f, "."),
            Key::NumpadEnter => write!(f, "Enter"),
            Key::NumpadZero => write!(f, "0"),
            Key::NumpadOne => write!(f, "1"),
            Key::NumpadTwo => write!(f, "2"),
            Key::NumpadThree => write!(f, "3"),
            Key::NumpadFour => write!(f, "4"),
            Key::NumpadFive => write!(f, "5"),
            Key::NumpadSix => write!(f, "6"),
            Key::NumpadSeven => write!(f, "7"),
            Key::NumpadEight => write!(f, "8"),
            Key::NumpadNine => write!(f, "9"),
            _ => write!(f, "{:?}", self),
        }
    }
}

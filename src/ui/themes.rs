use ratatui::style::{ Color, Modifier, Style };

pub const ACCENT: Color = Color::Rgb(118, 150, 86); // burnt orange — primary brand color
pub const MUTED: Color = Color::Rgb(92, 99, 112); // mid gray — secondary text, descriptions
pub const SUBTLE: Color = Color::Rgb(55, 62, 77); // dark gray — inactive borders

// pub const SUCCESS: Color = Color::Rgb(152, 195, 121); // valid move highlight
// pub const WARNING: Color = Color::Rgb(229, 192, 123); // check warning
// pub const ERR: Color = Color::Rgb(224, 108, 117);     // checkmate / illegal move
// pub const BRIGHT: Color = Color::Rgb(171, 178, 191);  // selected piece highlight
// pub const WHITE: Color = Color::Rgb(220, 223, 228);   // primary text

// --- board squares ---
// pub const SQUARE_LIGHT: Color = Color::Rgb(240, 217, 181); // classic light square (cream)
// pub const SQUARE_DARK: Color = Color::Rgb(181, 136, 99);   // classic dark square (brown)
// pub const SQUARE_SELECTED: Color = Color::Rgb(106, 168, 79);  // selected piece square (green)
// pub const SQUARE_LEGAL: Color = Color::Rgb(106, 135, 79);     // legal move target square
// pub const SQUARE_LAST_MOVE: Color = Color::Rgb(205, 210, 106); // last move highlight (yellow)

pub fn accent() -> Style {
    Style::default().fg(ACCENT)
}

pub fn muted() -> Style {
    Style::default().fg(MUTED)
}

pub fn subtle() -> Style {
    Style::default().fg(SUBTLE)
}

pub fn bold_accent() -> Style {
    Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
}

// pub fn err() -> Style {
//     Style::default().fg(ERR)
// }
// pub fn bright() -> Style {
//     Style::default().fg(BRIGHT)
// }
// pub fn bold_white() -> Style {
//     Style::default().fg(WHITE).add_modifier(Modifier::BOLD)
// }

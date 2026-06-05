use tui_big_text::{ BigText, PixelSize };
use ratatui::{
    Frame,
    layout::{ Alignment, Constraint, Direction, Layout },
    text::Line,
    widgets::Paragraph,
};
use crate::app::App;
use super::themes;

// Each entry is (button label, description shown below the buttons).
const ITEMS: &[(&str, &str)] = &[
    ("SinglePlayer", "Play against the machine"),
    ("Multiplayer", "Local multiplayer against a friend"),
    ("Exit", "Close the application"),
];

pub fn draw(frame: &mut Frame, app: &App) {
    // Restrict the layout to the centered 75% column.
    let area = super::centered_area(frame.area());

    // Vertical slots:
    // [0] top spacer     — pushes content toward center
    // [1] logo           — BigText title (4 lines with HalfHeight pixel size)
    // [2] subtitle       — single muted line beneath the logo
    // [3] gap            — breathing room before the buttons
    // [4] buttons        — horizontal toggle row
    // [5] description    — one-line hint for the focused button
    // [6] bottom spacer  — mirrors the top spacer
    // [7] help bar       — key reference at the very bottom
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .split(area);

    let big_text = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .centered()
        .lines([Line::from("Echecs").style(themes::bold_accent())])
        .build();
    frame.render_widget(big_text, outer[1]);

    frame.render_widget(
        Paragraph::new("Chess, in Rust").alignment(Alignment::Center).style(themes::muted()),
        outer[2]
    );

    super::draw_buttons(frame, outer[4], ITEMS, app.menu_cursor);

    // Show the description of whichever item is focused.
    if let Some((_, desc)) = ITEMS.get(app.menu_cursor) {
        frame.render_widget(
            Paragraph::new(*desc).alignment(Alignment::Center).style(themes::muted()),
            outer[5]
        );
    }

    super::draw_help(
        frame,
        outer[8],
        &[
            ("← →", "navigate"),
            ("↵", "select"),
            ("q", "quit"),
        ]
    );
}

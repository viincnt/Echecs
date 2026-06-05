pub mod board;
pub mod menu;
pub mod themes;

use ratatui::{
    Frame,
    layout::{ Alignment, Constraint, Direction, Layout, Rect },
    text::{ Line, Span },
    widgets::{ Block, Paragraph },
};

// Constrains the drawable area to 75% of the terminal width, centered.
// Prevents layouts from stretching uncomfortably on wide terminals.
pub fn centered_area(area: Rect) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Percentage(75), Constraint::Fill(1)])
        .split(area);
    chunks[1]
}

// Renders a row of horizontally equal-width buttons.
// The button at `cursor` is highlighted with the accent style.
pub fn draw_buttons(frame: &mut Frame, area: Rect, items: &[(&str, &str)], cursor: usize) {
    // Each button gets an equal share of the available width.
    let constraints: Vec<Constraint> = items
        .iter()
        .map(|_| Constraint::Fill(1))
        .collect();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for (i, (label, _)) in items.iter().enumerate() {
        let (block_style, text_style) = if i == cursor {
            (themes::bold_accent(), themes::bold_accent())
        } else {
            (themes::subtle(), themes::muted())
        };

        let paragraph = Paragraph::new(*label)
            .alignment(Alignment::Center)
            .block(Block::bordered().style(block_style))
            .style(text_style);
        frame.render_widget(paragraph, chunks[i]);
    }
}

// Renders a centered help bar with alternating key / description pairs.
// Keys are styled with the accent color; descriptions are muted.
pub fn draw_help(frame: &mut Frame, area: Rect, items: &[(&str, &str)]) {
    let spans: Vec<Span> = items
        .iter()
        .flat_map(|(key, desc)| {
            [
                Span::styled(*key, themes::accent()),
                Span::styled(format!("  {}   ", desc), themes::muted()),
            ]
        })
        .collect();

    frame.render_widget(Paragraph::new(Line::from(spans)).alignment(Alignment::Center), area);
}

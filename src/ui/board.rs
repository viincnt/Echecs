use ratatui::{ Frame, layout::Alignment, widgets::{ Block, Paragraph } };

pub fn draw(frame: &mut Frame) {
    let text = Paragraph::new("Hello, Ratatui! (Press 'q' to quit)")
        .alignment(Alignment::Center)
        .block(Block::bordered());

    frame.render_widget(text, frame.area());
}

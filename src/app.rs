use crossterm::event::{ self, Event, KeyCode };
use ratatui::{ backend::CrosstermBackend, Terminal };
use std::io;

use crate::ui::{ board, menu };

// Top-level screens of the application.
pub enum Screen {
    Menu,
    Game,
}

// Central application state passed (by reference) into every draw call.
pub struct App {
    pub menu_cursor: usize, // index of the currently highlighted menu button
    pub screen: Screen, // which screen is currently rendered
    pub running: bool, // set to false to exit the main loop
}

impl App {
    pub fn new() -> Self {
        Self {
            menu_cursor: 0,
            screen: Screen::Menu,
            running: true,
        }
    }
}

// Must stay in sync with the number of items in ui::menu::ITEMS.
const MENU_LEN: usize = 3;

pub fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new();

    while app.running {
        // Draw the active screen, passing app state as read-only.
        terminal.draw(|frame| {
            match app.screen {
                Screen::Menu => menu::draw(frame, &app),
                Screen::Game => board::draw(frame),
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match app.screen {
                Screen::Menu =>
                    match key.code {
                        KeyCode::Left => {
                            if app.menu_cursor > 0 {
                                app.menu_cursor -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if app.menu_cursor < MENU_LEN - 1 {
                                app.menu_cursor += 1;
                            }
                        }
                        // Items 0 (SinglePlayer) and 1 (Multiplayer) both launch the game;
                        // the last item is always Exit.
                        KeyCode::Enter =>
                            match app.menu_cursor {
                                0 | 1 => {
                                    app.screen = Screen::Game;
                                }
                                _ => {
                                    app.running = false;
                                }
                            }
                        KeyCode::Char('q') => {
                            app.running = false;
                        }
                        _ => {}
                    }
                // From any game screen, q returns to the main menu.
                Screen::Game => {
                    if key.code == KeyCode::Char('q') {
                        app.screen = Screen::Menu;
                    }
                }
            }
        }
    }

    Ok(())
}

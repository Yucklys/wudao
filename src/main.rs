use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use wudao::app::{App, GameResult};
use wudao::event::Event;
use wudao::handler::*;
use wudao::tui::Tui;

#[tokio::main]
async fn main() -> GameResult<()> {
    App::sync();
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    app.init(250)?;
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        if let Some(event) = app.events.next().await {
            match event {
                Event::Tick => app.tick(),
                Event::Crossterm(event) => handle_term_events(event, &mut app)?,
                Event::Action(action) => handle_actions(action, &mut app)?,
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

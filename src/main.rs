use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use wudao::app::{App, AppResult};
use wudao::event::{Event, EventHandler};
use wudao::handler::handle_events;
use wudao::tui::Tui;

#[tokio::main]
async fn main() -> AppResult<()> {
    App::sync();
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new();
    events.start(250)?;
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        if let Some(event) = tui.events.next().await {
            match event {
                Event::Tick => app.tick(),
                Event::Crossterm(event) => handle_events(event, &mut app)?,
                Event::Action(_) => {}
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

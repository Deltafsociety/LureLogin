use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

pub fn start_tui(stats: String) {
    enable_raw_mode().unwrap();

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let size = f.area();

            let block = Paragraph::new(stats)
                .block(Block::default().title("SSHHoney").borders(Borders::ALL));

            f.render_widget(block, size);
        })
        .unwrap();
}

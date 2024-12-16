mod header;
mod taskfinder;
mod taskpreview;

use header::{Controls, Header};
use taskfinder::TaskFinder;
use taskpreview::TaskPreview;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, ScrollbarState};
use ratatui::Terminal;
use std::io;
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Arc::new(Mutex::new(App::new()));
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, app.clone()).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        let error_message = format!("{:?}", err);
        println!("{}", error_message);
        app.lock().await.log_error(&error_message).await;
    }

    Ok(())
}

struct App {
    pub task_finder: TaskFinder,
    task_preview: TaskPreview,
    error_message: Option<String>,
}

impl App {
    fn new() -> App {
        App {
            task_finder: TaskFinder::new(),
            task_preview: TaskPreview::new(),
            error_message: None,
        }
    }

    async fn log_error(&self, error_message: &str) {
        let mut file = match OpenOptions::new()
            .create(true)
            .append(true)
            .open("error.log")
            .await
        {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open log file: {}", e);
                return;
            }
        };

        if let Err(e) = file
            .write_all(format!("{}\n", error_message).as_bytes())
            .await
        {
            eprintln!("Failed to write to log file: {}", e);
        }
    }
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: Arc<Mutex<App>>,
) -> io::Result<()> {
    loop {
        let mut app = app.lock().await;
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(6), Constraint::Min(0)].as_ref())
                .split(f.area());

            let header_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(chunks[0]);

            let header = Header::new();
            let controls = Controls::new();
            f.render_widget(header, header_chunks[0]);
            f.render_widget(controls, header_chunks[1]);

            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(chunks[1]);

            app.task_finder
                .render::<CrosstermBackend<std::io::Stdout>>(f, main_chunks[0]);
            app.task_preview
                .render::<CrosstermBackend<std::io::Stdout>>(f, main_chunks[1]);

            if let Some(error_message) = &app.error_message {
                let error_block = Block::default()
                    .borders(Borders::ALL)
                    .title("Error")
                    .border_style(Style::default().fg(Color::Red));
                let error_message_text = format!("{}\n\nPress Enter to close", error_message);
                let error_paragraph =
                    Paragraph::new(error_message_text.as_str()).block(error_block);
                let area = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(50),
                            Constraint::Percentage(25),
                            Constraint::Percentage(25),
                        ]
                        .as_ref(),
                    )
                    .split(f.area())[1];
                let area = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Percentage(25),
                            Constraint::Percentage(50),
                            Constraint::Percentage(25),
                        ]
                        .as_ref(),
                    )
                    .split(area)[1];
                f.render_widget(Clear, area);
                f.render_widget(error_paragraph, area);
            }
        })?;

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            let mut app = app;
            match code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Down | KeyCode::Char('s') => {
                    app.task_finder.file_tree_state.key_down();
                }
                KeyCode::Up | KeyCode::Char('w') => {
                    app.task_finder.file_tree_state.key_up();
                }
                KeyCode::Right | KeyCode::Char('d') => {
                    app.task_finder.file_tree_state.key_right();
                }
                KeyCode::Left | KeyCode::Char('a') => {
                    app.task_finder.file_tree_state.key_left();
                }
                KeyCode::Enter => {
                    if app.error_message.is_some() {
                        app.error_message = None;
                        return Ok(());
                    }

                    let file_path = app.task_finder.file_tree_state.selected().join("/");
                    if file_path.contains("task") {
                        if let Err(e) = app.task_preview.run_task(&file_path).await {
                            app.error_message = Some(format!("Failed to run task: {}", e));
                        }
                    }
                }
                KeyCode::PageUp => {
                    if app.task_preview.scroll_offset > 10 {
                        app.task_preview.scroll_offset -= 10;
                    } else {
                        app.task_preview.scroll_offset = 0;
                    }
                    app.task_preview.scrollbar_state =
                        ScrollbarState::new(app.task_preview.total_lines)
                            .position(app.task_preview.scroll_offset);
                }
                KeyCode::PageDown => {
                    if app.task_preview.scroll_offset + 10 < app.task_preview.total_lines {
                        app.task_preview.scroll_offset += 10;
                    } else {
                        app.task_preview.scroll_offset = app.task_preview.total_lines;
                    }
                    app.task_preview.scrollbar_state =
                        ScrollbarState::new(app.task_preview.total_lines)
                            .position(app.task_preview.scroll_offset);
                }
                _ => {}
            }
        }
    }
}

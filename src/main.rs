use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Terminal;
use regex::Regex;
use std::io;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use tokio::fs::OpenOptions;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tui_tree_widget::{Tree, TreeItem, TreeState}; // Add TreeState import

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
    file_tree: Vec<TreeItem<'static, String>>, // Replace file_picker with file_tree
    file_tree_state: TreeState<String>,        // Add file_tree_state
    file_preview: String,
    error_message: Option<String>,
}

impl App {
    fn new() -> App {
        let file_tree = App::load_file_tree(); // Initialize file_tree
        let mut file_tree_state = TreeState::default();
        file_tree_state.select_first(); // Select the first item by default
        App {
            file_tree,
            file_tree_state,
            file_preview: "Press Enter to execute a task".to_string(),
            error_message: None,
        }
    }

    fn load_file_tree() -> Vec<TreeItem<'static, String>> {
        let mut items = Vec::new();
        let re = Regex::new(r"^day\d+$").unwrap();
        let paths = match std::fs::read_dir(".") {
            Ok(paths) => paths,
            Err(_) => return items,
        };

        for path in paths {
            let path = match path {
                Ok(path) => path.path(),
                Err(_) => continue,
            };
            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if re.is_match(dir_name) {
                        let dir_item =
                            TreeItem::new(dir_name.to_string(), dir_name.to_string(), vec![]);
                        if let Ok(mut dir_item) = dir_item {
                            App::add_sub_items(&path, &mut dir_item);
                            items.push(dir_item);
                        }
                    }
                }
            }
        }
        items
    }

    fn add_sub_items(path: &Path, parent: &mut TreeItem<'static, String>) {
        let paths = match std::fs::read_dir(path) {
            Ok(paths) => paths,
            Err(_) => return,
        };

        for path in paths {
            let path = match path {
                Ok(path) => path.path(),
                Err(_) => continue,
            };
            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if dir_name == "target" {
                        continue;
                    }
                    let dir_item =
                        TreeItem::new(dir_name.to_string(), dir_name.to_string(), vec![]);
                    if let Ok(mut dir_item) = dir_item {
                        App::add_sub_items(&path, &mut dir_item);
                        match parent.add_child(dir_item) {
                            Ok(_) => {}
                            Err(_) => {
                                println!("Error adding child");
                            }
                        }
                    }
                }
            } else if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                let leaf = TreeItem::new_leaf(file_name.to_string(), file_name.to_string());
                match parent.add_child(leaf) {
                    Ok(_) => {}
                    Err(_) => {
                        println!("Error adding leaf");
                    }
                }
            }
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

enum AppEvent {
    LoadFile(String),
    Key(KeyCode),
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

            let header = Paragraph::new(
                r#"    _       _                 _            __    ____          _        ____   ___ ____  _  _   
   / \   __| |_   _____ _ __ | |_    ___  / _|  / ___|___   __| | ___  |___ \ / _ \___ \| || |  
  / _ \ / _` \ \ / / _ \ '_ \| __|  / _ \| |_  | |   / _ \ / _` |/ _ \   __) | | | |__) | || |_ 
 / ___ \ (_| |\ V /  __/ | | | |_  | (_) |  _| | |__| (_) | (_| |  __/  / __/| |_| / __/|__   _|
/_/   \_\__,_| \_/ \___|_| |_|\__|  \___/|_|    \____\___/ \__,_|\___| |_____|\___/_____|  |_|  
                "#,
            )
            .block(Block::default().borders(Borders::NONE));

            let controls = Paragraph::new(
                r#"Controls:
  q: Quit
  w: Up
  s: Down
  a: Left
  d: Right
                "#,
            )
            .block(Block::default().borders(Borders::NONE))
            .alignment(ratatui::layout::Alignment::Right);

            f.render_widget(header, header_chunks[0]);
            f.render_widget(controls, header_chunks[1]);

            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(chunks[1]);

            let binding = app.file_tree.clone();
            let file_tree = Tree::new(&binding)
                .unwrap()
                .block(Block::default().borders(Borders::ALL).title("Files"))
                .highlight_style(Style::default().fg(Color::Yellow))
                .highlight_symbol(">> ");

            f.render_stateful_widget(file_tree, main_chunks[0], &mut app.file_tree_state);

            let file_preview_block = Block::default().borders(Borders::ALL).title("Preview");
            let file_preview = Paragraph::new(app.file_preview.as_str()).block(file_preview_block);
            f.render_widget(file_preview, main_chunks[1]);

            if let Some(error_message) = &app.error_message {
                let error_block = Block::default()
                    .borders(Borders::ALL)
                    .title("Error")
                    .border_style(Style::default().fg(Color::Red));
                let error_message_text = format!("{}\n\nPress Enter to close", error_message);
                let error_paragraph = Paragraph::new(error_message_text.as_str())
                    .block(error_block);
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
                f.render_widget(Clear, area); // Clear the area before rendering the error message
                f.render_widget(error_paragraph, area);
            }
        })?;

        if let Event::Key(key) = event::read()? {
            let mut app = app;
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Down | KeyCode::Char('s') => {
                    app.file_tree_state.key_down();
                }
                KeyCode::Up | KeyCode::Char('w') => {
                    app.file_tree_state.key_up();
                }
                KeyCode::Right | KeyCode::Char('d') => {
                    app.file_tree_state.key_right();
                }
                KeyCode::Left | KeyCode::Char('a') => {
                    app.file_tree_state.key_left();
                }
                KeyCode::Enter => {
                    if app.error_message.is_some() {
                        app.error_message = None;
                    } else if let Some(file_name) = app.file_tree_state.selected().last() {
                        let file_path = format!("./{}", file_name);
                        // app.load_file(&file_path).await;
                    }
                }
                _ => {}
            }
        }
    }
}

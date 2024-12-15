use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent}; // Add KeyEvent import
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarState};
use ratatui::Terminal;
use regex::Regex;
use std::io;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
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

struct TaskFinder {
    file_tree: Vec<TreeItem<'static, String>>,
    file_tree_state: TreeState<String>,
}

impl TaskFinder {
    fn new() -> TaskFinder {
        let file_tree = App::load_file_tree();
        let mut file_tree_state = TreeState::default();
        Self::open_all_day_tasks(&file_tree, &mut file_tree_state);
        file_tree_state.select_first();
        TaskFinder {
            file_tree,
            file_tree_state,
        }
    }

    fn open_all_day_tasks(
        file_tree: &Vec<TreeItem<String>>,
        file_tree_state: &mut TreeState<String>,
    ) {
        file_tree.iter().for_each(|i| {
            let identifier = i.identifier().to_string();
            file_tree_state.open(vec![identifier]);
        });
    }

    fn render<B: ratatui::backend::Backend>(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
    ) {
        let binding = self.file_tree.clone();
        let file_tree = Tree::new(&binding)
            .unwrap()
            .block(Block::default().borders(Borders::ALL).title("Files"))
            .highlight_style(Style::default().fg(Color::Yellow))
            .highlight_symbol(">> ");
        f.render_stateful_widget(file_tree, area, &mut self.file_tree_state);
    }
}

struct TaskPreview {
    file_preview: String,
    scroll_offset: usize,
    total_lines: usize,
    scrollbar_state: ScrollbarState,
}

impl TaskPreview {
    fn new() -> TaskPreview {
        TaskPreview {
            file_preview: "Press Enter to execute a task".to_string(),
            scroll_offset: 0,
            total_lines: 0,
            scrollbar_state: ScrollbarState::default(),
        }
    }

    fn render<B: ratatui::backend::Backend>(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
    ) {
        let preview_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
            .split(area);

        let file_preview_block = Block::default().borders(Borders::ALL).title("Preview");
        let file_preview = Paragraph::new(self.file_preview.as_str())
            .block(file_preview_block)
            .scroll((self.scroll_offset.saturating_sub(10).try_into().unwrap(), 0));
        f.render_widget(file_preview, preview_chunks[0]);

        let scrollbar = Scrollbar::default().style(Style::default().fg(Color::Yellow));
        f.render_stateful_widget(scrollbar, preview_chunks[1], &mut self.scrollbar_state);
    }
}

struct App {
    task_finder: TaskFinder,
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
                            App::add_task_items(&path, &mut dir_item);
                            items.push(dir_item);
                        }
                    }
                }
            }
        }

        items
    }

    fn add_task_items(path: &Path, parent: &mut TreeItem<'static, String>) {
        let tasks = ["task1", "task2"];
        for task in tasks.iter() {
            let task_path = path.join(task);
            if task_path.is_dir() {
                let task_item = TreeItem::new(task.to_string(), task.to_string(), vec![]);
                if let Ok(task_item) = task_item {
                    match parent.add_child(task_item) {
                        Ok(_) => {}
                        Err(_) => {
                            println!("Error adding task");
                        }
                    }
                }
            }
        }
    }

    async fn run_task(&mut self, task_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let full_path = std::env::current_dir().unwrap().join(task_path);
        self.log_error(full_path.to_str().unwrap()).await;

        self.task_preview.file_preview.clear();
        self.task_preview.scroll_offset = 0;
        self.task_preview.total_lines = 0;

        let mut command = Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .current_dir(full_path)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Failed to start task");

        let stdout = command.stdout.take().expect("Failed to open stdout");
        let mut reader = tokio::io::BufReader::new(stdout).lines();

        while let Some(line) = reader.next_line().await? {
            self.task_preview.file_preview.push_str(&line);
            self.task_preview.file_preview.push('\n');
            self.task_preview.total_lines += 1;
            self.task_preview.scrollbar_state = ScrollbarState::new(self.task_preview.total_lines)
                .position(self.task_preview.scroll_offset);
        }

        let output = command.wait().await;

        match output {
            Ok(_) => Ok(()),
            Err(e) => {
                self.error_message = Some(format!("Failed to run task: {}", e));
                Err(Box::new(e))
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

            app.task_finder.render::<CrosstermBackend<std::io::Stdout>>(f, main_chunks[0]);
            app.task_preview.render::<CrosstermBackend<std::io::Stdout>>(f, main_chunks[1]);

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
                        if let Err(e) = app.run_task(&file_path).await {
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

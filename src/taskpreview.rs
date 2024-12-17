use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::symbols::line;
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarState};
use strip_ansi_escapes::strip;
use tokio::io::{AsyncBufReadExt, AsyncReadExt};
use tokio::process::Command;

pub struct TaskPreview {
    pub file_preview: String,
    pub scroll_offset: usize,
    pub total_lines: usize,
    pub scrollbar_state: ScrollbarState,
}

impl TaskPreview {
    pub fn new() -> TaskPreview {
        TaskPreview {
            file_preview: "Press Enter to execute a task".to_string(),
            scroll_offset: 0,
            total_lines: 0,
            scrollbar_state: ScrollbarState::default(),
        }
    }

    pub fn render<B: ratatui::backend::Backend>(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
    ) {
        let preview_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
            .split(area);

        let file_preview_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title("Preview");

        let file_preview = Paragraph::new(self.file_preview.as_str())
            .block(file_preview_block)
            .scroll((self.scroll_offset.saturating_sub(10).try_into().unwrap(), 0));

        f.render_widget(file_preview, preview_chunks[0]);

        let scrollbar = Scrollbar::default().style(Style::default().fg(Color::Green));
        f.render_stateful_widget(scrollbar, preview_chunks[1], &mut self.scrollbar_state);
    }

    pub async fn run_task(&mut self, task_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let full_path = std::env::current_dir().unwrap().join(task_path);

        self.file_preview.clear();
        self.scroll_offset = 0;
        self.total_lines = 0;

        let mut command = Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .current_dir(full_path)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Failed to start task");

        let stdout = command.stdout.take().expect("Failed to open stdout");
        let mut buffer = Vec::new();
        let mut reader = tokio::io::BufReader::new(stdout);
        reader.read_to_end(&mut buffer).await?;
        let colored_text = String::from_utf8_lossy(&buffer);

        let lines = colored_text.lines().count();
        self.file_preview.push_str(&colored_text);
        self.total_lines = lines;
        self.scrollbar_state = ScrollbarState::new(self.total_lines).position(self.scroll_offset);

        let output = command.wait().await;
        match output {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}

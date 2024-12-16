use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders};
use regex::Regex;
use std::path::Path;
use tui_tree_widget::{Tree, TreeItem, TreeState};

pub struct TaskFinder {
    pub file_tree: Vec<TreeItem<'static, String>>,
    pub file_tree_state: TreeState<String>,
}

impl TaskFinder {
    pub fn new() -> TaskFinder {
        let file_tree = Self::load_file_tree();
        let mut file_tree_state = TreeState::default();
        Self::open_all_day_tasks(&file_tree, &mut file_tree_state);
        file_tree_state.select_first();
        TaskFinder {
            file_tree,
            file_tree_state,
        }
    }

    fn load_file_tree() -> Vec<TreeItem<'static, String>> {
        let mut items = Vec::new();
        let re = Regex::new(r"^day\d+$").unwrap();
        let paths = match std::fs::read_dir(".") {
            Ok(paths) => paths,
            Err(_) => return items,
        };

        // Find all directories in the current directory that match the regex
        let filtered_paths = paths
            .filter(|p| p.is_ok())
            .filter(|p| p.as_ref().unwrap().path().is_dir())
            .filter(|p| {
                let binding = p.as_ref().unwrap().path();
                let dir_name = binding.file_name().and_then(|n| n.to_str());
                dir_name.is_some() && re.is_match(dir_name.unwrap())
            });

        // Sort the paths numerically by day
        let mut paths: Vec<_> = filtered_paths.collect();
        paths.sort_by(|a, b| {
            let a = a.as_ref().unwrap().path();
            let b = b.as_ref().unwrap().path();
            let a = a.file_name().and_then(|n| n.to_str()).unwrap();
            let b = b.file_name().and_then(|n| n.to_str()).unwrap();
            let a = a.trim_start_matches("day").parse::<u32>().unwrap();
            let b = b.trim_start_matches("day").parse::<u32>().unwrap();
            a.cmp(&b)
        });

        for path in paths {
            let path = path.as_ref().unwrap().path();
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            let dir_item = TreeItem::new(dir_name.to_string(), dir_name.to_string(), vec![]);
            if let Ok(mut dir_item) = dir_item {
                Self::add_task_items(&path, &mut dir_item);
                items.push(dir_item);
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

    pub fn open_all_day_tasks(
        file_tree: &Vec<TreeItem<String>>,
        file_tree_state: &mut TreeState<String>,
    ) {
        file_tree.iter().for_each(|i| {
            let identifier = i.identifier().to_string();
            file_tree_state.open(vec![identifier]);
        });
    }

    pub fn render<B: ratatui::backend::Backend>(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
    ) {
        let binding = self.file_tree.clone();
        let file_tree = Tree::new(&binding)
            .unwrap()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Red))
                    .title("Days"),
            )
            .highlight_style(Style::default().fg(Color::Green))
            .highlight_symbol(">> ");
        f.render_stateful_widget(file_tree, area, &mut self.file_tree_state);
    }
}

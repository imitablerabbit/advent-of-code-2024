use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Paragraph, Widget};

pub struct Header {
    letters: Vec<String>,
}

impl Header {
    pub fn new() -> Header {
        let letters = vec![
            a(),
            d(),
            v(),
            e(),
            n(),
            t(),
            space(),
            o(),
            f(),
            space(),
            c(),
            o(),
            d(),
            e(),
            space(),
            two(),
            zero(),
            two(),
            four(),
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        Header { letters }
    }
}

impl Widget for Header {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                self.letters
                    .iter()
                    .map(|s| {
                        let width = s.lines().next().unwrap().len();
                        Constraint::Length(width as u16)
                    })
                    .collect::<Vec<Constraint>>()
                    .as_slice(),
            )
            .split(area);

        for (i, letter) in self.letters.iter().enumerate() {
            let color = if i % 2 == 0 { Color::Red } else { Color::Green };
            let text = Text::styled(letter, Style::default().fg(color));
            let block = Block::default().borders(ratatui::widgets::Borders::NONE);
            let paragraph = Paragraph::new(text).block(block);
            paragraph.render(chunks[i], buf);
        }
    }
}

pub struct Controls {
    controls: Vec<String>,
}

impl Controls {
    pub fn new() -> Controls {
        let controls = [
            "q: Quit",
            "w: Up, s: Down",
            "a: Left, d: Right",
            "pgup: Scroll up",
            "pgdn: Scroll down",
            "enter: Run task",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        Controls { controls }
    }
}

impl Widget for Controls {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                self.controls
                    .iter()
                    .map(|_| Constraint::Length(1))
                    .collect::<Vec<Constraint>>()
                    .as_slice(),
            )
            .split(area);

        for (i, control) in self.controls.iter().enumerate() {
            let color = if i % 2 == 0 { Color::Red } else { Color::Green };
            let text = Text::styled(control, Style::default().fg(color));
            let block = Block::default().borders(ratatui::widgets::Borders::NONE);
            let paragraph = Paragraph::new(text)
                .block(block)
                .alignment(ratatui::layout::Alignment::Right);
            paragraph.render(chunks[i], buf);
        }
    }
}

fn a() -> &'static str {
    r#"    _    
   / \   
  / _ \  
 / ___ \ 
/_/   \_\"#
}

fn d() -> &'static str {
    r#"     _ 
  __| |
 / _` |
| (_| |
 \__,_|"#
}

fn v() -> &'static str {
    r#"       
__   __
\ \ / /
 \ V / 
  \_/  "#
}

fn e() -> &'static str {
    r#"      
  ___ 
 / _ \
|  __/
 \___|"#
}

fn n() -> &'static str {
    r#"       
 _ __  
| '_ \ 
| | | |
|_| |_|"#
}

fn t() -> &'static str {
    r#" _   
| |_ 
| __|
| |_ 
 \__|"#
}

fn o() -> &'static str {
    r#"       
  ___  
 / _ \ 
| (_) |
 \___/ "#
}

fn f() -> &'static str {
    r#"  __ 
 / _|
| |_ 
|  _|
|_|  "#
}

fn c() -> &'static str {
    r#"  ____ 
 / ___|
| |    
| |___ 
 \____|"#
}

fn two() -> &'static str {
    r#" ____  
|___ \ 
  __) |
 / __/ 
|_____|"#
}

fn zero() -> &'static str {
    r#"  ___  
 / _ \ 
| | | |
| |_| |
 \___/ "#
}

fn four() -> &'static str {
    r#" _  _   
| || |  
| || |_ 
|__   _|
   |_|  "#
}

fn space() -> &'static str {
    r#"   
   
   
   
   "#
}

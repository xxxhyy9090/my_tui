use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::event::KeyEventKind;
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<()> {
    // 备用屏幕
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut input_number_str=String::new();


    loop {
        terminal.draw(|f| {
            f.render_widget(
                ratatui::widgets::Paragraph::new(format!("INPUT YOUR NUMBER：{}",input_number_str)),
                f.area(),
            );
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind==KeyEventKind::Press{

                match key.code{
                    KeyCode::Char('q')=>break,
                    KeyCode::Char(c) => {
                        // 如果带 CONTROL 修饰（Ctrl+H 退格），当成退格处理
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            input_number_str.pop();
                        } else if c.is_ascii_digit() || "+-*/.".contains(c) {
                            input_number_str.push(c);
                        }
                    }

                    KeyCode::Backspace=>{input_number_str.pop();},
                    KeyCode::Enter=>input_number_str.clear(),
                    _=>(),
                };
            }

        }
    }

    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::event::KeyEventKind;
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<()> {
    // ① setup：进 raw 模式 + 备用屏幕
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut input_number_str=String::new();

    // ② 主循环：画 → 读按键 → 循环，按 q 退出
    loop {
        terminal.draw(|f| {
            f.render_widget(
                ratatui::widgets::Paragraph::new(format!("INPUT YOUR NUMBER：{}",input_number_str)),
                f.area(),   // 老版本可能是 f.size()
            );
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind==KeyEventKind::Press{
                match key.code{
                    KeyCode::Char('q')=>break,
                    KeyCode::Char(c)=>input_number_str.push(c),
                    KeyCode::Backspace=>{input_number_str.pop();},
                    KeyCode::Enter=>input_number_str.clear(),
                    _=>(),
                };
            }

        }
    }

    // ③ teardown：恢复终端（千万别漏！）
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
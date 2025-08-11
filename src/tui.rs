use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io;
use tui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

use crate::gap_buffer::GapBuffer;

pub fn init_tui() -> Result<(), io::Error> {
    // Create terminal and enter alternate mode`
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture,)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut gap_buffer = GapBuffer::new();
    draw_to_terminal(&mut term, gap_buffer.to_string())?;

    // TODO Show a cursor witin the block

    #[allow(clippy::single_match, clippy::collapsible_match, clippy::match_single_binding)]
    loop {
        match event::read()? {
            event::Event::Key(ev) => match ev {
                KeyEvent {
                    code,
                    modifiers: _,
                    kind: _,
                    state: _,
                } => match code {
                    KeyCode::Char(chr) => {
                        gap_buffer.insert(chr);
                        draw_to_terminal(&mut term, gap_buffer.to_string())?;
                    }
                    KeyCode::Backspace => {
                        gap_buffer.remove();
                        draw_to_terminal(&mut term, gap_buffer.to_string())?;
                    }
                    KeyCode::Enter => {
                        gap_buffer.insert('\n');
                        draw_to_terminal(&mut term, gap_buffer.to_string())?;
                    }
                    KeyCode::Right => gap_buffer.move_to(gap_buffer.gap_start + 1),
                    KeyCode::Left => gap_buffer.move_to(gap_buffer.gap_start - 1),
                    KeyCode::Esc => break,
                    _ => {}
                },
            },
            _ => {}
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(term.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    term.show_cursor()?;

    Ok(())
}

pub fn draw_to_terminal<S: AsRef<str>>(
    term: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    text: S,
) -> Result<(), std::io::Error> {
    let style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::ITALIC | Modifier::BOLD);

    term.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let text = str_into_spans_styled(text.as_ref(), style);

        let paragraph = Paragraph::new(text)
            .block(Block::default().title("Gap Buffer Example").borders(Borders::ALL))
            .style(Style::default().fg(Color::Magenta));
        f.render_widget(paragraph, chunks[0]);
    })?;

    Ok(())
}

pub fn str_into_spans(text: &str) -> Vec<Spans<'_>> {
    text.split_terminator('\n').map(Spans::from).collect::<Vec<_>>()
}

pub fn str_into_spans_styled(text: &str, style: Style) -> Vec<Spans<'_>> {
    text.split_terminator('\n')
        .map(|s| Spans::from(Span::styled(s, style)))
        .collect::<Vec<_>>()
}

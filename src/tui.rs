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
    draw_to_terminal(&mut term, gap_buffer.to_string(), gap_buffer.gap_start)?;

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
                    KeyCode::Char(chr) => gap_buffer.insert(chr),
                    KeyCode::Backspace => gap_buffer.remove(),
                    KeyCode::Enter => gap_buffer.insert('\n'),
                    KeyCode::Right => gap_buffer.move_to(gap_buffer.gap_start + 1),
                    KeyCode::Left => gap_buffer.move_to(gap_buffer.gap_start - 1),
                    KeyCode::Esc => break,
                    _ => {}
                },
            },
            _ => {}
        }

        draw_to_terminal(&mut term, gap_buffer.to_string(), gap_buffer.gap_start)?;
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
    gap_start: usize,
) -> Result<(), std::io::Error> {
    let style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::ITALIC | Modifier::BOLD);

    let cursor_style = Style::default().fg(Color::Black).bg(Color::White);

    term.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let text = str_into_spans_styled_with_cursor(text.as_ref(), gap_start, style, cursor_style);

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

pub fn str_into_spans_styled_with_cursor<'a>(
    text: &'a str,
    gap_start: usize,
    style: Style,
    cursor_style: Style,
) -> Vec<Spans<'a>> {
    if gap_start == 0 {
        if text.is_empty() {
            vec![Spans::from(Span::styled(' '.to_string(), cursor_style))]
        } else {
            str_into_spans_styled(text, style)
        }
    } else {
        let text_string = text.chars().collect::<Vec<_>>();
        let (text_before, text_on, text_after) = (
            text_string[..gap_start - 1].iter().collect::<String>(),
            text_string[gap_start - 1],
            text_string[gap_start..].iter().collect::<String>(),
        );

        let mut styled_before = text_before
            .split_terminator('\n')
            .map(ToString::to_string)
            .map(|s| Spans::from(Span::styled(s, style)))
            .collect::<Vec<_>>();

        let styled_on = Span::styled(text_on.to_string(), cursor_style);

        let len = styled_before.len();
        if len > 0 {
            styled_before.get_mut(len - 1).unwrap().0.push(styled_on);
        } else {
            styled_before = vec![Spans::from(styled_on)];
        }

        let mut styled_after = text_after
            .split_terminator('\n')
            .map(ToString::to_string)
            .map(|s| Span::styled(s, style))
            .collect::<Vec<_>>();

        let len = styled_before.len();
        if len > 0 {
            styled_before.get_mut(len - 1).unwrap().0.append(&mut styled_after);
        }

        styled_before
    }
}

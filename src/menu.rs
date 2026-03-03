use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

use crate::{KbtError, KeyboardLang, KeyboardSize, MenuResult};

struct MenuState<T> {
    selections: Vec<T>,
    cursor: usize,
}

enum SelectionResult<T> {
    Selected(T),
    Back,
    Terminate,
}

pub fn run_menu<B: Backend>(terminal: &mut Terminal<B>) -> Result<MenuResult, KbtError> {
    let sizes = vec![
        KeyboardSize::Keyboard60,
        KeyboardSize::Keyboard80,
        KeyboardSize::Keyboard100,
    ];
    let langs = vec![KeyboardLang::US, KeyboardLang::DE];

    loop {
        let size = match run_selection(terminal, "kbt", &sizes, false)? {
            SelectionResult::Selected(s) => s,
            SelectionResult::Terminate | SelectionResult::Back => {
                return Ok(MenuResult::Terminate)
            }
        };

        match run_selection(terminal, "layout", &langs, true)? {
            SelectionResult::Selected(l) => {
                return Ok(MenuResult::KeyboardSelected(size, l));
            }
            SelectionResult::Back => continue,
            SelectionResult::Terminate => return Ok(MenuResult::Terminate),
        }
    }
}

fn run_selection<B: Backend, T: ToString + Clone>(
    terminal: &mut Terminal<B>,
    title: &str,
    items: &[T],
    allow_back: bool,
) -> Result<SelectionResult<T>, KbtError> {
    let mut state = MenuState {
        selections: items.to_vec(),
        cursor: 0,
    };
    let max_idx = state.selections.len() - 1;

    loop {
        terminal.draw(|f| view_menu(f, title, &state).expect("Failed to draw menu"))?;

        if let Event::Key(key) = event::read()? {
            match (key.kind, key.code) {
                (KeyEventKind::Press, KeyCode::Up | KeyCode::Char('k')) => {
                    state.cursor = if state.cursor == 0 {
                        max_idx
                    } else {
                        state.cursor - 1
                    }
                }
                (KeyEventKind::Press, KeyCode::Down | KeyCode::Char('j')) => {
                    state.cursor = if state.cursor == max_idx {
                        0
                    } else {
                        state.cursor + 1
                    }
                }
                (KeyEventKind::Press, KeyCode::Enter) => {
                    return Ok(SelectionResult::Selected(
                        state
                            .selections
                            .get(state.cursor)
                            .ok_or(KbtError {
                                message: String::from("Failed to get a menu selection by idx"),
                            })?
                            .clone(),
                    ))
                }
                (KeyEventKind::Press, KeyCode::Esc) => {
                    if allow_back {
                        return Ok(SelectionResult::Back);
                    }
                }
                (KeyEventKind::Press, KeyCode::Char('c') | KeyCode::Char('q')) => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        return Ok(SelectionResult::Terminate);
                    }
                }
                _ => {}
            }
        }
    }
}

fn view_menu<T: ToString>(
    frame: &mut Frame,
    title: &str,
    state: &MenuState<T>,
) -> Result<(), KbtError> {
    let items: Vec<ListItem> = state
        .selections
        .iter()
        .map(|selection| ListItem::new(selection.to_string()))
        .collect();

    let item_count = items.len() as u16;

    let list = List::new(items)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(state.cursor));

    let terminal_size: Rect = frame.size();

    let layout_height: u16 = 2 + item_count;
    let layout_width: u16 = 15;
    let left_padding: u16 = (terminal_size.width / 2) - (layout_width / 2);
    let top_padding: u16 = (terminal_size.height / 2) - (layout_height / 2);

    let rect = Rect::new(left_padding, top_padding, layout_width, layout_height);

    let layout_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(item_count)].as_ref())
        .split(rect);

    let title_widget = Paragraph::new(title).style(
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::ITALIC),
    );

    frame.render_widget(
        title_widget,
        *layout_chunks.get(0).ok_or(KbtError {
            message: String::from("Failed to get correct layout chunk for title"),
        })?,
    );

    frame.render_stateful_widget(
        list,
        *layout_chunks.get(1).ok_or(KbtError {
            message: String::from("Failed to get correct layout chunk for list"),
        })?,
        &mut list_state,
    );

    Ok(())
}

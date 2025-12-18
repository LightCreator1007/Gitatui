use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{App, Focus, theme};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(f.size());

    draw_dashboard(f, app, chunks[0]);
    draw_footer(f, app, chunks[1]);
}

fn draw_dashboard(f: &mut Frame, app: &App, area: Rect) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Branches
            Constraint::Percentage(50), // Commits
            Constraint::Percentage(30), // Staged
        ])
        .split(area);

    let branches: Vec<ListItem> = app
        .branches
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let style = if i == app.branch_idx {
                Style::default()
                    .bg(theme::GREEN)
                    .fg(theme::WHITE)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme::FG)
            };
            ListItem::new(Line::from(b.as_str())).style(style)
        })
        .collect();

    f.render_widget(
        List::new(branches).block(create_block(
            " Branches ",
            &app.current_focus,
            Focus::Branches,
        )),
        columns[0],
    );

    let commits: Vec<ListItem> = app
        .commits
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let style = if i == app.commit_idx {
                Style::default()
                    .bg(theme::ORANGE)
                    .fg(theme::WHITE)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme::FG)
            };
            ListItem::new(Line::from(c.as_str())).style(style)
        })
        .collect();

    f.render_widget(
        List::new(commits).block(create_block(
            " Commits ",
            &app.current_focus,
            Focus::Commits,
        )),
        columns[1],
    );

    let staged: Vec<ListItem> = app
        .staged_files
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let style = if i == app.staged_idx {
                Style::default()
                    .bg(theme::RED)
                    .fg(theme::WHITE)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme::GREEN)
            };
            ListItem::new(Line::from(s.to_string_lossy().into_owned())).style(style)
        })
        .collect();

    f.render_widget(
        List::new(staged).block(create_block(" Staged ", &app.current_focus, Focus::Staged)),
        columns[2],
    );
}

fn create_block<'a>(title: &'a str, current_focus: &Focus, target_focus: Focus) -> Block<'a> {
    let border_color = if *current_focus == target_focus {
        theme::SELECTION
    } else {
        theme::COMMENT
    };

    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .title(title)
}

fn draw_footer(f: &mut Frame, _app: &App, area: Rect) {
    let footer = Paragraph::new(" [h/l] Nav • [j/k] Scroll • [Ctrl+C] Quit ")
        .style(Style::default().fg(theme::BG).bg(theme::FG));
    f.render_widget(footer, area);
}

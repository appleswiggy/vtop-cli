use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

use crate::app::App;

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let parent_layout = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(rect.size());

    draw_tabs(rect, app, parent_layout[0]);
}

pub fn draw_tabs<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let mut before_active_tab = String::from("|");
    let mut after_active_tab = String::from("|");

    for i in 1..app.state.tabs.len() + 1 {
        if i < app.state.active_tab + 1 {
            before_active_tab += &format!(" Tab {} |", i);
        } else if i > app.state.active_tab + 1 {
            after_active_tab += &format!(" Tab {} |", i);
        }
    }

    let mut spans = vec![
        Span::raw(before_active_tab),
        Span::styled(
            format!(" Tab {} ", app.state.active_tab + 1),
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(after_active_tab),
    ];

    if app.state.tabs.len() < 10 {
        spans.push(Span::styled(
            " +",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        ));
    }

    let tabs_text = vec![Spans::from(spans)];

    let tabs = Paragraph::new(tabs_text)
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Left);

    f.render_widget(tabs, layout_chunk);
}

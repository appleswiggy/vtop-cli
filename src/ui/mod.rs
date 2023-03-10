use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

use crate::app::App;
use crate::pages::Page;
use crate::util::{MAXIMUM_TABS, NOTIFICATION_SEPERATOR, NOTIFICATION_TIMEOUT_SECS};

const TABS_BLOCK_INDEX: usize = 0;
const PAGE_BLOCK_INDEX: usize = 1;
const NOTIFICATION_BLOCK_INDEX: usize = 2;

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

    if let Page::Debug { .. } = app.state.tabs[app.state.active_tab].state.page {
        draw_debug_block(rect, app, parent_layout[1]);
        draw_debug(rect, app, parent_layout[1]);
    }

    draw_notifications_footer(rect, app, parent_layout[2]);
}

pub fn draw_tabs<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let mut before_active_tab = String::from("|");
    let mut after_active_tab = String::from("|");

    for i in 0..app.state.tabs.len() {
        if i < app.state.active_tab {
            before_active_tab += &format!(" {} |", app.state.tabs[i].title.clone());
        } else if i > app.state.active_tab {
            after_active_tab += &format!(" {} |", app.state.tabs[i].title.clone());
        }
    }

    let mut spans = vec![
        Span::raw(before_active_tab),
        Span::styled(
            format!(" {} ", app.state.tabs[app.state.active_tab].title.clone()),
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(after_active_tab),
    ];

    if app.state.tabs.len() < MAXIMUM_TABS {
        spans.push(Span::styled(
            " +",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        ));
    }

    let tabs_text = Spans::from(spans);

    let tabs = Paragraph::new(tabs_text)
        .block(
            Block::default().borders(Borders::ALL).border_style(
                if let Some(TABS_BLOCK_INDEX) = app.state.tabs[app.state.active_tab]
                    .state
                    .blocks
                    .active_block
                {
                    Style::default().fg(Color::LightCyan)
                } else if let Some(TABS_BLOCK_INDEX) = app.state.tabs[app.state.active_tab]
                    .state
                    .blocks
                    .hovered_block
                {
                    Style::default().fg(Color::Magenta)
                } else {
                    Style::default()
                },
            ),
        )
        .alignment(tui::layout::Alignment::Left);

    f.render_widget(tabs, layout_chunk);
}

pub fn draw_notifications_footer<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let footer_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " Notifications ",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        ))
        .border_style(
            if let Some(NOTIFICATION_BLOCK_INDEX) = app.state.tabs[app.state.active_tab]
                .state
                .blocks
                .active_block
            {
                Style::default().fg(Color::LightCyan)
            } else if let Some(NOTIFICATION_BLOCK_INDEX) = app.state.tabs[app.state.active_tab]
                .state
                .blocks
                .hovered_block
            {
                Style::default().fg(Color::Magenta)
            } else {
                Style::default()
            },
        );

    let footer_paragraph = if app.state.notifications.len() == 0
        || app.state.notifications[app.state.notifications.len() - 1]
            .origin_time
            .elapsed()
            .as_secs() as usize
            > NOTIFICATION_TIMEOUT_SECS
    {
        let spans = vec![
            Span::raw("Made with "),
            Span::styled(
                "♥",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" by Chirag Madaan"),
        ];

        Paragraph::new(Spans::from(spans))
            .block(footer_block)
            .alignment(tui::layout::Alignment::Center)
    } else {
        let text = app.state.notifications[app.state.notifications.len() - 1]
            .text
            .clone();

        let mut footer_text = String::new();
        let seperator = NOTIFICATION_SEPERATOR;

        // Two character width is occupied by borders hence substracting 2
        if text.len() > (layout_chunk.width - 2).into() {
            footer_text += &(text
                .chars()
                .skip(app.state.notification_scroll)
                .collect::<String>());

            if app.state.notification_scroll > text.len() {
                footer_text += &(seperator
                    .chars()
                    .skip(app.state.notification_scroll - text.len())
                    .collect::<String>());
            } else {
                footer_text += &seperator;
            }

            footer_text += &text;

            footer_text = footer_text
                .chars()
                .take((layout_chunk.width - 2).into())
                .collect::<String>();
        } else {
            footer_text += &text;
        }

        Paragraph::new(Text::from(footer_text))
            .block(footer_block)
            .alignment(tui::layout::Alignment::Center)
    };

    f.render_widget(footer_paragraph, layout_chunk);
}

pub fn draw_debug<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let layout = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
        .margin(2)
        .split(layout_chunk);

    let mut v = vec![];

    for i in (0..app.state.notifications.len()).rev() {
        v.push(Spans::from(app.state.notifications[i].text.clone()));
    }

    let debug_paragraph = Paragraph::new(v)
        .alignment(tui::layout::Alignment::Center)
        .block(
            Block::default()
                .title(" Notifications ")
                .borders(Borders::ALL)
                .border_style(
                    if let Page::Debug {
                        interactive_blocks: n,
                        active_block: ab,
                        hovered_block: hb,
                    } = app.state.tabs[app.state.active_tab].state.page
                    {
                        if let Some(0) = ab {
                            Style::default().fg(Color::LightCyan)
                        } else if let Some(0) = hb {
                            Style::default().fg(Color::Magenta)
                        } else {
                            Style::default()
                        }
                    } else {
                        Style::default()
                    },
                ),
        );

    f.render_widget(debug_paragraph, layout[0]);

    let temp_block = Block::default()
        .title(" Temp block ")
        .borders(Borders::ALL)
        .border_style(
            if let Page::Debug {
                interactive_blocks: _,
                active_block: ab,
                hovered_block: hb,
            } = app.state.tabs[app.state.active_tab].state.page
            {
                if let Some(1) = ab {
                    Style::default().fg(Color::LightCyan)
                } else if let Some(1) = hb {
                    Style::default().fg(Color::Magenta)
                } else {
                    Style::default()
                }
            } else {
                Style::default()
            },
        );

    f.render_widget(temp_block, layout[1]);
}

pub fn draw_debug_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let styled_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " DEBUG WINDOW ",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        ))
        .border_style(
            if let Some(PAGE_BLOCK_INDEX) = app.state.tabs[app.state.active_tab]
                .state
                .blocks
                .active_block
            {
                Style::default().fg(Color::LightCyan)
            } else if let Some(PAGE_BLOCK_INDEX) = app.state.tabs[app.state.active_tab]
                .state
                .blocks
                .hovered_block
            {
                Style::default().fg(Color::Magenta)
            } else {
                Style::default()
            },
        );

    f.render_widget(styled_block, layout_chunk);
}

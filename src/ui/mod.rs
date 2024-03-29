use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use tui::Frame;

use crate::app::App;
use crate::pages::Page;
use crate::state::Window;
use crate::util::{MAXIMUM_TABS, NOTIFICATION_SEPERATOR, NOTIFICATION_TIMEOUT_SECS};

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
        .split(rect.size());

    draw_tabs(rect, app, parent_layout[0]);

    let body_layout = Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(1)].as_ref())
        .split(parent_layout[1]);

    draw_sidebar(rect, app, body_layout[0]);

    draw_page_window_block(rect, app, body_layout[1]);

    app.state.tabs[app.state.selected_tab]
        .state
        .page_block
        .draw_page(rect, app, body_layout[1]);

    draw_notifications_footer(rect, app, parent_layout[2]);
}

pub fn draw_tabs<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let mut before_selected_tab = String::from("|");
    let mut after_selected_tab = String::from("|");

    for i in 0..app.state.tabs.len() {
        if i < app.state.selected_tab {
            before_selected_tab += &format!(" {} |", app.state.tabs[i].title.clone());
        } else if i > app.state.selected_tab {
            after_selected_tab += &format!(" {} |", app.state.tabs[i].title.clone());
        }
    }

    let mut spans = vec![
        Span::raw(before_selected_tab),
        Span::styled(
            format!(" {} ", app.state.tabs[app.state.selected_tab].title.clone()),
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(after_selected_tab),
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
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Left);

    f.render_widget(tabs, layout_chunk);
}

pub fn draw_sidebar<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let sidebar_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " Sidebar ",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        ))
        .border_style(
            if let Some(Window::SidebarWindow) =
                app.state.tabs[app.state.selected_tab].state.active_window
            {
                Style::default().fg(Color::LightCyan)
            } else if let Some(Window::SidebarWindow) =
                app.state.tabs[app.state.selected_tab].state.hovered_window
            {
                Style::default().fg(Color::Magenta)
            } else {
                Style::default()
            },
        );

    let mut items = Page::iterator()
        .map(|page| ListItem::new(page.to_string()))
        .collect::<Vec<ListItem>>();

    items[app.state.tabs[app.state.selected_tab].state.sidebar_hover] = items
        [app.state.tabs[app.state.selected_tab].state.sidebar_hover]
        .clone()
        .style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        );

    let sidebar_list = List::new(items)
        .block(sidebar_block)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let active_page = app.state.tabs[app.state.selected_tab].state.page_block.page;

    let active_page_index = Page::iterator()
        .position(|page| page.to_string() == active_page.to_string())
        .unwrap();

    let mut list_state = ListState::default();
    list_state.select(Some(active_page_index));

    f.render_stateful_widget(sidebar_list, layout_chunk, &mut list_state);
}

pub fn draw_page_window_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let page_title = app.state.tabs[app.state.selected_tab]
        .state
        .page_block
        .page
        .to_string();

    let page_window_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            format!(" {} ", page_title),
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        ))
        .border_style(
            if let Some(Window::PageWindow) =
                app.state.tabs[app.state.selected_tab].state.active_window
            {
                Style::default().fg(Color::LightCyan)
            } else if let Some(Window::PageWindow) =
                app.state.tabs[app.state.selected_tab].state.hovered_window
            {
                Style::default().fg(Color::Magenta)
            } else {
                Style::default()
            },
        );

    f.render_widget(page_window_block, layout_chunk)
}

pub fn draw_notifications_footer<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let footer_block = Block::default().borders(Borders::ALL).title(Span::styled(
        " Notifications ",
        Style::default()
            .fg(Color::LightCyan)
            .add_modifier(Modifier::BOLD),
    ));

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

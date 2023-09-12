use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::Paragraph,
    Frame,
};

use crate::app::App;

use super::{Block, BlockContent, BlockType};

pub struct DebugPage {}

impl DebugPage {
    pub fn fill_inner_blocks(block: &mut Block) {
        let mut temp1 = Block::default("Notifications".to_string(), BlockType::ContainerBlock);
        let mut temp3 = Block::default("Temp 3".to_string(), BlockType::ContainerBlock);
        let mut temp4 = Block::default("Temp 4".to_string(), BlockType::ContainerBlock);

        let temp5 = Block::default("Temp 5".to_string(), BlockType::ParagraphBlock);
        let temp6 = Block::default("Temp 6".to_string(), BlockType::ParagraphBlock);
        let temp7 = Block::default("Temp 7".to_string(), BlockType::ParagraphBlock);

        temp3.append_inner_block(temp5).unwrap();
        temp3.append_inner_block(temp6).unwrap();
        temp4.append_inner_block(temp7).unwrap();

        temp1.append_inner_block(temp3).unwrap();
        temp1.append_inner_block(temp4).unwrap();

        block.append_inner_block(temp1).unwrap();

        let temp2 = Block::default("Temp Block".to_string(), BlockType::ParagraphBlock);
        block.append_inner_block(temp2).unwrap();
    }

    pub fn draw<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
        B: Backend,
    {
        let block_self = &app.state.tabs[app.state.selected_tab]
            .state
            .page_block
            .block;

        let layout = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
            .margin(2)
            .split(layout_chunk);

        let mut v = vec![];

        for i in (0..app.state.notifications.len()).rev() {
            v.push(Spans::from(app.state.notifications[i].text.clone()));
        }

        if let BlockContent::ContainerBlock {
            inner_blocks,
            selected_block,
            hovered_block,
        } = &block_self.content
        {
            let notifications_block = tui::widgets::Block::default()
                .title(Span::styled(
                    format!(" {} ", inner_blocks[0].title()),
                    Style::default()
                        .fg(Color::LightCyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(inner_blocks[0].border())
                .border_style(if let Some(0) = selected_block {
                    Style::default().fg(Color::LightCyan)
                } else if let Some(0) = hovered_block {
                    Style::default().fg(Color::Magenta)
                } else {
                    Style::default()
                });

            f.render_widget(notifications_block, layout[0]);

            let notifications_layout = Layout::default()
                .direction(tui::layout::Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(7)].as_ref())
                .margin(2)
                .split(layout[0]);

            if let BlockContent::ContainerBlock {
                inner_blocks,
                selected_block,
                hovered_block,
            } = &inner_blocks[0].content
            {
                let temp3block = tui::widgets::Block::default()
                    .title(Span::styled(
                        format!(" {} ", inner_blocks[0].title()),
                        Style::default()
                            .fg(Color::LightCyan)
                            .add_modifier(Modifier::BOLD),
                    ))
                    .borders(inner_blocks[0].border())
                    .border_style(if let Some(0) = selected_block {
                        Style::default().fg(Color::LightCyan)
                    } else if let Some(0) = hovered_block {
                        Style::default().fg(Color::Magenta)
                    } else {
                        Style::default()
                    });

                f.render_widget(temp3block, notifications_layout[0]);

                let temp4block = tui::widgets::Block::default()
                    .title(Span::styled(
                        format!(" {} ", inner_blocks[1].title()),
                        Style::default()
                            .fg(Color::LightCyan)
                            .add_modifier(Modifier::BOLD),
                    ))
                    .borders(inner_blocks[1].border())
                    .border_style(if let Some(1) = selected_block {
                        Style::default().fg(Color::LightCyan)
                    } else if let Some(1) = hovered_block {
                        Style::default().fg(Color::Magenta)
                    } else {
                        Style::default()
                    });

                f.render_widget(temp4block, notifications_layout[1]);

                let temp3_layout = Layout::default()
                    .direction(tui::layout::Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .margin(2)
                    .split(notifications_layout[0]);

                let temp4_layout = Layout::default()
                    .direction(tui::layout::Direction::Vertical)
                    .constraints([Constraint::Length(10)].as_ref())
                    .margin(2)
                    .split(notifications_layout[1]);

                if let BlockContent::ContainerBlock {
                    inner_blocks,
                    selected_block,
                    hovered_block,
                } = &inner_blocks[0].content
                {
                    let temp5 = Paragraph::new(v)
                        .alignment(tui::layout::Alignment::Center)
                        .block(
                            tui::widgets::Block::default()
                                .title(Span::styled(
                                    format!(" {} ", inner_blocks[0].title()),
                                    Style::default()
                                        .fg(Color::LightCyan)
                                        .add_modifier(Modifier::BOLD),
                                ))
                                .borders(inner_blocks[0].border())
                                .border_style(if let Some(0) = selected_block {
                                    Style::default().fg(Color::LightCyan)
                                } else if let Some(0) = hovered_block {
                                    Style::default().fg(Color::Magenta)
                                } else {
                                    Style::default()
                                }),
                        );

                    f.render_widget(temp5, temp3_layout[0]);

                    let temp6 = Paragraph::new("Hello, World!")
                        .alignment(tui::layout::Alignment::Center)
                        .block(
                            tui::widgets::Block::default()
                                .title(Span::styled(
                                    format!(" {} ", inner_blocks[1].title()),
                                    Style::default()
                                        .fg(Color::LightCyan)
                                        .add_modifier(Modifier::BOLD),
                                ))
                                .borders(inner_blocks[1].border())
                                .border_style(if let Some(1) = selected_block {
                                    Style::default().fg(Color::LightCyan)
                                } else if let Some(1) = hovered_block {
                                    Style::default().fg(Color::Magenta)
                                } else {
                                    Style::default()
                                }),
                        );

                    f.render_widget(temp6, temp3_layout[1]);
                }

                if let BlockContent::ContainerBlock {
                    inner_blocks,
                    selected_block,
                    hovered_block,
                } = &inner_blocks[1].content
                {
                    let temp7 = Paragraph::new("Tuition Framework")
                        .alignment(tui::layout::Alignment::Center)
                        .block(
                            tui::widgets::Block::default()
                                .title(Span::styled(
                                    format!(" {} ", inner_blocks[0].title()),
                                    Style::default()
                                        .fg(Color::LightCyan)
                                        .add_modifier(Modifier::BOLD),
                                ))
                                .borders(inner_blocks[0].border())
                                .border_style(if let Some(0) = selected_block {
                                    Style::default().fg(Color::LightCyan)
                                } else if let Some(0) = hovered_block {
                                    Style::default().fg(Color::Magenta)
                                } else {
                                    Style::default()
                                }),
                        );

                    f.render_widget(temp7, temp4_layout[0]);
                }
            }

            let temp_block = Paragraph::new("Test Block")
                .alignment(tui::layout::Alignment::Center)
                .block(
                    tui::widgets::Block::default()
                        .title(Span::styled(
                            format!(" {} ", inner_blocks[1].title()),
                            Style::default()
                                .fg(Color::LightCyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(inner_blocks[1].border())
                        .border_style(if let Some(1) = selected_block {
                            Style::default().fg(Color::LightCyan)
                        } else if let Some(1) = hovered_block {
                            Style::default().fg(Color::Magenta)
                        } else {
                            Style::default()
                        }),
                );

            f.render_widget(temp_block, layout[1]);
        }
    }
}

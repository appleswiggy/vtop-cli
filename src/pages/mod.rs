use std::{
    fmt::{self, Display},
    slice::Iter,
};

use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Borders, Paragraph},
    Frame,
};

use crate::{app::App, input::Key};

#[derive(Copy, Clone)]
pub enum Page {
    Home,
    Spotlight,
    FacultyInfo,
    ClassMessages,
    TimeTable,
    ClassAttendance,
    CoursePage,
    Marks,
    Grades,
    GradeHistory,
    RoomInformation,
    Debug,
}

impl Page {
    pub fn iterator() -> Iter<'static, Page> {
        static PAGES: [Page; 12] = [
            Page::Home,
            Page::Spotlight,
            Page::FacultyInfo,
            Page::ClassMessages,
            Page::TimeTable,
            Page::ClassAttendance,
            Page::CoursePage,
            Page::Marks,
            Page::Grades,
            Page::GradeHistory,
            Page::RoomInformation,
            Page::Debug,
        ];
        PAGES.iter()
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Page::Home => "Home",
            Page::Spotlight => "Spotlight",
            Page::FacultyInfo => "Faculty Info",
            Page::ClassMessages => "Class Messages",
            Page::TimeTable => "Time Table",
            Page::ClassAttendance => "Class Attendance",
            Page::CoursePage => "Course Page",
            Page::Marks => "Marks",
            Page::Grades => "Grades",
            Page::GradeHistory => "Grade History",
            Page::RoomInformation => "Room Information",
            Page::Debug => "Debug Page",
        };
        write!(f, "{}", str)
    }
}

pub struct PageBlock {
    pub page: Page,
    pub block: Block,
}

impl PageBlock {
    pub fn fill_inner_blocks(mut self) -> Self {
        match self.page {
            Page::Debug => DebugPage::fill_inner_blocks(&mut self.block),
            _ => (),
        }

        self
    }
}

pub struct DebugPage {}

impl DebugPage {
    pub fn fill_inner_blocks(block: &mut Block) {
        let temp1 = Block::default("Notifications".to_string(), BlockType::ParagraphBlock);
        block.append_inner_block(temp1).unwrap();

        let temp2 = Block::default("Temp Block".to_string(), BlockType::ParagraphBlock);
        block.append_inner_block(temp2).unwrap();
    }

    pub fn draw<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
        B: Backend,
    {
        let page_block = &app.state.tabs[app.state.active_tab].state.page_block.block;

        let outer_block = tui::widgets::Block::default()
            .borders(page_block.border())
            .title(Span::styled(
                format!(" {} ", page_block.title()),
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD),
            ))
            .border_style(match page_block.border {
                BlockBorder::NoBorder => Style::default(),
                BlockBorder::Border {
                    is_active,
                    is_highlighted,
                } => {
                    if is_active {
                        Style::default().fg(Color::LightCyan)
                    } else if is_highlighted {
                        Style::default().fg(Color::Magenta)
                    } else {
                        Style::default()
                    }
                }
            });

        f.render_widget(outer_block, layout_chunk);

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
            active_block,
            hovered_block,
        } = &page_block.content
        {
            let notification_history = Paragraph::new(v)
                .alignment(tui::layout::Alignment::Center)
                .block(
                    tui::widgets::Block::default()
                        .title(format!(" {} ", inner_blocks[0].title()))
                        .borders(inner_blocks[0].border())
                        .border_style(if let Some(0) = active_block {
                            Style::default().fg(Color::LightCyan)
                        } else if let Some(0) = hovered_block {
                            Style::default()
                        } else {
                            Style::default()
                        }),
                );

            f.render_widget(notification_history, layout[0]);

            let temp_block = tui::widgets::Block::default()
                .title(format!(" {} ", inner_blocks[1].title()))
                .borders(inner_blocks[1].border())
                .border_style(if let Some(1) = active_block {
                    Style::default().fg(Color::LightCyan)
                } else if let Some(1) = hovered_block {
                    Style::default()
                } else {
                    Style::default()
                });

            f.render_widget(temp_block, layout[1]);
        }
    }
}

pub enum BlockType {
    InputBlock,
    ParagraphBlock,
    Button,
    ContainerBlock,
}

pub enum BlockContent {
    InputBlock {
        input_text: String,
    },
    ParagraphBlock {
        text: String,
    },
    Button,
    ContainerBlock {
        inner_blocks: Vec<Block>,
        active_block: Option<usize>,
        hovered_block: Option<usize>,
    },
}

pub enum BlockBorder {
    NoBorder,
    Border {
        is_active: bool,
        is_highlighted: bool,
    },
}

pub enum BlockTitle {
    NoTitle,
    Title(String),
}

pub struct Block {
    pub title: BlockTitle,
    pub border: BlockBorder,
    pub content: BlockContent,
}

impl Block {
    pub fn default_raw(block_type: BlockType) -> Block {
        Block {
            title: BlockTitle::NoTitle,
            border: BlockBorder::NoBorder,
            content: match block_type {
                BlockType::InputBlock => BlockContent::InputBlock {
                    input_text: String::new(),
                },
                BlockType::ParagraphBlock => BlockContent::ParagraphBlock {
                    text: String::new(),
                },
                BlockType::Button => BlockContent::Button,
                BlockType::ContainerBlock => BlockContent::ContainerBlock {
                    inner_blocks: vec![],
                    active_block: None,
                    hovered_block: None,
                },
            },
        }
    }

    pub fn default(title: String, block_type: BlockType) -> Block {
        let mut block = Block::default_raw(block_type);
        block.add_title(title);
        block.add_border();

        return block;
    }

    pub fn add_title(&mut self, title: String) {
        self.title = BlockTitle::Title(title);
    }

    pub fn title(&self) -> String {
        match &self.title {
            BlockTitle::NoTitle => "".to_string(),
            BlockTitle::Title(title) => title.clone(),
        }
    }

    pub fn add_border(&mut self) {
        self.border = BlockBorder::Border {
            is_active: false,
            is_highlighted: false,
        };
    }

    pub fn border(&self) -> Borders {
        match &self.border {
            BlockBorder::NoBorder => Borders::NONE,
            BlockBorder::Border { .. } => Borders::ALL,
        }
    }

    pub fn append_inner_block(&mut self, block: Block) -> Result<(), &str> {
        if let BlockContent::ContainerBlock {
            ref mut inner_blocks,
            ..
        } = self.content
        {
            inner_blocks.push(block);
            Ok(())
        } else {
            Err("Can only append blocks to container blocks.")
        }
    }

    pub fn hovered_block_right(&mut self) {
        if let BlockContent::ContainerBlock {
            ref inner_blocks,
            ref mut hovered_block,
            ..
        } = self.content
        {
            if let Some(index) = *hovered_block {
                *hovered_block = Some((index + 1) % inner_blocks.len());
            }
        }
    }

    pub fn hovered_block_left(&mut self) {
        if let BlockContent::ContainerBlock {
            ref inner_blocks,
            ref mut hovered_block,
            ..
        } = self.content
        {
            if let Some(index) = *hovered_block {
                if index == 0 {
                    *hovered_block = Some(inner_blocks.len() - 1);
                } else {
                    *hovered_block = Some(index - 1);
                }
            }
        }
    }

    pub fn handle_input(&mut self, _key: Key) {}
}

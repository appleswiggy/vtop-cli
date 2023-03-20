use std::{
    fmt::{self, Display},
    slice::Iter,
};

use tui::widgets::Borders;

use crate::input::Key;
pub use debug::DebugPage;

mod debug;

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
    pub fn init_page(mut self) -> Self {
        // fill inner blocks
        match self.page {
            Page::Debug => DebugPage::fill_inner_blocks(&mut self.block),
            _ => (),
        }

        self.block.select_border();
        self.block.hover_first_block();

        return self;
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
        selected_block: Option<usize>,
        hovered_block: Option<usize>,
    },
}

pub enum BlockBorder {
    NoBorder,
    Border {
        is_selected: bool,
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
                    selected_block: None,
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
            is_selected: false,
            is_highlighted: false,
        };
    }

    pub fn select_border(&mut self) {
        if let BlockBorder::Border {
            ref mut is_selected,
            ..
        } = self.border
        {
            *is_selected = true;
        }
    }

    pub fn highlight_border(&mut self) {
        if let BlockBorder::Border {
            ref mut is_highlighted,
            ..
        } = self.border
        {
            *is_highlighted = true;
        }
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

    pub fn get_inner_blocks_mut(&mut self) -> Result<&mut Vec<Block>, &str> {
        if let BlockContent::ContainerBlock {
            ref mut inner_blocks,
            ..
        } = self.content
        {
            Ok(inner_blocks)
        } else {
            Err("Can only get inner blocks from container blocks.")
        }
    }

    pub fn get_inner_blocks(&self) -> Result<&Vec<Block>, &str> {
        if let BlockContent::ContainerBlock {
            ref inner_blocks, ..
        } = self.content
        {
            Ok(inner_blocks)
        } else {
            Err("Can only get inner blocks from container blocks.")
        }
    }

    pub fn hover_first_block(&mut self) {
        if let BlockContent::ContainerBlock {
            ref inner_blocks,
            ref mut hovered_block,
            ..
        } = self.content
        {
            if inner_blocks.len() != 0 {
                *hovered_block = Some(0);
            }
        }
    }

    pub fn hover_block_right(&mut self) {
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

    pub fn hover_block_left(&mut self) {
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

    pub fn deselect(&mut self) {
        if let BlockContent::ContainerBlock {
            ref mut inner_blocks,
            ref mut selected_block,
            ref mut hovered_block,
        } = self.content
        {
            *selected_block = None;
            *hovered_block = None;

            for block in inner_blocks {
                block.deselect();
            }
        }
    }

    pub fn has_selected_child(&mut self) -> bool {
        if let BlockContent::ContainerBlock { selected_block, .. } = self.content {
            return selected_block.is_some();
        }

        return false;
    }

    pub fn handle_input(&mut self, key: Key) {
        if let BlockContent::ContainerBlock {
            ref mut inner_blocks,
            ref mut selected_block,
            ref mut hovered_block,
        } = self.content
        {
            // what if innerblocks.len() is 0
            if hovered_block.is_none() {
                *hovered_block = Some(0);
            }

            if selected_block.is_none() {
                if let Key::Tab | Key::Right | Key::Down = key {
                    self.hover_block_right();
                } else if let Key::ShiftTab | Key::Left | Key::Up = key {
                    self.hover_block_left();
                } else if let Key::Enter = key {
                    *selected_block = Some(hovered_block.unwrap());
                    inner_blocks[selected_block.unwrap()].hover_first_block();
                }
                // }
            } else {
                let mut flag = false;
                if let Key::Esc = key {
                    if inner_blocks[selected_block.unwrap()].has_selected_child() == false {
                        inner_blocks[selected_block.unwrap()].deselect();
                        *selected_block = None;
                        flag = true;
                    }
                }
                if flag == false {
                    inner_blocks[selected_block.unwrap()].handle_input(key);
                }
            }
        }
    }
}

use std::{
    fmt::{self, Display},
    slice::Iter,
};

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
    Debug {
        interactive_blocks: usize,
        active_block: Option<usize>,
        hovered_block: Option<usize>,
    },
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
            Page::Debug {
                interactive_blocks: 2,
                active_block: None,
                hovered_block: Some(0),
            },
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
            Page::Debug { .. } => "Debug Page",
        };
        write!(f, "{}", str)
    }
}

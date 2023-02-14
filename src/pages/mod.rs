use std::{slice::Iter, fmt::{self, Display}};

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
}

impl Page {
    pub fn iterator() -> Iter<'static, Page> {
        static PAGES: [Page; 11] = [
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
        };
        write!(f, "{}", str)
    }
}

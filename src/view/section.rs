use ratatui::layout::Constraint;

use super::ViewSection;

pub struct Section {
    pub view_section: Box<dyn ViewSection>,
    pub constraint: Constraint,
    pub is_selectable: bool,
}

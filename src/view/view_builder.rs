use ratatui::layout::{Constraint, Direction};

use crate::view::{Section, View, ViewSection};

#[derive(Default)]
pub struct ViewBuilder {
    sections: Vec<Section>,
    /// the internally selected section
    selected_section: usize,
    /// whether the entire view is selected
    is_selected: bool,
    direction: Direction,
}

impl<T: ViewSection, const N: usize> From<[T; N]> for ViewBuilder {
    fn from(sections: [T; N]) -> Self {
        Self {
            sections: sections
                .into_iter()
                .map(|v| Section {
                    view_section: Box::new(v) as Box<dyn ViewSection>,
                    constraint: Constraint::Ratio(1, 1),
                    is_selectable: true,
                })
                .collect(),
            selected_section: 0,
            is_selected: false,
            direction: Direction::default(),
        }
    }
}

impl ViewBuilder {
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn select(mut self) -> Self {
        self.is_selected = true;
        self
    }

    pub fn add_selectable(mut self, section: impl ViewSection) -> Self {
        self.sections.push(Section {
            view_section: Box::new(section),
            constraint: Constraint::Ratio(1, 1),
            is_selectable: true,
        });
        self
    }

    pub fn add_selectable_with_constraint(
        mut self,
        section: impl ViewSection,
        constraint: Constraint,
    ) -> Self {
        self.sections.push(Section {
            view_section: Box::new(section),
            constraint,
            is_selectable: true,
        });
        self
    }

    pub fn add_non_selectable(mut self, section: impl ViewSection) -> Self {
        self.sections.push(Section {
            view_section: Box::new(section),
            constraint: Constraint::Ratio(1, 1),
            is_selectable: false,
        });
        self
    }

    pub fn add_non_selectable_with_constraint(
        mut self,
        section: impl ViewSection,
        constraint: Constraint,
    ) -> Self {
        self.sections.push(Section {
            view_section: Box::new(section),
            constraint,
            is_selectable: false,
        });
        self
    }

    pub fn add_sections<I, T>(mut self, sections: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: ViewSection,
    {
        for section in sections.into_iter() {
            self = self.add_selectable(section);
        }
        self
    }

    pub fn build(mut self) -> View {
        if self.selected_section < self.sections.len() {
            self.sections[self.selected_section].view_section.select();
        }

        View {
            sections: self.sections,
            selected_section: self.selected_section,
            last_selected_section: 0,
            is_selected: self.is_selected,
            direction: self.direction,
        }
    }
}

use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget, WidgetRef},
};

use crate::app::model::NotesListState;

pub struct NotesListView<'a> {
    state: &'a NotesListState,
}

impl<'a> NotesListView<'a> {
    pub fn new(state: &'a NotesListState) -> Self {
        Self { state }
    }
}

/// Format a daily note filename like `daily-2026-02-18` into "Tue, Feb 18 2026"
fn format_daily_name(stem: &str) -> Option<String> {
    let date_str = stem.strip_prefix("daily-")?;
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()?;
    Some(date.format("%a, %b %-d %Y").to_string())
}

/// Format a slug like `iteration-42-sprint-name` into "Iteration 42 Sprint Name"
fn format_slug(stem: &str) -> String {
    stem.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    format!("{}{}", upper, chars.as_str())
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn display_name(path: &Path, is_daily: bool) -> String {
    let stem = match path.file_stem().and_then(|s| s.to_str()) {
        Some(s) => s,
        None => return "???".to_string(),
    };

    if is_daily
        && let Some(formatted) = format_daily_name(stem)
    {
        return formatted;
    }

    format_slug(stem)
}

impl<'a> WidgetRef for NotesListView<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        if self.state.daily_notes.is_empty() && self.state.other_notes.is_empty() {
            let block = Block::bordered().border_set(border::THICK);
            let inner = block.inner(area);
            block.render(area, buf);

            let message = "No notes found.";
            let style = Style::default().gray();
            let paragraph = Paragraph::new(message)
                .style(style)
                .alignment(Alignment::Center);

            if inner.height > 0 {
                let centered_area =
                    Rect::new(inner.x, inner.y + inner.height / 2, inner.width, 1);
                paragraph.render(centered_area, buf);
            }
            return;
        }

        let sections: Vec<(&str, &[PathBuf], bool)> = vec![
            ("Daily Notes", &self.state.daily_notes, true),
            ("Iteration Notes", &self.state.other_notes, false),
        ];

        let mut constraints = Vec::new();
        for (_, notes, _) in &sections {
            if notes.is_empty() {
                continue;
            }
            constraints.push(Constraint::Length(1));
            // notes * 2 lines + 2 for border
            constraints.push(Constraint::Length((notes.len() * 2 + 2) as u16));
            constraints.push(Constraint::Length(1));
        }

        if !constraints.is_empty() {
            constraints.pop();
        }
        constraints.push(Constraint::Min(0));

        let section_areas = Layout::vertical(constraints).split(area);

        let mut area_index = 0;
        for (title, notes, is_daily) in &sections {
            if notes.is_empty() {
                continue;
            }

            // Render section header
            let header_area = section_areas[area_index];
            area_index += 1;

            let header_style = Style::default().dark_gray();
            let display = format!(" ── {} ──", title);
            let title_line = Line::from(display).style(header_style);
            buf.set_line(header_area.x, header_area.y, &title_line, header_area.width);

            // Render bordered note items
            let list_area = section_areas[area_index];
            area_index += 1;

            let list_block = Block::bordered().border_set(border::THICK);
            let items_area = list_block.inner(list_area);
            list_block.render(list_area, buf);

            let mut y = items_area.y;
            for note_path in *notes {
                if y + 1 >= items_area.y + items_area.height {
                    break;
                }

                let is_selected = self.state.selected_path.as_ref() == Some(note_path);
                let name = display_name(note_path, *is_daily);

                let name_style = if is_selected {
                    Style::default().bold()
                } else {
                    Style::default()
                };
                let name_line = Line::from(format!("  {}", name)).style(name_style);
                buf.set_line(items_area.x, y, &name_line, items_area.width);
                y += 1;

                if y < items_area.y + items_area.height {
                    let divider_style = if is_selected {
                        Style::default().yellow()
                    } else {
                        Style::default().dark_gray()
                    };
                    let divider = "─".repeat(items_area.width as usize);
                    let divider_line = Line::from(divider).style(divider_style);
                    buf.set_line(items_area.x, y, &divider_line, items_area.width);
                    y += 1;
                }
            }

            // Skip spacing
            area_index += 1;
        }
    }
}

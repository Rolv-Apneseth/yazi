use ratatui::{buffer::Buffer, layout::{self, Constraint, Direction, Rect}, widgets::{Block, Borders, Padding, Widget}};

use super::{Folder, Preview};
use crate::{core::manager::{Mode, ALL_RATIO, CURRENT_RATIO, PARENT_RATIO, PREVIEW_RATIO}, ui::Ctx};

pub struct Layout<'a> {
	cx: &'a Ctx,
}

impl<'a> Layout<'a> {
	pub fn new(cx: &'a Ctx) -> Self { Self { cx } }
}

impl<'a> Widget for Layout<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let manager = &self.cx.manager;

		let chunks = layout::Layout::default()
			.direction(Direction::Horizontal)
			.constraints(
				[
					Constraint::Ratio(PARENT_RATIO, ALL_RATIO),
					Constraint::Ratio(CURRENT_RATIO, ALL_RATIO),
					Constraint::Ratio(PREVIEW_RATIO, ALL_RATIO),
				]
				.as_ref(),
			)
			.split(area);

		// Parent
		let block = Block::default().borders(Borders::RIGHT).padding(Padding::new(1, 0, 0, 0));
		if let Some(ref parent) = manager.parent() {
			Folder::new(self.cx, parent).render(block.inner(chunks[0]), buf);
		}
		block.render(chunks[0], buf);

		// Current
		Folder::new(self.cx, manager.current())
			.with_selection(matches!(manager.active().mode(), Mode::Select(_)))
			.render(chunks[1], buf);

		// Preview
		let block = Block::default().borders(Borders::LEFT).padding(Padding::new(0, 1, 0, 0));
		Preview::new(self.cx).render(block.inner(chunks[2]), buf);
		block.render(chunks[2], buf);
	}
}
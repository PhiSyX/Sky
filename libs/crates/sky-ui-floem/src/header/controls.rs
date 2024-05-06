// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use floem::event::Event;
use floem::reactive::use_context;
use floem::style::CursorStyle;
use floem::view::View;
use floem::views::{h_stack, Decorators};
use floem::{action, window};

use crate::classes::align::gap::Gap8;
use crate::icons::*;
use crate::state::FloemApplicationStateShared;
use crate::variables::*;

// --------- //
// Structure //
// --------- //

pub struct WindowControls
{
	minimize: WindowControlMinimize,
	maximize: WindowControlMaximize,
	close: WindowControlClose,
}

struct WindowControlMinimize;

struct WindowControlMaximize;

struct WindowControlClose(window::WindowId);

// -------------- //
// Implémentation //
// -------------- //

impl WindowControls
{
	pub fn new(window_id: window::WindowId) -> Self
	{
		Self {
			minimize: WindowControlMinimize,
			maximize: WindowControlMaximize,
			close: WindowControlClose(window_id),
		}
	}

	pub fn render(&self) -> impl View
	{
		let change_theme_handler = |_: &Event| {
			let state = use_context::<FloemApplicationStateShared>()
				.expect("État de l'application");
			state.theme_data.toggle();
		};

		h_stack((
			h_stack((
				notification_icon(),
				theme_icon().class(Icon).on_click_cont(change_theme_handler),
			))
			.class(Gap8),
			h_stack((
				self.minimize.render(),
				self.maximize.render(),
				self.close.render(),
			))
			.class(Gap8),
		))
		.style(|style| {
			style
				.gap(space(5), space(5))
				.justify_end()
				.cursor(CursorStyle::Pointer)
		})
	}
}

impl WindowControlMinimize
{
	pub fn render(&self) -> impl View
	{
		let icon = window_minimize_icon().class(Icon);
		self.attach_events(icon)
	}

	fn attach_events(&self, icon: impl View) -> impl View
	{
		icon.on_click_cont(|_| {
			action::minimize_window();
		})
	}
}

impl WindowControlMaximize
{
	pub fn render(&self) -> impl View
	{
		let icon = window_maximize_icon().class(Icon);
		self.attach_events(icon)
	}

	fn attach_events(&self, icon: impl View) -> impl View
	{
		icon.on_click_cont(|_| {
			action::toggle_window_maximized();
		})
	}
}

impl WindowControlClose
{
	pub fn render(&self) -> impl View
	{
		let icon = window_close_icon();
		self.attach_events(icon)
	}

	fn attach_events(&self, icon: impl View) -> impl View
	{
		let wid = self.0;
		icon.on_click_cont(move |_| {
			window::close_window(wid);
		})
	}
}

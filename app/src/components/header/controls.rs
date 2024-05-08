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
use floem::style::CursorStyle;
use floem::view::View;
use floem::views::{h_stack, Decorators};
use floem::{action, reactive, window};

use crate::components::icons::*;
use crate::state::ApplicationStateShared;
use crate::styles::classes::align::gap::*;
use crate::styles::variables::*;

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
		let state: ApplicationStateShared = reactive::use_context() /* dfplz */
			.expect("État de l'application");

		let change_theme_handler = move |_: &Event| {
			state.theme_data.toggle();
		};

		let user_controls = h_stack((
			notification_icon(),
			theme_icon().class(Icon).on_click_cont(change_theme_handler),
		))
		.class(Gap8);

		let sys_controls = h_stack((
			self.minimize.render(),
			self.maximize.render(),
			self.close.render(),
		))
		.class(Gap8);

		h_stack((
			user_controls,
			sys_controls, // don't format please
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

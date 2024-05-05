// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::process::Termination;

use floem::keyboard::{Key, Modifiers, NamedKey};
use floem::view::View;
use floem::views::Decorators;
use floem::window::{self, WindowConfig};
use floem::{reactive, Application};
use sky_ui::{ApplicationAdapter, ApplicationSettings};

use crate::header::title::WindowTitleData;
use crate::window::FloemWindow;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
pub struct FloemApplication
{
	settings: ApplicationSettings,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl ApplicationAdapter for FloemApplication
{
	fn new() -> Self
	{
		Self {
			settings: ApplicationSettings::default(),
		}
	}

	fn define_window_title(&mut self, default_title: impl ToString)
	{
		self.settings.set_title(default_title);
	}

	fn launch(self) -> impl Termination
	{
		let window_settings = WindowConfig::default()
			.size((1440.0, 800.0))
			.title(self.settings.title())
			.show_titlebar(false);

		let window_render = |window_id| {
			// Shared Data

			let window_title_data = WindowTitleData {
				default_title: self.settings.title().to_owned(),
				title: reactive::create_signal(
					self.settings.title().to_owned(),
				),
			};

			let window_title_data_shared = window_title_data.shared();
			floem::reactive::provide_context(window_title_data_shared.clone());

			// Rendu

			let view = FloemWindow::new(self.settings, window_id).view();
			let view_id = view.id();
			view
				// NOTE: inspection
				.on_key_up(
					Key::Named(NamedKey::F11),
					Modifiers::empty(),
					move |_| view_id.inspect(),
				)
				// NOTE: fermeture
				.on_key_up(
					Key::Named(NamedKey::Escape),
					Modifiers::empty(),
					move |_| window::close_window(window_id),
				)
				.on_key_up(
					Key::Character("c".into()),
					Modifiers::CONTROL,
					move |_| window::close_window(window_id),
				)
		};

		Application::new()
			.window(window_render, Some(window_settings))
			.run()
	}
}

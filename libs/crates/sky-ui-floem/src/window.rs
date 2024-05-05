// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::sync::Arc;

use floem::view::View;
use floem::views::{text, v_stack, Decorators};
use floem::window::WindowId;
use sky_ui::ApplicationSettings;

use crate::colors::{COLOR_BLACK, TEXT_COLOR_WHITE};

// --------- //
// Structure //
// --------- //

pub struct FloemWindow
{
	settings: Arc<ApplicationSettings>,
	window_id: WindowId,
}

// -------------- //
// Implémentation //
// -------------- //

impl FloemWindow
{
	pub fn new(settings: ApplicationSettings, window_id: WindowId) -> Self
	{
		let shared_settings = settings.shared();
		Self {
			window_id,
			settings: shared_settings,
		}
	}

	pub fn view(&self) -> impl View
	{
		v_stack((
			text("Test")
		)).style(|style| {
			style
				.size_full()
				.font_family(String::from("Roboto"))
				.font_size(14.0)
				.background(COLOR_BLACK)
				.color(TEXT_COLOR_WHITE)
		})
	}
}

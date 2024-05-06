// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use floem::action::set_window_title;
use floem::reactive::{self, use_context};
use floem::view::Widget;
use floem::views::{container, label, Decorators};

use crate::state::ApplicationStateShared;
use crate::styles::variables::*;

// --------- //
// Structure //
// --------- //

pub struct Title;

// -------------- //
// Implémentation //
// -------------- //

impl Title
{
	pub fn render(&self) -> impl Widget
	{
		let state: ApplicationStateShared =
			use_context().expect("État de l'application");

		let title = state.title_data.read();

		reactive::create_effect(move |_| {
			set_window_title(title.get());
		});

		container(
			label(move || title.get())
				.style(|style| style.max_width(space8(310))),
		)
		.on_click_cont(move |_| {
			state.title_data.write().set(title.get());
		})
	}
}

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
use floem::views::{container, label, Decorators};
use floem::{reactive, View};

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
	pub fn render(&self) -> impl View
	{
		let state: ApplicationStateShared = reactive::use_context() /* dfplz */
			.expect("État de l'application");

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

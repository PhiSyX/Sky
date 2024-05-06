// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use floem::reactive::use_context;
use floem::style::TextOverflow;
use floem::view::View;
use floem::views::{dyn_container, scroll, text, Decorators};

use crate::state::{ApplicationStateShared, Page};
use crate::styles::colors::*;
use crate::styles::variables::*;

// --------- //
// Structure //
// --------- //

pub struct ContentArea;

// -------------- //
// Implémentation //
// -------------- //

impl ContentArea
{
	pub fn current_page(page: Page) -> impl View
	{
		match page.fetch() {
			| Ok(content) => {
				text(content)
					.style(|style| style.text_overflow(TextOverflow::Clip))
			}
			| Err(err) => {
				text(err.to_string()).style(|style| style.color(COLOR_RED600))
			}
		}
	}

	pub fn render(&self) -> impl View
	{
		let state: ApplicationStateShared =
			use_context().expect("État de l'application");

		let state_ref = ApplicationStateShared::clone(&state);

		dyn_container(
			move || state_ref.pages_data.current_page.get(),
			move |page| {
				scroll(Self::current_page(page))
					.style(|style| {
						style.size_full().text_overflow(TextOverflow::Wrap)
					})
					.any()
			},
		)
		.style(move |style| {
			style
				.apply_if(state.theme_data.is_current_dark(), |s| {
					s.background(MAIN_AREA_DARK_MODE)
				})
				.apply_if(state.theme_data.is_current_light(), |s| {
					s.background(MAIN_AREA_LIGHT_MODE)
				})
				.padding(space(2))
				.flex_grow(1.0)
				.size_full()
		})
	}
}

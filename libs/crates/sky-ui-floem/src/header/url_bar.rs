// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use floem::peniko::Color;
use floem::reactive::{create_rw_signal, use_context};
use floem::view::View;
use floem::views::{h_stack, text_input, Decorators};
use floem::widgets::PlaceholderTextClass;

use crate::classes::align::gap::Gap8;
use crate::colors::*;
use crate::icons::*;
use crate::state::FloemApplicationStateShared;
use crate::variables::*;

// --------- //
// Structure //
// --------- //

pub struct URLBar;

// -------------- //
// Implémentation //
// -------------- //

impl URLBar
{
	pub fn render(&self) -> impl View
	{
		let state: FloemApplicationStateShared =
			use_context().expect("État de l'application");

		let url = create_rw_signal(String::new());
		// MAIN_AREA_LIGHT_MODE

		h_stack((
			search_icon().class(IconWithOpacity),
			text_input(url)
				.placeholder("Entrer une URL...")
				.style(|style| {
					style
						.flex_grow(1.0)
						.items_center()
						.size_full()
						.margin_top(-4)
						.color(Color::WHITE)
				}),
		))
		.class(Gap8)
		.style(|style| {
			style.class(PlaceholderTextClass, |style| {
				style.color(PLACEHOLDER_INPUT)
			})
		})
		.style(move |style| {
			style
				.apply_if(state.theme_data.is_current_dark(), |style| {
					style
						.background(MAIN_AREA_DARK_MODE)
						.border_color(COLOR_GREY700)
				})
				.apply_if(state.theme_data.is_current_light(), |style| {
					style
						.background(MAIN_AREA_LIGHT_MODE)
						.border_color(COLOR_GREY300)
				})
				.items_center()
				.size(space8(352), space(6))
				.padding_horiz(DEFAULT_SPACE)
				.border(1)
				.border_radius(DEFAULT_BORDER_RADIUS)
		})
	}
}

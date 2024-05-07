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
use floem::style::{AlignSelf, FontStyle, TextOverflow};
use floem::taffy::AlignItems;
use floem::view::{AnyView, View};
use floem::views::{dyn_container, h_stack, scroll, text, v_stack, Decorators};

use crate::state::{ApplicationStateShared, Page};
use crate::styles::classes::align::gap::*;
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
	pub fn current_page(page: Page) -> AnyView
	{
		match page.render() {
			| Ok((content, stack)) => {
				let left_content = scroll(
					v_stack((
						text("Prévisualisation du rendu").style(|style| {
							style
								.color(COLOR_GREY500)
								.font_style(floem::cosmic_text::Style::Italic)
						}),
						stack.style(|style| {
							style.text_overflow(TextOverflow::Clip)
						}),
					))
					.class(Gap16)
					.style(|style| style.text_overflow(TextOverflow::Clip)),
				)
				.style(|style| style.size_pct(50.0, 100.0));

				let right_content = scroll(
					v_stack((
						text("HTML Brut").style(|style| {
							style
								.color(COLOR_GREY500)
								.font_style(floem::cosmic_text::Style::Italic)
						}),
						text(content).style(|style| {
							style.text_overflow(TextOverflow::Clip)
						}),
					))
					.class(Gap16)
					.style(|style| style.text_overflow(TextOverflow::Clip)),
				)
				.style(|style| style.size_pct(50.0, 100.0));

				h_stack((left_content, right_content))
					.class(Gap8)
					.style(|style| style.size_full())
					.any()
			}
			| Err(err) => {
				text(err.to_string())
					.style(|style| style.color(COLOR_RED600))
					.any()
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
			Self::current_page,
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

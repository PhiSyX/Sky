// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use sky_floem::cosmic_text::Style;
use sky_floem::style::TextOverflow;
use sky_floem::views::{
	dyn_container,
	h_stack,
	scroll,
	text,
	v_stack,
	Decorators,
};
use sky_floem::{reactive, AnyView, IntoView, ScrollableExt, View};

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
	pub fn current_page(state: ApplicationStateShared, page: Page) -> AnyView
	{
		match page.render() {
			| Ok(page_view) => {
				state.title_data.set_title(page_view.new_title);

				let Some(dyn_content) = page_view.dyn_content else {
					return text("").into_any();
				};

				if cfg!(debug_assertions) {
					let left_content = v_stack((
						text("Prévisualisation du rendu") // don't format please
							.style(|style| {
								style
									.color(COLOR_GREY500)
									.font_style(Style::Italic)
							}),
						// NOTE: Ici qu'est injecté le contenu dynamiquement
						dyn_content
							.scroll_x()
							.style(|style| style.height_pct(95.0)),
					))
					.class(Gap16)
					.style(|style| style.text_overflow(TextOverflow::Clip))
					.style(|style| style.size_pct(50.0, 100.0));

					let right_content = v_stack((
						text("HTML (raw)") // don't format please
							.style(|style| {
								style
									.color(COLOR_GREY500)
									.font_style(Style::Italic)
							}),
						text(page_view.raw_content) // don't format please
							.scroll_x()
							.style(|style| style.height_pct(95.0)),
					))
					.class(Gap16)
					.style(|style| style.text_overflow(TextOverflow::Clip))
					.style(|style| style.size_pct(50.0, 100.0));

					h_stack((
						left_content, // don't format please
						right_content,
					))
				} else {
					// NOTE: Ici qu'est injecté le contenu dynamiquement
					let left_content = dyn_content.scroll_full_size();

					h_stack((
						left_content, // don't format please
					))
				}
				.class(Gap8)
				.style(|style| style.size_full())
				.into_any()
			}
			| Err(err) => {
				text(err.to_string())
					.style(|style| style.color(COLOR_RED600))
					.into_any()
			}
		}
	}

	pub fn render(&self) -> impl View
	{
		let state: ApplicationStateShared = reactive::use_context() /* dfplz */
			.expect("État de l'application");

		let state_r = ApplicationStateShared::clone(&state);
		let state_w = ApplicationStateShared::clone(&state);

		dyn_container(move || {
			Self::current_page(
				state_w.clone(),
				state_r.pages_data.current_page.get(),
			)
		})
		.style(|style| style.padding(space(2)).flex_grow(1.0).size_full())
		.style(move |style| {
			style
				.apply_if(state.theme_data.is_current_dark(), |s| {
					s.background(MAIN_AREA_DARK_MODE)
				})
				.apply_if(state.theme_data.is_current_light(), |s| {
					s.background(MAIN_AREA_LIGHT_MODE)
				})
		})
	}
}

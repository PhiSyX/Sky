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
use floem::reactive::{provide_context, use_context};
use floem::style::{CursorStyle, Foreground, TextColor, Transition};
use floem::view::View;
use floem::views::{v_stack, Decorators};
use floem::window::WindowId;
use sky_ui::{ApplicationSettings, ThemeSettings};

use crate::classes::align::gap::{Gap16, Gap8};
use crate::colors::*;
use crate::header::area::HeaderArea;
use crate::icons::{Icon, IconWithOpacity};
use crate::state::{
	FloemApplicationState,
	FloemApplicationStateShared,
	ThemeData,
	TitleData,
};
use crate::variables::*;

// --------- //
// Structure //
// --------- //

pub struct FloemWindow
{
	window_id: WindowId,
	header_area: HeaderArea,
}

// -------------- //
// Implémentation //
// -------------- //

impl FloemWindow
{
	pub fn new(settings: ApplicationSettings, window_id: WindowId) -> Self
	{
		let shared_settings = settings.shared();

		let state = FloemApplicationState {
			theme_data: ThemeData::new(shared_settings.theme()),
			title_data: TitleData::new(shared_settings.title()),
		};

		provide_context(shared_settings);
		provide_context(state.shared());

		Self {
			window_id,
			header_area: HeaderArea::new(window_id),
		}
	}

	pub fn view(&self) -> impl View
	{
		let state: FloemApplicationStateShared =
			use_context().expect("État de l'application");

		let header_area = self.header_area.render();

		v_stack((
			header_area, // -- don't format please
		))
		.style(move |style| {
			style
				.apply_if(state.theme_data.is_current_dark(), |style| {
					style
						.background(COLOR_BLACK)
						.color(TEXT_COLOR_WHITE)
						.border_color(COLOR_GREY700)
				})
				.apply_if(state.theme_data.is_current_light(), |style| {
					style
						.background(COLOR_ULTRA_WHITE)
						.color(COLOR_BLACK)
						.border_color(COLOR_GREY300)
				})
				.size_full()
				.font_family(String::from(DEFAULT_FONT_FAMILY))
				.font_size(DEFAULT_FONT_SIZE)
				.border(1)
				.border_radius(DEFAULT_BORDER_RADIUS * 2)
				// Définition des classes
				.class(Gap8, |style| style.gap(space(1), space(1)))
				.class(Gap16, |style| style.gap(space(2), space(2)))
				.class(Icon, |style| {
					style
						.apply_if(state.theme_data.is_current_dark(), |s| {
							s.color(TEXT_COLOR_WHITE)
						})
						.apply_if(state.theme_data.is_current_light(), |s| {
							s.color(COLOR_BLACK)
						})
						.cursor(CursorStyle::Pointer)
						.transition(TextColor, Transition::linear(2.0))
						.transition(Foreground, Transition::linear(2.0))
				})
				.class(IconWithOpacity, |style| {
					style
						.apply_if(
							state.theme_data.read().get()
								== ThemeSettings::Dark,
							|s| {
								s.color(Color::WHITE.with_alpha_factor(0.5))
									.hover(|style| style.color(Color::WHITE))
									.focus(|style| style.color(Color::WHITE))
							},
						)
						.apply_if(
							state.theme_data.read().get()
								== ThemeSettings::Light,
							|s| {
								s.color(Color::BLACK.with_alpha_factor(0.5))
									.hover(|style| style.color(Color::BLACK))
									.focus(|style| style.color(Color::BLACK))
							},
						)
						.padding(space(1))
						.cursor(CursorStyle::Pointer)
						.transition(TextColor, Transition::linear(2.0))
						.transition(Foreground, Transition::linear(2.0))
				})
		})
	}
}

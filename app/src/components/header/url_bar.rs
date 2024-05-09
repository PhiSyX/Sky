// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use sky_floem::peniko::Color;
use sky_floem::views::{h_stack, text_input, Decorators, PlaceholderTextClass};
use sky_floem::{event, keyboard, reactive, View};

use crate::components::icons::*;
use crate::state::{ApplicationStateShared, Page};
use crate::styles::classes::align::gap::*;
use crate::styles::colors::*;
use crate::styles::variables::*;

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
		let state: ApplicationStateShared = reactive::use_context() /* dfplz */
			.expect("État de l'application");

		let url_s = reactive::create_rw_signal(String::new());

		let state_w = ApplicationStateShared::clone(&state);
		reactive::create_effect(move |_| {
			url_s.set(state_w.pages_data.current_page.get().url_to_display());
		});

		let open_url_handler = {
			let state_w = ApplicationStateShared::clone(&state);

			move |_: &event::Event| {
				let file_or_url = url_s.get();
				let file_or_url = file_or_url.trim();

				if file_or_url.is_empty() {
					return;
				}

				if let Ok(url) = file_or_url.parse::<url::Url>() {
					if url.scheme().starts_with("http") {
						state_w.pages_data.current_page.set(Page::Url(url));
						return;
					}
				}

				state_w.pages_data.current_page.set(Page::File(
					file_or_url.trim_start_matches("file://").into(),
				));
			}
		};

		h_stack((
			search_icon().class(IconWithOpacity),
			text_input(url_s)
				.placeholder("Entrer une URL...")
				.on_key_up(
					keyboard::Key::Named(keyboard::NamedKey::Enter),
					keyboard::Modifiers::empty(),
					open_url_handler,
				)
				.style(move |style| {
					style
						.flex_grow(1.0)
						.items_center()
						.size_full()
						.margin_top(-4)
				})
				.style({
					let state_r = ApplicationStateShared::clone(&state);

					move |style| {
						style
							.apply_if(
								state_r.theme_data.is_current_dark(),
								|s| {
									s.background(Color::TRANSPARENT)
										.color(COLOR_WHITE)
								},
							)
							.apply_if(
								state_r.theme_data.is_current_light(),
								|s| {
									s.background(Color::TRANSPARENT)
										.color(COLOR_GREY700)
								},
							)
							.border(0)
							// WTF ??????
							.active(|style| {
								style.background(Color::TRANSPARENT)
							})
							.hover(|style| style.background(Color::TRANSPARENT))
							.focus(|style| {
								style.background(Color::TRANSPARENT).hover(
									|style| {
										style.background(Color::TRANSPARENT)
									},
								)
							})
							.selected(|style| {
								style.background(Color::TRANSPARENT)
							})
							.focus_visible(|style| {
								style.background(Color::TRANSPARENT)
							})
							.background(Color::TRANSPARENT)
					}
				}),
		))
		.class(Gap8)
		.style(|style| {
			style.class(PlaceholderTextClass, |style| {
				style.color(PLACEHOLDER_INPUT)
			})
		})
		.style(|style| {
			style
				.items_center()
				.size(space8(352), space(6))
				.padding_horiz(DEFAULT_SPACE)
				.border(1)
				.border_radius(DEFAULT_BORDER_RADIUS)
		})
		.style({
			let state_r = ApplicationStateShared::clone(&state);

			move |style| {
				style
					.apply_if(state_r.theme_data.is_current_dark(), |style| {
						style
							.background(MAIN_AREA_DARK_MODE)
							.border_color(COLOR_GREY700)
					})
					.apply_if(state_r.theme_data.is_current_light(), |style| {
						style
							.background(MAIN_AREA_LIGHT_MODE)
							.border_color(COLOR_GREY300)
					})
			}
		})
	}
}

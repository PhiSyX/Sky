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

use floem::peniko::Color;
use floem::views::{h_stack, v_stack, Decorators};
use floem::window::{WindowConfig, WindowId};
use floem::{keyboard, reactive, style, window, View};
use sky_ui::{ApplicationSettings, Size};

use crate::components::header::area::HeaderArea;
use crate::components::icons::*;
use crate::components::main::MainArea;
use crate::components::nav::NavigationArea;
use crate::state::{
	ApplicationState,
	ApplicationStateShared,
	PagesData,
	ThemeData,
	TitleData,
};
use crate::styles::classes::align::gap::*;
use crate::styles::colors::*;
use crate::styles::variables::*;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
pub struct Application
{
	settings: ApplicationSettings,
}

pub struct Window
{
	window_id: WindowId,
	header: HeaderArea,
	nav: NavigationArea,
	main: MainArea,
}

// -------------- //
// Implémentation //
// -------------- //

impl Application
{
	pub fn new() -> Self
	{
		Self {
			settings: ApplicationSettings::default(),
		}
	}

	pub fn window_size(mut self, default_size: impl Into<Size>) -> Self
	{
		self.settings.set_size(default_size);
		self
	}

	pub fn window_title(mut self, default_title: impl ToString) -> Self
	{
		self.settings.set_title(default_title);
		self
	}
}

impl Application
{
	#[inline(always)]
	pub fn run(self) -> impl Termination
	{
		let window_settings = WindowConfig::default()
			.size(self.settings.size_tuple())
			.title(self.settings.title())
			// .apply_default_theme(false)
			// .with_transparent(true)
			.show_titlebar(false);

		let window_render = |window_id| {
			let view = Window::new(self.settings, window_id).view();
			let view_id = view.id();
			view
				// NOTE: inspection
				.on_key_up(
					keyboard::Key::Named(keyboard::NamedKey::F11),
					keyboard::Modifiers::empty(),
					move |_| view_id.inspect(),
				)
				// NOTE: fermeture
				.on_key_up(
					keyboard::Key::Named(keyboard::NamedKey::Escape),
					keyboard::Modifiers::empty(),
					move |_| window::close_window(window_id),
				)
				.on_key_up(
					keyboard::Key::Character("c".into()),
					keyboard::Modifiers::CONTROL,
					move |_| window::close_window(window_id),
				)
		};

		floem::Application::new()
			.window(window_render, Some(window_settings))
			.run()
	}
}

impl Window
{
	pub fn new(settings: ApplicationSettings, window_id: WindowId) -> Self
	{
		let shared_settings = settings.shared();

		let state = ApplicationState {
			pages_data: PagesData::new(),
			theme_data: ThemeData::new(shared_settings.theme()),
			title_data: TitleData::new(shared_settings.title()),
		};

		reactive::provide_context(shared_settings);
		reactive::provide_context(state.shared());

		Self {
			window_id,
			header: HeaderArea::new(window_id),
			nav: NavigationArea,
			main: MainArea::new(),
		}
	}

	pub fn view(&self) -> impl View
	{
		let state: ApplicationStateShared = reactive::use_context() /* dfplz */
			.expect("État de l'application");

		let header_area = self.header.render();
		let nav_area = self.nav.render();
		let main_area = self.main.render();

		v_stack((
			header_area,
			h_stack((nav_area, main_area)).style(|style| {
				style.flex_grow(1.0).size_pct(100.0, 100.0 - 50.0)
			}),
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
				.class(Gap24, |style| style.gap(space(3), space(3)))
				.class(Icon, |style| {
					style
						.apply_if(state.theme_data.is_current_dark(), |s| {
							s.color(TEXT_COLOR_WHITE)
						})
						.apply_if(state.theme_data.is_current_light(), |s| {
							s.color(COLOR_BLACK)
						})
						.cursor(style::CursorStyle::Pointer)
						.transition(
							style::TextColor,
							style::Transition::linear(2.0),
						)
						.transition(
							style::Foreground,
							style::Transition::linear(2.0),
						)
				})
				.class(IconWithOpacity, |style| {
					style
						.apply_if(state.theme_data.is_current_dark(), |s| {
							s.color(Color::WHITE.with_alpha_factor(0.5))
								.hover(|style| style.color(Color::WHITE))
								.focus(|style| style.color(Color::WHITE))
						})
						.apply_if(state.theme_data.is_current_light(), |s| {
							s.color(Color::BLACK.with_alpha_factor(0.5))
								.hover(|style| style.color(Color::BLACK))
								.focus(|style| style.color(Color::BLACK))
						})
						.padding(space(1))
						.cursor(style::CursorStyle::Pointer)
						.transition(
							style::TextColor,
							style::Transition::linear(2.0),
						)
						.transition(
							style::Foreground,
							style::Transition::linear(2.0),
						)
				})
		})
	}
}

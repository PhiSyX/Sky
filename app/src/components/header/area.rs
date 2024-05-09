// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use sky_floem::views::{drag_window_area, h_stack, Decorators};
use sky_floem::window::WindowId;
use sky_floem::View;

use super::controls::WindowControls;
use super::title::Title;
use super::url_bar::URLBar;
use super::user_avatar::UserAvatar;
use crate::styles::classes::align::gap::*;
use crate::styles::variables::*;

// --------- //
// Structure //
// --------- //

pub struct HeaderArea
{
	user_avatar: UserAvatar,
	url_bar: URLBar,
	title: Title,
	controls: WindowControls,
}

// -------------- //
// Implémentation //
// -------------- //

impl HeaderArea
{
	pub fn new(window_id: WindowId) -> Self
	{
		Self {
			user_avatar: UserAvatar,
			url_bar: URLBar,
			title: Title,
			controls: WindowControls::new(window_id),
		}
	}

	pub fn render(&self) -> impl View
	{
		let avatar = self.user_avatar.render();

		let search_url_bar = self.url_bar.render();

		let window_title = drag_window_area(self.title.render()) // dfplz
			.style(|style| {
				style.flex_grow(1.0).justify_center().items_center()
			});

		let window_controls = self.controls.render();

		h_stack((
			avatar,
			search_url_bar,
			window_title,
			window_controls, // don't format please
		))
		.class(Gap16)
		.style(|style| style.width_full().height(space8(80)).padding(space(2)))
	}
}

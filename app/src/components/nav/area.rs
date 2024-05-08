// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use floem::view::View;
use floem::views::{v_stack, Decorators};

use crate::components::icons::*;
use crate::styles::classes::align::gap::*;
use crate::styles::variables::*;

// --------- //
// Structure //
// --------- //

pub struct NavigationArea;

// -------------- //
// Implémentation //
// -------------- //

impl NavigationArea
{
	pub fn render(&self) -> impl View
	{
		v_stack((
			home_icon().class(IconWithOpacity), // don't format please
		))
		.class(Gap24)
		.style(|style| {
			style
				.min_width(space(6))
				.width(space(6))
				.items_center()
				.padding_vert(space(2))
		})
	}
}

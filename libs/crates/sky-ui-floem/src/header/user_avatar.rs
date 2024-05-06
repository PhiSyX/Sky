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
use floem::view::View;
use floem::views::{container, img, Decorators};

use crate::variables::*;

// --------- //
// Structure //
// --------- //

pub struct UserAvatar;

// -------------- //
// Implémentation //
// -------------- //

impl UserAvatar
{
	pub fn render(&self) -> impl View
	{
		// FIXME(phisyx): récupérer une image via une URL distante ou la
		// récupérer via le profil utilisateur de l'OS.
		let user_profile_avatar =
			include_bytes!("../../../../../assets/img/user.jpg");

		container(img(move || user_profile_avatar.to_vec()).style(|style| {
			style
				.size(space(6), space(6))
				.border_radius(space(6))
				.box_shadow_color(Color::BLACK)
				.box_shadow_blur(4)
				.box_shadow_h_offset(2)
				.box_shadow_v_offset(2)
		}))
	}
}

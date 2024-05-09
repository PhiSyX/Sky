// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use sky_floem::style_class;

// ----- //
// MACRO //
// ----- //

macro_rules! make_svg_icon {
	(
		$(
			$(#[$doc:meta])*
			$vis:vis const $constant:ident : &str = $svg:literal;
		)*
	) => { $(paste::paste! {
		$(#[$doc])*
		$vis fn [ < $constant:lower _icon > ] () -> impl sky_floem::View
		{
			use sky_floem::views::{Decorators, svg};
			const $constant: &str = include_str!(concat!("../../../", $svg));
			svg(|| $constant.to_owned())
				.style(|style| style.size(24, 24))
		}
	})* };
}

// -------- //
// Constant //
// -------- //

make_svg_icon! {
	pub const HOME: &str = "assets/svg/home.svg";
	pub const NOTIFICATION: &str = "assets/svg/notification.svg";
	pub const SEARCH: &str = "assets/svg/search.svg";
	pub const THEME: &str = "assets/svg/theme.svg";
	pub const WINDOW_CLOSE: &str = "assets/svg/window-close.svg";
	pub const WINDOW_MAXIMIZE: &str = "assets/svg/window-maximize.svg";
	pub const WINDOW_MINIMIZE: &str = "assets/svg/window-minimize.svg";
}

// --------- //
// Structure //
// --------- //

style_class!(pub Icon);
style_class!(pub IconWithOpacity);

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

// -------- //
// Constant //
// -------- //

pub const COLOR_BLACK: Color = Color::rgb8(17, 17, 17);
pub const COLOR_WHITE: Color = Color::rgb8(233, 242, 244);
pub const COLOR_ULTRA_WHITE: Color = Color::WHITE;

// ROUGE
pub const COLOR_RED50: Color = Color::rgb8(255, 235, 238);
pub const COLOR_RED200: Color = Color::rgb8(239, 154, 154);
pub const COLOR_RED300: Color = Color::rgb8(229, 115, 115);
pub const COLOR_RED400: Color = Color::rgb8(239, 83, 80);
pub const COLOR_RED500: Color = Color::rgb8(244, 67, 54);
pub const COLOR_RED600: Color = Color::rgb8(229, 57, 53);
pub const COLOR_RED700: Color = Color::rgb8(211, 47, 47);
pub const COLOR_RED800: Color = Color::rgb8(198, 40, 40);
pub const COLOR_RED900: Color = Color::rgb8(183, 28, 28);

// GRIS
pub const COLOR_GREY50: Color = Color::rgb8(250, 250, 250);
pub const COLOR_GREY100: Color = Color::rgb8(245, 245, 245);
pub const COLOR_GREY200: Color = Color::rgb8(238, 238, 238);
pub const COLOR_GREY300: Color = Color::rgb8(224, 224, 224);
pub const COLOR_GREY400: Color = Color::rgb8(189, 189, 189);
pub const COLOR_GREY500: Color = Color::rgb8(158, 158, 158);
pub const COLOR_GREY600: Color = Color::rgb8(117, 117, 117);
pub const COLOR_GREY700: Color = Color::rgb8(97, 97, 97);
pub const COLOR_GREY800: Color = Color::rgb8(66, 66, 66);
pub const COLOR_GREY900: Color = Color::rgb8(33, 33, 33);

// Global

pub const PLACEHOLDER_INPUT: Color = Color::GRAY;
pub const TEXT_COLOR_WHITE: Color = COLOR_GREY50;

// Dark Mode
pub const MAIN_AREA_DARK_MODE: Color = Color::rgb8(29, 33, 37);

// Light Mode
pub const MAIN_AREA_LIGHT_MODE: Color = COLOR_GREY100;

// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::sync::Arc;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Default)]
pub struct ApplicationSettings
{
	theme: ThemeSettings,
	title: String,
	size: Size,
}

#[derive(Debug)]
pub struct Size(/* width */ f64, /* height */ f64);

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Default)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum ThemeSettings
{
	#[default]
	Dark,
	Light,
}

// -------------- //
// Implémentation //
// -------------- //

impl ApplicationSettings
{
	pub fn shared(self) -> Arc<Self>
	{
		Arc::new(self)
	}
}

impl ApplicationSettings
{
	pub fn theme(&self) -> ThemeSettings
	{
		self.theme
	}

	pub fn set_theme(&mut self, theme: ThemeSettings)
	{
		self.theme = theme;
	}
}

impl ApplicationSettings
{
	pub fn size(&self) -> &Size
	{
		&self.size
	}

	pub fn size_tuple(&self) -> (f64, f64)
	{
		(self.size.0, self.size.1)
	}

	pub fn set_size(&mut self, size: impl Into<Size>)
	{
		self.size = size.into();
	}
}

impl ApplicationSettings
{
	pub fn title(&self) -> &str
	{
		&self.title
	}

	pub fn set_title(&mut self, title: impl ToString)
	{
		self.title = title.to_string();
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl Default for Size
{
	fn default() -> Self
	{
		Self(1000.0, 500.0)
	}
}

macro_rules! into_size {
	($($ty:ty),*) => {
$(
impl From<($ty, $ty)> for Size
{
	fn from(value: ($ty, $ty)) -> Self
	{
		Self(value.0 as _, value.1 as _)
	}
}
)*
	};
}

into_size!(u8, i8, u16, i16, u32, i32, u64, i64);
into_size!(f32, f64);

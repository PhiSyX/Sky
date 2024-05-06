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
}

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
	pub fn title(&self) -> &str
	{
		&self.title
	}

	pub fn set_title(&mut self, title: impl ToString)
	{
		self.title = title.to_string();
	}
}
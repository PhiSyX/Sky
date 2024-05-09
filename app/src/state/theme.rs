// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use sky_floem::reactive::{create_signal, ReadSignal, WriteSignal};
use sky_ui::ThemeSettings;

// --------- //
// Structure //
// --------- //

pub struct ThemeData
{
	signal: (ReadSignal<ThemeSettings>, WriteSignal<ThemeSettings>),
}

// -------------- //
// Implémentation //
// -------------- //

impl ThemeData
{
	pub fn new(theme: ThemeSettings) -> Self
	{
		Self {
			signal: create_signal(theme),
		}
	}

	pub fn read(&self) -> ReadSignal<ThemeSettings>
	{
		self.signal.0
	}

	pub fn write(&self) -> WriteSignal<ThemeSettings>
	{
		self.signal.1
	}
}

impl ThemeData
{
	pub fn is_current_dark(&self) -> bool
	{
		self.read().get() == ThemeSettings::Dark
	}

	pub fn is_current_light(&self) -> bool
	{
		self.read().get() == ThemeSettings::Light
	}
}

impl ThemeData
{
	pub fn toggle(&self)
	{
		if self.is_current_dark() {
			self.write().set(ThemeSettings::Light)
		} else {
			self.write().set(ThemeSettings::Dark)
		}
	}
}

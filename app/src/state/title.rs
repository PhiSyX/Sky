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

// --------- //
// Structure //
// --------- //

pub struct TitleData
{
	default: String,
	signal: (ReadSignal<String>, WriteSignal<String>),
}

// -------------- //
// Implémentation //
// -------------- //

impl TitleData
{
	pub fn new(default_title: impl ToString) -> Self
	{
		Self {
			default: default_title.to_string(),
			signal: create_signal(default_title.to_string()),
		}
	}

	pub fn read(&self) -> ReadSignal<String>
	{
		self.signal.0
	}

	pub fn write(&self) -> WriteSignal<String>
	{
		self.signal.1
	}

	pub fn set_title(&self, new_title: impl ToString)
	{
		self.signal.1.update(move |current_title| {
			let title = new_title.to_string();
			if title.trim().is_empty() {
				*current_title = self.default.to_string();
				return;
			}

			if title.trim() == self.default {
				return;
			}
			*current_title = format!("{} - {}", title, self.default);
		})
	}
}

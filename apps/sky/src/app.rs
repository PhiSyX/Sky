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

use sky_ui::ApplicationAdapter;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
pub struct Application<A: ApplicationAdapter>
{
	adapter: A,
}

// -------------- //
// Implémentation //
// -------------- //

impl<A: ApplicationAdapter> Application<A>
{
	pub fn new() -> Self
	{
		Self { adapter: A::new() }
	}

	pub fn window_title(mut self, default_title: impl ToString) -> Self
	{
		self.adapter.define_window_title(default_title);
		self
	}
}

impl<A: ApplicationAdapter + 'static> Application<A>
{
	#[inline(always)]
	pub fn run(self) -> impl Termination
	{
		self.adapter.launch()
	}
}

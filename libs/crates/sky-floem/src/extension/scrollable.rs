// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

//! Élément scrollable
//!
//! ```rs
//! my_el
//!   .scroll()
//! ```

use floem::style::TextOverflow;
use floem::unit::PxPctAuto;
use floem::views::{scroll, Decorators, Scroll};
use floem::IntoView;

// --------- //
// Interface //
// --------- //

pub trait ScrollableExt: IntoView
{
	fn scroll(self) -> Scroll
	{
		scroll(self.style(|style| style.text_overflow(TextOverflow::Clip)))
			.style(|style| style.text_overflow(TextOverflow::Clip))
	}

	fn scroll_full_size(self) -> Scroll
	{
		self.scroll().style(|style| style.size_full())
	}

	fn scroll_x(self) -> Scroll
	{
		self.scroll().style(|style| style.width_full())
	}

	fn scroll_y(self) -> Scroll
	{
		self.scroll().style(|style| style.height_full())
	}

	fn scroll_with_size(
		self,
		size: (impl Into<PxPctAuto>, impl Into<PxPctAuto>),
	) -> Scroll
	{
		let (w, h) = (size.0.into(), size.1.into());
		self.scroll().style(move |style| style.size(w, h))
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<V: IntoView + 'static> ScrollableExt for V {}

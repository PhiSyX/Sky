// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::fmt;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Location
{
	start: usize,
	end: usize,
	column: usize,
	line: usize,
}

// -------------- //
// Implémentation //
// -------------- //

impl Location
{
	pub const fn new() -> Self
	{
		Self {
			start: 0,
			end: 0,
			column: 1,
			line: 1,
		}
	}
}

impl Location
{
	/// Incrémente une colonne de 1.
	pub fn increment_column(&mut self) -> &mut Self
	{
		self.column = self.column.saturating_add(1);
		self.increment_end()
	}

	/// Incrémente une colonne de N.
	pub fn increment_column_by(&mut self, n: usize) -> &mut Self
	{
		self.column = self.column.saturating_add(n);
		self.increment_end_by(n)
	}

	/// Incrémente une colonne de 1.
	pub fn increment_column_if_zeroed(&mut self) -> &mut Self
	{
		if self.column == 0 {
			self.column = self.column.saturating_add(1);
		}
		self
	}

	/// Incrémente la fin de 1.
	pub fn increment_end(&mut self) -> &mut Self
	{
		self.end = self.end.saturating_add(1);
		self
	}

	/// Incrémente la fin de N.
	pub fn increment_end_by(&mut self, n: usize) -> &mut Self
	{
		self.end = self.end.saturating_add(n);
		self
	}

	/// Incrémente une ligne de 1.
	pub fn increment_line(&mut self) -> &mut Self
	{
		self.line = self.line.saturating_add(1);
		self.increment_end()
	}

	/// Incrémente une ligne de N.
	pub fn increment_line_by(&mut self, n: usize) -> &mut Self
	{
		self.line = self.line.saturating_add(n);
		self.increment_end_by(n)
	}

	/// Incrémente une ligne de 1 si la ligne vaut zero.
	pub fn increment_line_if_zeroed(&mut self) -> &mut Self
	{
		if self.line == 0 {
			self.line = self.line.saturating_add(1);
		}
		self
	}

	/// Réinitialise la colonne.
	pub fn reset_column(&mut self) -> &mut Self
	{
		self.column = 1;
		self
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl Default for Location
{
	fn default() -> Self
	{
		Self {
			start: 0,
			end: 0,
			column: 1,
			line: 1,
		}
	}
}

// EXAMPLE(output): Ln {0..9}, Col {0..9}
impl fmt::Display for Location
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		let ln = format!("Ln {}", if self.line == 0 { 1 } else { self.line });
		let col =
			format!("Col {}", if self.column == 0 { 1 } else { self.column });
		let ln_col = format!("{ln}, {col}");
		write!(f, "{}", ln_col)
	}
}

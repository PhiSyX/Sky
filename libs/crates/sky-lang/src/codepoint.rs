// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

// --------- //
// Interface //
// --------- //

// NOTE: la présence des deux underscores (_) sur les noms des méthodes est pour
// pour éviter de surcharger des méthodes déjà existantes, par ex: is_whitespace
#[allow(non_snake_case)]
pub trait UnitCodePointExt:
	Clone + std::fmt::Display + std::fmt::Debug
{
	fn is(&self, ch: char) -> bool;

	fn one_of(&self, chars: impl IntoIterator<Item = char>) -> bool;

	fn is__alphabetic(&self) -> bool;

	fn is__upper_alphabetic(&self) -> bool;

	fn is__alphanumeric(&self) -> bool;

	fn is__carriage_return(&self) -> bool;

	fn is__digit(&self) -> bool;

	fn is__ident_start(&self) -> bool
	{
		self.is__alphabetic() || self.is__non_ascii() || self.is('_')
	}

	fn is__ident_after_start(&self) -> bool
	{
		self.is__alphanumeric()
			|| self.is__non_ascii()
			|| self.one_of(['_', '-'])
	}

	fn is__linefeed(&self) -> bool;

	fn is__newline(&self) -> bool;

	fn is__non_ascii(&self) -> bool;

	fn is__valid(&self) -> bool;

	fn is__tab(&self) -> bool;

	fn is__whitespace(&self) -> bool;

	fn unit(&self) -> char;
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl UnitCodePointExt for char
{
	fn is(&self, ch: char) -> bool
	{
		*self == ch
	}

	fn one_of(&self, chars: impl IntoIterator<Item = char>) -> bool
	{
		chars.into_iter().any(|ch| *self == ch)
	}

	fn is__alphabetic(&self) -> bool
	{
		self.is_ascii_alphabetic()
	}

	fn is__upper_alphabetic(&self) -> bool
	{
		self.is__alphabetic() && self.is_uppercase()
	}

	fn is__alphanumeric(&self) -> bool
	{
		self.is_ascii_alphanumeric()
	}

	fn is__carriage_return(&self) -> bool
	{
		*self == '\r'
	}

	fn is__digit(&self) -> bool
	{
		self.is_ascii_digit()
	}

	fn is__linefeed(&self) -> bool
	{
		*self == '\n'
	}

	fn is__newline(&self) -> bool
	{
		self.is__carriage_return() || self.is__linefeed() || *self == '\x0C'
	}

	fn is__non_ascii(&self) -> bool
	{
		*self as u32 >= 0x80
	}

	fn is__valid(&self) -> bool
	{
		*self != '\0'
	}

	fn is__tab(&self) -> bool
	{
		*self == '\t'
	}

	fn is__whitespace(&self) -> bool
	{
		self.is_whitespace()
	}

	fn unit(&self) -> char
	{
		*self
	}
}

impl UnitCodePointExt for u8
{
	fn is(&self, ch: char) -> bool
	{
		*self == ch as u8
	}

	fn one_of(&self, chars: impl IntoIterator<Item = char>) -> bool
	{
		chars.into_iter().any(|ch| *self == ch as u8)
	}

	fn is__alphabetic(&self) -> bool
	{
		self.is_ascii_alphabetic()
	}

	fn is__upper_alphabetic(&self) -> bool
	{
		self.is__alphabetic() && self.is_ascii_uppercase()
	}

	fn is__alphanumeric(&self) -> bool
	{
		self.is_ascii_alphanumeric()
	}

	fn is__carriage_return(&self) -> bool
	{
		*self == b'\r'
	}

	fn is__digit(&self) -> bool
	{
		self.is_ascii_digit()
	}

	fn is__linefeed(&self) -> bool
	{
		*self == b'\n'
	}

	fn is__newline(&self) -> bool
	{
		self.is__carriage_return() || self.is__linefeed() || *self == b'\x0C'
	}

	fn is__non_ascii(&self) -> bool
	{
		*self as u32 >= 0x80
	}

	fn is__valid(&self) -> bool
	{
		*self != b'\0'
	}

	fn is__tab(&self) -> bool
	{
		*self == b'\t'
	}

	fn is__whitespace(&self) -> bool
	{
		self.is_ascii_whitespace()
	}

	fn unit(&self) -> char
	{
		*self as char
	}
}

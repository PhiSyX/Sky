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

use sky_lang::Location;

use super::lexeme::{HTMLLexeme, TagState};

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
pub struct HTMLToken
{
	lexeme: HTMLLexeme,
	location: Location,
}

// -------------- //
// Implémentation //
// -------------- //

impl HTMLToken
{
	pub const fn character(ch: char) -> Self
	{
		Self {
			lexeme: HTMLLexeme::Character(ch),
			location: Location::new(),
		}
	}
}

impl HTMLToken
{
	pub const fn empty_comment() -> Self
	{
		Self {
			lexeme: HTMLLexeme::Comment(String::new()),
			location: Location::new(),
		}
	}
}

impl HTMLToken
{
	pub fn end_tag() -> Self
	{
		Self {
			lexeme: HTMLLexeme::Tag {
				state: TagState::Closed,
				name: Default::default(),
				attributes: Default::default(),
			},
			location: Location::new(),
		}
	}

	pub fn start_tag() -> Self
	{
		Self {
			lexeme: HTMLLexeme::Tag {
				state: TagState::Opened,
				name: Default::default(),
				attributes: Default::default(),
			},
			location: Location::new(),
		}
	}

	pub const fn is_opened_tag(&self) -> bool
	{
		matches!(
			&self.lexeme,
			HTMLLexeme::Tag {
				state: TagState::Opened,
				..
			}
		)
	}

	pub fn add_character_to_tag_name(&mut self, ch: char)
	{
		self.lexeme.add_character_to_tag_name(ch);
	}
}

impl HTMLToken
{
	pub const fn end_of_stream() -> Self
	{
		Self {
			lexeme: HTMLLexeme::EndOfStream,
			location: Location::new(),
		}
	}

	pub const fn is_end_of_stream(&self) -> bool
	{
		matches!(&self.lexeme, HTMLLexeme::EndOfStream)
	}
}

impl HTMLToken
{
	pub fn with_location(mut self, location: Location) -> Self
	{
		self.location = location;
		self
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for HTMLToken
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "token-{}", self.lexeme)
	}
}

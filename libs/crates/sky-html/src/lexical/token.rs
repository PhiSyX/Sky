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
	/// Crée un jeton de caractère.
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
	/// Crée un jeton de commentaire vide.
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
	/// Crée un jeton de balise de fin.
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

	/// Crée un jeton de balise de début.
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

	/// Vérifie que le jeton est une balise ouverte.
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

	/// Vérifie que le jeton est une balise ouverte.
	pub fn add_character_to_tag_name(&mut self, ch: char)
	{
		self.lexeme.add_character_to_tag_name(ch);
	}

	/// Ajoute un caractère au dernier nom d'un attribut de la balise courante.
	pub fn add_character_to_last_attribute_name_of_tag(&mut self, ch: char)
	{
		self.lexeme.add_character_to_last_attribute_name_of_tag(ch);
	}

	/// Ajoute un caractère à la dernière valeur d'un attribut de la balise
	/// courante.
	pub fn add_character_to_last_attribute_value_of_tag(&mut self, ch: char)
	{
		self.lexeme.add_character_to_last_attribute_value_of_tag(ch);
	}

	/// Ajoute un attribut vide pour la balise courante.
	pub fn start_empty_attribute_for_tag(&mut self)
	{
		self.lexeme.start_empty_attribute_for_tag();
	}

	/// Ajoute un attribut avec comme nom le caractère donné pour la balise
	/// courante.
	pub fn start_attribute_tag_with(&mut self, ch: char)
	{
		self.lexeme.start_attribute_tag_with(ch);
	}
}

impl HTMLToken
{
	/// Crée un jeton de fin de flux.
	pub const fn end_of_stream() -> Self
	{
		Self {
			lexeme: HTMLLexeme::EndOfStream,
			location: Location::new(),
		}
	}

	/// Vérifie que le jeton est un jeton de fin de flux.
	pub const fn is_end_of_stream(&self) -> bool
	{
		matches!(&self.lexeme, HTMLLexeme::EndOfStream)
	}
}

impl HTMLToken
{
	/// Définit la position du jeton dans le flux, la source.
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

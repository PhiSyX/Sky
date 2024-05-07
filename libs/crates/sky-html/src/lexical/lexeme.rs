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

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub enum HTMLLexeme
{
	/// Commentaire HTML, ex: <!-- Mon super commentaire -->
	Comment(
		/// Le contenu du commentaire, ex: `Mon super commentaire`
		String,
	),

	/// Balise HTML, ex: `<button onclick="...">`, `</div>`, `<input
	/// type="text" />`
	Tag
	{
		/// Nom de la balise, ex: "button", "div", "input"
		name: String,
		/// Attributs de la balise, ex: `onclick="..."`, `type="text"`
		attributes: Vec<(String, String)>,
		/// État de la balise, ex: ouverte, fermée, auto-fermante.
		state: TagState,
	},

	/// Un caractère.
	Character(char),

	/// Fin du flux.
	EndOfStream,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum TagState
{
	/// Balise ouverte.
	Opened,
	/// Balise fermée.
	Closed,
	/// Balise auto-fermée.
	SelfClosed,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for HTMLLexeme
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let name = match self {
			| Self::Comment(_) => String::from("comment"),
			| Self::Tag { name, .. } => format!("tag-{}", name.to_owned()),
			| Self::Character(ch) => ch.to_string(),
			| Self::EndOfStream => String::from("eos"),
		};
		write!(f, "{}", name)
	}
}

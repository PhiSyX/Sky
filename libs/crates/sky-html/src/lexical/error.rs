// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use sky_lang::Location;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(thiserror::Error)]
#[error(
	"Erreur d'analyse de l'HTML, raison: « {variant} », à la position \
	 {location}"
)]
pub struct HTMLLexicalError
{
	variant: HTMLLexicalErrorVariant,
	location: Location,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum HTMLLexicalErrorVariant
{
	/// Cette erreur se produit si l'analyseur rencontre un point de code
	/// U+003F (?) alors que le premier point de code d'un nom de balise de
	/// début est attendu. Le point de code U+003F (?) et tout le contenu qui
	/// suit jusqu'à un point de code U+003E (>) (s'il est présent) ou jusqu'à
	/// la fin du flux d'entrée est traité comme un commentaire.
	///
	/// Example:
	///   `<?xml-stylesheet type="text/css" href="style.css"?>`
	///
	/// Parsed into:
	///   |- #comment: ?xml-stylesheet type="text/css" href="style.css"?
	///   |- html
	///      | - head
	///      | - body
	///
	/// NOTE(html): la raison courante de cette erreur est une instruction de
	/// traitement XML (par exemple, `<?xml-stylesheet type="text/css"
	/// href="style.css"?>`) ou une déclaration XML (par exemple, `<?xml
	/// version="1.0" encoding="UTF-8"?>`) utilisée dans HTML.
	#[error("Le caractère `?` est inattendu, un nom de balise est attendu")]
	UnexpectedQuestionMarkInsteadOfTagName,

	/// Cette erreur se produit si l'analyseur syntaxique rencontre un point de
	/// code U+0000 NULL dans le flux d'entrée à certaines positions. En
	/// général, ces points de code sont soit ignorés, soit, pour des raisons
	/// de sécurité, remplacés par un CHARACTER DE REMPLACEMENT U+FFFD.
	#[error("Caractère NULL inattendu")]
	UnexpectedNullCharacter,

	#[error("Unknown")]
	Unknown,
}

// -------------- //
// Implémentation //
// -------------- //

impl HTMLLexicalError
{
	pub const fn idk() -> Self
	{
		Self {
			variant: HTMLLexicalErrorVariant::Unknown,
			location: Location::new(),
		}
	}

	pub const fn unexpected_null_character() -> Self
	{
		Self {
			variant: HTMLLexicalErrorVariant::UnexpectedNullCharacter,
			location: Location::new(),
		}
	}

	pub const fn unexpected_question_mark_instead_of_tag_name() -> Self
	{
		Self {
			variant:
				HTMLLexicalErrorVariant::UnexpectedQuestionMarkInsteadOfTagName,
			location: Location::new(),
		}
	}
}

impl HTMLLexicalError
{
	pub fn with_location(mut self, location: Location) -> Self
	{
		self.location = location;
		self
	}
}

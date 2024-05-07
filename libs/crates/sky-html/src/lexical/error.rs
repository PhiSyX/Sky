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
	/// Cette erreur se produit si l'analyseur syntaxique rencontre la fin du
	/// flux d'entrée où un nom de balise est attendu. Dans ce cas, l'analyseur
	/// traite le début d'une balise de début (<) ou d'une balise de fin (</)
	/// comme du contenu textuel.
	#[error("Fin du flux avant le nom de balise")]
	EndOfStreamBeforeTagName,

	/// Cette erreur se produit si l'analyseur syntaxique rencontre la fin
	/// du flux d'entrée dans une balise de début ou une balise de fin
	/// (par exemple, `<div id=`). Une telle balise est ignorée.
	#[error("Fin du flux dans la balise")]
	EndOfStreamInTag,

	/// Cette erreur se produit si l'analyseur rencontre un point de code
	/// qui n'est pas un alpha ASCII où le premier point de code d'une
	/// balise de début ou d'une balise de fin est attendu. Si une balise
	/// de début était attendue, ce point de code et un U+003C (<) qui le
	/// précède sont traités comme du contenu texte, et tout le contenu
	/// qui suit est traité comme du balisage. En revanche, si une balise
	/// de fin était attendue, ce point de code et tout le contenu qui
	/// suit jusqu'à un point de code U+003E (>) (s'il est présent) ou
	/// jusqu'à la fin du flux d'entrée est traité comme un commentaire.
	///
	/// Example: `<42></42>`
	///
	/// Parsed into:
	///   |- html
	///      |- head
	///      |- body
	///         |- #text: <42>
	///         |- #comment: 42
	///
	/// NOTE(html): alors que le premier point de code d'un nom de balise est
	/// limité à un alpha ASCII, un large éventail de points de code (y
	/// compris des chiffres ASCII) est autorisé dans les positions
	/// suivantes.
	#[error(
		"Le premier caractère du nom de la balise '{found}' est invalide, \
		 caractère alphabétique attendu"
	)]
	InvalidFirstCharacterOfTagName
	{
		found: char
	},

	/// Cette erreur se produit si l'analyseur rencontre un point de code
	/// U+003E (>) là où un nom de balise de fin est attendu, c'est-à-dire </>.
	/// L'analyseur syntaxique ignore l'ensemble de la séquence de points de
	/// code "</>".
	#[error("Caractère '>' manquant")]
	MissingEndTagName,

	/// Cette erreur se produit si l'analyseur syntaxique rencontre un point de
	/// code U+0022 ("), U+0027 (') ou U+003C (<) dans un nom d'attribut.
	/// L'analyseur syntaxique inclut ces points de code dans le nom de
	/// l'attribut.
	///
	/// NOTE(html): les points de code qui déclenchent cette erreur font
	/// généralement partie d'une autre construction syntaxique et peuvent être
	/// le signe d'une faute de frappe autour du nom de l'attribut.
	///
	/// Example: `<div foo<div>` En raison d'un point de code U+003E (>) oublié
	/// après foo, l'analyseur syntaxique traite ce balisage comme un seul
	/// élément div avec un attribut "foo<div".
	///
	/// Example: `<div id'bar'>` En raison d'un point de code U+003D (=) oublié
	/// entre un nom d'attribut et une valeur, l'analyseur syntaxique traite ce
	/// balisage comme un élément div dont l'attribut "id'bar'" a une valeur
	/// vide.
	#[error("Caractère '{found}' inattendu dans le nom d'un attribut")]
	UnexpectedCharacterInAttributeName
	{
		found: char
	},

	/// Cette erreur se produit si l'analyseur syntaxique rencontre un point de
	/// code U+003D (=) avant un nom d'attribut. Dans ce cas, l'analyseur
	/// syntaxique traite U+003D (=) comme le premier point de code du nom de
	/// l'attribut.
	///
	/// NOTE(html): la raison courante de cette erreur est un nom d'attribut
	/// oublié.
	///
	/// Example: `<div foo="bar" ="baz">` En raison d'un nom d'attribut oublié,
	/// l'analyseur syntaxique traite ce balisage comme un élément div avec
	/// deux attributs : un attribut "foo" avec une valeur "bar" et un attribut
	/// "="baz"" avec une valeur vide.
	#[error("Caractère '=' inattendu avant le nom d'un attribut")]
	UnexpectedEqualsSignBeforeAttributeName,

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
	pub const fn end_of_stream_before_tag_name() -> Self
	{
		Self {
			variant: HTMLLexicalErrorVariant::EndOfStreamBeforeTagName,
			location: Location::new(),
		}
	}

	pub const fn end_of_stream_in_tag() -> Self
	{
		Self {
			variant: HTMLLexicalErrorVariant::EndOfStreamInTag,
			location: Location::new(),
		}
	}

	pub const fn idk() -> Self
	{
		Self {
			variant: HTMLLexicalErrorVariant::Unknown,
			location: Location::new(),
		}
	}

	pub const fn invalid_first_character_of_tag_name(found: char) -> Self
	{
		Self {
			variant: HTMLLexicalErrorVariant::InvalidFirstCharacterOfTagName {
				found,
			},
			location: Location::new(),
		}
	}

	pub const fn missing_end_tag_name() -> Self
	{
		Self {
			variant: HTMLLexicalErrorVariant::MissingEndTagName,
			location: Location::new(),
		}
	}

	pub const fn unexpected_character_in_attribute(found: char) -> Self
	{
		Self {
			variant:
				HTMLLexicalErrorVariant::UnexpectedCharacterInAttributeName {
					found,
				},
			location: Location::new(),
		}
	}

	pub const fn unexpected_equals_sign_before_attribute_name() -> Self
	{
		Self {
			variant:
				HTMLLexicalErrorVariant::UnexpectedEqualsSignBeforeAttributeName,
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

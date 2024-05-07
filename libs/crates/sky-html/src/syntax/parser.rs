// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use sky_lang::UnitCodePointExt;

use super::HTMLParserError;
use crate::lexical::HTMLTokenizer;
use crate::HTMLToken;

// --------- //
// Structure //
// --------- //

pub struct HTMLParser<Input: Iterator>
{
	tokenizer: HTMLTokenizer<Input>,
}

// -------------- //
// Implémentation //
// -------------- //

impl<Input: Iterator> HTMLParser<Input>
{
	pub fn new(input: Input) -> Self
	{
		let tokenizer = HTMLTokenizer::new(input);
		Self { tokenizer }
	}
}

impl<Input: Iterator> HTMLParser<Input>
where
	Input::Item: UnitCodePointExt,
{
	// TODO: tâche non aboutie.
	pub fn parse(mut self) -> Result<Vec<HTMLToken>, HTMLParserError>
	{
		let mut ret_tokens = Vec::new();

		'tokenizer: while let Ok(tokens) = self.tokenizer.consume_next() {
			for token in tokens {
				if token.is_end_of_stream() {
					break 'tokenizer;
				}
				ret_tokens.push(token);
			}
		}

		Ok(ret_tokens)
	}
}

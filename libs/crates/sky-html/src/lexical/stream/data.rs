// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::ops::ControlFlow;

use sky_lang::UnitCodePointExt;

use crate::lexical::error::HTMLLexicalError;
use crate::lexical::token::HTMLToken;
use crate::lexical::tokenizer::{
	HTMLTokenizerErr,
	HTMLTokenizerOk,
	HTMLTokenizerState,
};
use crate::lexical::HTMLTokenizer;

// --------- //
// Interface //
// --------- //

pub trait HTMLTokenizerDataStream
{
	fn handle_data_state(
		&mut self,
	) -> ControlFlow<HTMLTokenizerErr, HTMLTokenizerOk>;
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<Input: Iterator> HTMLTokenizerDataStream for HTMLTokenizer<Input>
where
	Input::Item: UnitCodePointExt,
{
	fn handle_data_state(
		&mut self,
	) -> ControlFlow<HTMLTokenizerErr, HTMLTokenizerOk>
	{
		match self.input.consume_next() {
			// TODO: character reference
			| Some(cp) if cp.is('&') => {
				ControlFlow::Continue(HTMLTokenizerOk::None)
			}

			| Some(cp) if cp.is('<') => {
				self.current_state.switch(HTMLTokenizerState::TagOpen);
				ControlFlow::Continue(HTMLTokenizerOk::None)
			}

			| Some(cp) if !cp.is__valid() => {
				ControlFlow::Break(HTMLTokenizerErr::Emit(
					HTMLLexicalError::unexpected_null_character(),
				))
			}

			| Some(cp) => {
				ControlFlow::Continue(HTMLTokenizerOk::Emit(
					HTMLToken::character(cp.unit()),
				))
			}

			| None => {
				ControlFlow::Continue(HTMLTokenizerOk::Emit(
					HTMLToken::end_of_stream(),
				))
			}
		}
	}
}

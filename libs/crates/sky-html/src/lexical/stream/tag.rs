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
use crate::lexical::tokenizer::{
	HTMLTokenizerErr,
	HTMLTokenizerOk,
	HTMLTokenizerState,
};
use crate::lexical::HTMLTokenizer;
use crate::HTMLToken;

// --------- //
// Interface //
// --------- //

pub trait HTMLTokenizerTagStream
{
	fn handle_tag_open_state(
		&mut self,
	) -> ControlFlow<HTMLTokenizerErr, HTMLTokenizerOk>;
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<Input: Iterator> HTMLTokenizerTagStream for HTMLTokenizer<Input>
where
	Input::Item: UnitCodePointExt,
{
	fn handle_tag_open_state(
		&mut self,
	) -> ControlFlow<HTMLTokenizerErr, HTMLTokenizerOk>
	{
		match self.input.consume_next() {
			| Some(cp) if cp.is('!') => {
				self.current_state
					.switch(HTMLTokenizerState::MarkupDeclarationOpen);
				ControlFlow::Continue(HTMLTokenizerOk::None)
			}

			| Some(cp) if cp.is('/') => {
				self.current_state.switch(HTMLTokenizerState::EndTagOpen);
				ControlFlow::Continue(HTMLTokenizerOk::None)
			}

			| Some(cp) if cp.is('?') => {
				self.reconsume(HTMLTokenizerState::BogusComment);
				ControlFlow::Break(
					HTMLTokenizerErr::Update(
						HTMLToken::empty_comment(),
						HTMLLexicalError::unexpected_question_mark_instead_of_tag_name(),
					)
				)
			}

			| Some(cp) if cp.is__alphabetic() => {
				self.reconsume(HTMLTokenizerState::TagName);
				ControlFlow::Continue(HTMLTokenizerOk::Update(
					HTMLToken::start_tag(),
				))
			}

			| Some(cp) => {
				self.reconsume(HTMLTokenizerState::Data);
				ControlFlow::Continue(HTMLTokenizerOk::EmitWithError(
					HTMLToken::character('<'),
					HTMLLexicalError::invalid_first_character_of_tag_name(
						cp.unit(),
					),
				))
			}

			| None => {
				ControlFlow::Continue(HTMLTokenizerOk::Full {
					emit: HTMLToken::character('<'),
					update: HTMLToken::end_of_stream(),
					error: HTMLLexicalError::end_of_stream_before_tag_name(),
				})
			}
		}
	}
}

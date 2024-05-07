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

	fn handle_end_tag_open_state(
		&mut self,
	) -> ControlFlow<HTMLTokenizerErr, HTMLTokenizerOk>;

	fn handle_tag_name_state(
		&mut self,
	) -> ControlFlow<HTMLTokenizerErr, HTMLTokenizerOk>;

	fn handle_before_attribute_name_state(
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

	fn handle_end_tag_open_state(
		&mut self,
	) -> ControlFlow<HTMLTokenizerErr, HTMLTokenizerOk>
	{
		match self.input.consume_next() {
			| Some(cp) if cp.is__alphabetic() => {
				self.reconsume(HTMLTokenizerState::TagName);
				ControlFlow::Continue(HTMLTokenizerOk::Update(
					HTMLToken::end_tag(),
				))
			}

			| Some(cp) if cp.is('>') => {
				self.current_state.switch(HTMLTokenizerState::Data);
				ControlFlow::Break(HTMLTokenizerErr::Emit(
					HTMLLexicalError::missing_end_tag_name(),
				))
			}

			| Some(cp) => {
				self.reconsume(HTMLTokenizerState::BogusComment);
				ControlFlow::Break(HTMLTokenizerErr::Update(
					HTMLToken::empty_comment(),
					HTMLLexicalError::invalid_first_character_of_tag_name(
						cp.unit(),
					),
				))
			}

			| None => {
				ControlFlow::Continue(HTMLTokenizerOk::ManyEmitWithError(
					vec![
						HTMLToken::character('<'),
						HTMLToken::character('/'),
						HTMLToken::end_of_stream(),
					],
					HTMLLexicalError::end_of_stream_before_tag_name(),
				))
			}
		}
	}

	fn handle_tag_name_state(
		&mut self,
	) -> ControlFlow<HTMLTokenizerErr, HTMLTokenizerOk>
	{
		let update_tag_name = |ch: char| {
			move |token: &mut HTMLToken| {
				assert!(token.is_opened_tag());
				token.add_character_to_tag_name(ch);
			}
		};

		match self.input.consume_next() {
			| Some(cp) if cp.is__whitespace() => {
				self.current_state
					.switch(HTMLTokenizerState::BeforeAttributeName);
				ControlFlow::Continue(HTMLTokenizerOk::None)
			}

			| Some(cp) if cp.is('/') => {
				self.current_state
					.switch(HTMLTokenizerState::SelfClosingStartTag);
				ControlFlow::Continue(HTMLTokenizerOk::None)
			}

			| Some(cp) if cp.is('>') => {
				self.current_state.switch(HTMLTokenizerState::Data);
				ControlFlow::Continue(HTMLTokenizerOk::EmitCurrent)
			}

			| Some(cp) if cp.is__upper_alphabetic() => {
				ControlFlow::Continue(HTMLTokenizerOk::UpdateFn(Box::new(
					update_tag_name(cp.unit().to_ascii_lowercase()),
				)))
			}

			| Some(cp) if !cp.is__valid() => {
				ControlFlow::Continue(HTMLTokenizerOk::UpdateFnWithError(
					Box::new(update_tag_name(char::REPLACEMENT_CHARACTER)),
					HTMLLexicalError::unexpected_null_character(),
				))
			}

			| Some(cp) => {
				ControlFlow::Continue(HTMLTokenizerOk::UpdateFn(Box::new(
					update_tag_name(cp.unit()),
				)))
			}

			| None => {
				ControlFlow::Continue(HTMLTokenizerOk::EmitWithError(
					HTMLToken::end_of_stream(),
					HTMLLexicalError::end_of_stream_in_tag(),
				))
			}
		}
	}

	fn handle_before_attribute_name_state(
		&mut self,
	) -> ControlFlow<HTMLTokenizerErr, HTMLTokenizerOk>
	{
		match self.input.consume_next() {
			| Some(cp) if cp.is__whitespace() => {
				ControlFlow::Continue(HTMLTokenizerOk::None)
			}

			| Some(cp) if cp.one_of(['/', '>']) => {
				self.reconsume(HTMLTokenizerState::AfterAttributeName);
				ControlFlow::Continue(HTMLTokenizerOk::None)
			}

			| Some(cp) if cp.is('=') => {
				self.current_state.switch(HTMLTokenizerState::AttributeName);

				let ch = cp.unit();
				ControlFlow::Continue(HTMLTokenizerOk::UpdateFnWithError(
					Box::new(move |token| {
						assert!(token.is_opened_tag());
						token.start_attribute_tag_with(ch);
					}),
					HTMLLexicalError::unexpected_equals_sign_before_attribute_name(),
				))
			}

			| Some(_) => {
				self.reconsume(HTMLTokenizerState::AttributeName);
				ControlFlow::Continue(HTMLTokenizerOk::UpdateFn(Box::new(
					|token| {
						assert!(token.is_opened_tag());
						token.start_empty_attribute_for_tag();
					},
				)))
			}

			| None => {
				self.reconsume(HTMLTokenizerState::AfterAttributeName);
				ControlFlow::Continue(HTMLTokenizerOk::None)
			}
		}
	}
}

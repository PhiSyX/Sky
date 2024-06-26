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

use sky_lang::{InputStream, Location, UnitCodePointExt};

use super::error::HTMLLexicalError;
use super::stream::{HTMLTokenizerDataStream, HTMLTokenizerTagStream};
use super::token::HTMLToken;

// --------- //
// Structure //
// --------- //

pub struct HTMLTokenizer<Input: Iterator>
{
	pub(crate) input: InputStream<Input>,
	/// Jeton courant, ce jeton (peut / va) être modifié au fur & à mesure des
	/// états.
	current_token: Option<HTMLToken>,
	/// Position du jeton, modifié à chaque itération.
	pub(crate) current_location: Location,
	/// État courant du fournisseur de jeton.
	pub(crate) current_state: HTMLTokenizerState,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum HTMLTokenizerState
{
	AfterAttributeName,
	AttributeName,
	AttributeValue
	{
		// NOTE: quoted   = Some('\'')
		// NOTE: quoted   = Some('"')
		// NOTE: unquoted = None
		quote: Option<char>,
	},
	AfterAttributeValue
	{
		// NOTE: quoted   = Some('\'')
		// NOTE: quoted   = Some('"')
		// NOTE: unquoted = None
		quote: Option<char>,
	},
	BeforeAttributeName,
	BeforeAttributeValue,
	BogusComment,
	Data,
	EndTagOpen,
	MarkupDeclarationOpen,
	SelfClosingStartTag,
	TagName,
	TagOpen,
}

pub enum HTMLTokenizerOk
{
	Update(HTMLToken),
	UpdateFn(Box<dyn Fn(&mut HTMLToken)>),
	UpdateFnWithError(Box<dyn Fn(&mut HTMLToken)>, HTMLLexicalError),

	Emit(HTMLToken),
	EmitCurrent,
	EmitCurrentWithError(HTMLLexicalError),
	EmitWithError(HTMLToken, HTMLLexicalError),
	ManyEmitWithError(Vec<HTMLToken>, HTMLLexicalError),

	Full
	{
		emit: HTMLToken,
		update: HTMLToken,
		error: HTMLLexicalError,
	},

	None,
}

pub enum HTMLTokenizerErr
{
	Update(HTMLToken, HTMLLexicalError),
	Emit(HTMLLexicalError),
}

// -------------- //
// Implémentation //
// -------------- //

impl<Input: Iterator> HTMLTokenizer<Input>
{
	pub fn new(input: Input) -> Self
	{
		Self {
			input: InputStream::new(input),
			current_token: Default::default(),
			current_location: Location::default(),
			current_state: HTMLTokenizerState::Data,
		}
	}
}

impl<Input: Iterator> HTMLTokenizer<Input>
where
	Input::Item: UnitCodePointExt,
{
	pub fn consume_next(&mut self) -> Result<Vec<HTMLToken>, HTMLLexicalError>
	{
		loop {
			if let Some(cp) = self.input.peek_next() {
				if cp.is__tab() {
					self.current_location.increment_column_by(4);
				} else if cp.is__linefeed() {
					self.current_location.increment_line().reset_column();
				} else if !cp.is__carriage_return() {
					self.current_location.increment_column();
				}
			}

			#[rustfmt::skip]
			let control_flow = match self.current_state {
				// 13.2.5.1 Data state
				| HTMLTokenizerState::Data => self.handle_data_state(),
				// 13.2.5.6 Tag open state
				| HTMLTokenizerState::TagOpen => self.handle_tag_open_state(),
				// 13.2.5.7 End tag open state
				| HTMLTokenizerState::EndTagOpen => self.handle_end_tag_open_state(),
				// 13.2.5.8 Tag name state
				| HTMLTokenizerState::TagName => self.handle_tag_name_state(),
				// 13.2.5.32 Before attribute name state
				| HTMLTokenizerState::BeforeAttributeName => self.handle_before_attribute_name_state(),
				// 13.2.5.33 Attribute name state
				| HTMLTokenizerState::AttributeName => self.handle_attribute_name_state(),
				// 13.2.5.34 After attribute name state
				| HTMLTokenizerState::AfterAttributeName => self.handle_after_attribute_name_state(),
				// 13.2.5.35 Before attribute value state
				| HTMLTokenizerState::BeforeAttributeValue => self.handle_before_attribute_value_state(),
				// 13.2.5.36 Attribute value (double-quoted) state
				// 13.2.5.37 Attribute value (single-quoted) state
				// 13.2.5.38 Attribute value (unquoted) state
				| HTMLTokenizerState::AttributeValue { quote } => self.handle_attribute_value_state(quote),
				// 13.2.5.39 After attribute value (quoted) state
				| HTMLTokenizerState::AfterAttributeValue { quote } => self.handle_after_attribute_value_state(quote),

				| _ => return {
					Ok(vec![HTMLToken::end_of_stream().with_location(self.current_location)])
				},
			};

			match control_flow {
				| ControlFlow::Continue(ok_flow) => {
					match ok_flow {
						| HTMLTokenizerOk::Update(token) => {
							self.current_token.replace(
								token.with_location(self.current_location),
							);
						}
						| HTMLTokenizerOk::UpdateFn(update) => {
							if let Some(token) = self.current_token.as_mut() {
								update(token);
							}
						}
						// TODO: améliorer la gestion d'erreur.
						| HTMLTokenizerOk::UpdateFnWithError(update, err) => {
							eprintln!(
								"HTMLTokenizer error: {}",
								err.with_location(self.current_location)
							);

							if let Some(token) = self.current_token.as_mut() {
								update(token);
							}
						}

						| HTMLTokenizerOk::Emit(token) => {
							return Ok(vec![
								token.with_location(self.current_location)
							]);
						}

						| HTMLTokenizerOk::EmitCurrent => {
							break;
						}
						// TODO: améliorer la gestion d'erreur.
						| HTMLTokenizerOk::EmitCurrentWithError(err) => {
							eprintln!(
								"HTMLTokenizer error: {}",
								err.with_location(self.current_location)
							);

							break;
						}

						// TODO: améliorer la gestion d'erreur.
						| HTMLTokenizerOk::EmitWithError(token, err) => {
							eprintln!(
								"HTMLTokenizer error: {}",
								err.with_location(self.current_location)
							);
							return Ok(vec![
								token.with_location(self.current_location)
							]);
						}
						// TODO: améliorer la gestion d'erreur.
						| HTMLTokenizerOk::ManyEmitWithError(tokens, err) => {
							eprintln!(
								"HTMLTokenizer error: {}",
								err.with_location(self.current_location)
							);
							return Ok(tokens);
						}

						// TODO: améliorer la gestion d'erreur.
						| HTMLTokenizerOk::Full {
							emit,
							update,
							error,
						} => {
							self.current_token.replace(
								update.with_location(self.current_location),
							);

							eprintln!(
								"HTMLTokenizer error: {}",
								error.with_location(self.current_location)
							);

							return Ok(vec![
								emit.with_location(self.current_location)
							]);
						}

						| HTMLTokenizerOk::None => continue,
					}
				}

				| ControlFlow::Break(err_flow) => {
					match err_flow {
						| HTMLTokenizerErr::Update(token, err) => {
							self.current_token.replace(token);
							return Err(
								err.with_location(self.current_location)
							);
						}

						| HTMLTokenizerErr::Emit(err) => {
							return Err(err.with_location(self.current_location))
						}
					}
				}
			}
		}

		self.current_token
			.as_ref()
			.map(|t| vec![t.to_owned()])
			.clone()
			.ok_or(HTMLLexicalError::idk().with_location(self.current_location))
	}

	pub fn reconsume(&mut self, state: HTMLTokenizerState)
	{
		self.input.rollback_once();
		self.current_state.switch(state);
	}
}

impl HTMLTokenizerState
{
	pub(crate) fn switch(&mut self, new_state: Self)
	{
		*self = new_state;
	}
}

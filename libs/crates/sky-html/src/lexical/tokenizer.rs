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
	current_token: Option<HTMLToken>,
	pub(crate) current_location: Location,
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
	Data,
	TagOpen,
pub enum HTMLTokenizerOk
{
	Emit(HTMLToken),
	None,
}

pub enum HTMLTokenizerErr
{
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
				|_ => return {
					Ok(vec![HTMLToken::end_of_stream().with_location(self.current_location)])
				},
			};

			match control_flow {
				| ControlFlow::Continue(ok_flow) => {
					match ok_flow {
						| HTMLTokenizerOk::Emit(token) => {
							return Ok(vec![
								token.with_location(self.current_location)
							]);
						}
					}
				}

				| ControlFlow::Break(err_flow) => {
					match err_flow {
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
}

impl HTMLTokenizerState
{
	pub(crate) fn switch(&mut self, new_state: Self)
	{
		*self = new_state;
	}
}

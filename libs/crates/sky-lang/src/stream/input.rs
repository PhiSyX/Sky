// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::collections::VecDeque;
use std::ops::RangeInclusive;

use crate::codepoint::UnitCodePointExt;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
pub struct InputStream<Input: Iterator>
{
	source: Input,
	source_idx: usize,
	current_item: Option<Input::Item>,
	temporary_list: VecDeque<Input::Item>,
	look_ahead_offset: u8,
	recycle_list: Vec<Input::Item>,
	should_continue_peek: usize,
}

// -------------- //
// Implémentation //
// -------------- //

impl<Input: Iterator> InputStream<Input>
{
	pub fn new(input: Input) -> Self
	{
		Self {
			source: input,
			source_idx: 0,
			current_item: Default::default(),
			temporary_list: Default::default(),
			look_ahead_offset: Default::default(),
			recycle_list: Default::default(),
			should_continue_peek: Default::default(),
		}
	}
}

impl<Input: Iterator> InputStream<Input>
{
	fn decrement_look_ahead(&mut self)
	{
		self.look_ahead_offset = self.look_ahead_offset.saturating_sub(1);
	}

	fn enqueue(&mut self)
	{
		if let Some(item) = self.source.next() {
			self.temporary_list.push_back(item);
		}
	}

	fn fill(&mut self, required_elements: u8)
	{
		let stored_elements = self.temporary_list.len();
		if stored_elements <= required_elements as usize {
			for _ in stored_elements..=required_elements as usize {
				self.enqueue();
			}
		}
	}

	fn take_or_next(&mut self) -> Option<Input::Item>
	{
		if self.temporary_list.is_empty() {
			self.source_idx += 1;
			return self.source.next();
		}
		self.temporary_list.pop_front()
	}
}

impl<Input> InputStream<Input>
where
	Input: Iterator,
	Input::Item: Clone,
{
	pub fn advance(&mut self, n: usize)
	{
		for _ in 0..=n {
			self.consume_next();
		}
	}

	pub fn consume_next(&mut self) -> Option<Input::Item>
	{
		let next_item = self.take_or_next();
		self.decrement_look_ahead();
		self.current_item.clone_from(&next_item);
		self.recycle_list.extend(next_item.clone());
		next_item
	}

	pub fn current(&self) -> Option<&Input::Item>
	{
		self.current_item.as_ref()
	}

	pub fn is_peek(&mut self, cb: impl Fn(&Input::Item) -> bool) -> bool
	{
		self.peek_next().filter(cb).is_some()
	}

	pub fn is_peek_char(&mut self, expected_char: char) -> bool
	where
		Input::Item: UnitCodePointExt,
	{
		self.peek_next()
			.filter(|item| item.is(expected_char))
			.is_some()
	}

	pub fn is_peek_chars(&mut self, rng_chars: RangeInclusive<char>) -> bool
	where
		Input::Item: UnitCodePointExt,
	{
		self.peek_next()
			.filter(|item| rng_chars.contains(&item.unit()))
			.is_some()
	}

	pub fn peek_item(
		&mut self,
		expected_item: Input::Item,
	) -> Option<Input::Item>
	where
		Input::Item: UnitCodePointExt,
	{
		self.peek_next()
			.filter(|item| item.is(expected_item.unit()))
	}

	pub fn peek_next(&mut self) -> Option<Input::Item>
	{
		self.fill(self.look_ahead_offset);
		self.temporary_list
			.get(self.look_ahead_offset as usize)
			.cloned()
	}

	pub fn peek_n(&mut self, n: usize) -> Option<String>
	where
		Input::Item: UnitCodePointExt,
	{
		if self.should_continue_peek == 0 {
			self.fill(n as u8);
		}

		self.should_continue_peek = n;

		let stru = self
			.temporary_list
			.iter()
			.take(n)
			.map(|item| item.unit())
			.collect::<String>();

		Option::from(stru)
	}

	pub fn reset_peek(&mut self)
	{
		self.should_continue_peek = 0;
	}

	pub fn rollback_once(&mut self)
	{
		let Some(last) = self.recycle_list.pop() else {
			return;
		};
		self.temporary_list.push_front(last);
	}
}

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests
{
	use super::*;

	const SOURCE: &str = "Hello World\n";

	#[test]
	fn test_consume_next()
	{
		// Chars

		let mut input_stream = InputStream::new(SOURCE.chars());
		assert_eq!(input_stream.consume_next(), Some('H'));
		assert_eq!(input_stream.consume_next(), Some('e'));
		assert_eq!(input_stream.consume_next(), Some('l'));
		assert_eq!(input_stream.consume_next(), Some('l'));
		assert_eq!(input_stream.consume_next(), Some('o'));

		assert_eq!(input_stream.consume_next(), Some(' '));

		assert_eq!(input_stream.consume_next(), Some('W'));
		assert_eq!(input_stream.consume_next(), Some('o'));
		assert_eq!(input_stream.consume_next(), Some('r'));
		assert_eq!(input_stream.consume_next(), Some('l'));
		assert_eq!(input_stream.consume_next(), Some('d'));

		assert_eq!(input_stream.consume_next(), Some('\n'));

		assert_eq!(input_stream.consume_next(), None);
		assert_eq!(input_stream.consume_next(), None);
		assert_eq!(input_stream.consume_next(), None);
		assert_eq!(input_stream.consume_next(), None);
		assert_eq!(input_stream.consume_next(), None);
		assert_eq!(input_stream.consume_next(), None);

		// Bytes

		let mut input_stream = InputStream::new(SOURCE.bytes());

		assert_eq!(input_stream.consume_next(), Some(b'H'));
		assert_eq!(input_stream.consume_next(), Some(b'e'));
		assert_eq!(input_stream.consume_next(), Some(b'l'));
		assert_eq!(input_stream.consume_next(), Some(b'l'));
		assert_eq!(input_stream.consume_next(), Some(b'o'));

		assert_eq!(input_stream.consume_next(), Some(b' '));

		assert_eq!(input_stream.consume_next(), Some(b'W'));
		assert_eq!(input_stream.consume_next(), Some(b'o'));
		assert_eq!(input_stream.consume_next(), Some(b'r'));
		assert_eq!(input_stream.consume_next(), Some(b'l'));
		assert_eq!(input_stream.consume_next(), Some(b'd'));

		assert_eq!(input_stream.consume_next(), Some(b'\n'));

		assert_eq!(input_stream.consume_next(), None);
		assert_eq!(input_stream.consume_next(), None);
		assert_eq!(input_stream.consume_next(), None);
		assert_eq!(input_stream.consume_next(), None);
		assert_eq!(input_stream.consume_next(), None);
		assert_eq!(input_stream.consume_next(), None);
	}

	#[test]
	fn test_peek_next()
	{
		let mut input_stream = InputStream::new(SOURCE.chars());
		assert_eq!(input_stream.consume_next(), Some('H'));
		assert_eq!(input_stream.consume_next(), Some('e'));
		assert_eq!(input_stream.consume_next(), Some('l'));
		assert_eq!(input_stream.consume_next(), Some('l'));

		// Peek
		assert_eq!(input_stream.peek_next(), Some('o'));
		assert_eq!(input_stream.peek_next(), Some('o'));
		assert_eq!(input_stream.peek_next(), Some('o'));
		assert_eq!(input_stream.peek_next(), Some('o'));
		assert_eq!(input_stream.peek_next(), Some('o'));
		assert_eq!(input_stream.peek_next(), Some('o'));
		assert_eq!(input_stream.peek_next(), Some('o'));
		assert_eq!(input_stream.peek_next(), Some('o'));
		assert_eq!(input_stream.peek_next(), Some('o'));

		assert_eq!(input_stream.consume_next(), Some('o'));

		assert_eq!(input_stream.consume_next(), Some(' '));

		assert_eq!(input_stream.consume_next(), Some('W'));
		assert_eq!(input_stream.consume_next(), Some('o'));
		assert_eq!(input_stream.consume_next(), Some('r'));
		assert_eq!(input_stream.consume_next(), Some('l'));
		assert_eq!(input_stream.consume_next(), Some('d'));

		assert_eq!(input_stream.consume_next(), Some('\n'));

		// Peek on none
		assert_eq!(input_stream.peek_next(), None);
		assert_eq!(input_stream.consume_next(), None);
		assert_eq!(input_stream.consume_next(), None);
		assert_eq!(input_stream.consume_next(), None);
		assert_eq!(input_stream.consume_next(), None);
		assert_eq!(input_stream.consume_next(), None);
	}

	#[test]
	fn test_rollback_once()
	{
		let mut input_stream = InputStream::new("PhiSyX".chars());
		assert_eq!(input_stream.consume_next(), Some('P'));
		assert_eq!(input_stream.consume_next(), Some('h'));
		assert_eq!(input_stream.consume_next(), Some('i'));
		assert_eq!(input_stream.consume_next(), Some('S'));

		input_stream.rollback_once(); // NOTE: effet
		input_stream.rollback_once(); // NOTE: effet
		assert_eq!(input_stream.consume_next(), Some('i'));
		assert_eq!(input_stream.consume_next(), Some('S'));
		assert_eq!(input_stream.consume_next(), Some('y'));
		assert_eq!(input_stream.consume_next(), Some('X'));
	}
}

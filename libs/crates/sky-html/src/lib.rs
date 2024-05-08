// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

#![allow(dead_code)]

mod lexical;
mod syntax;

use std::borrow::Cow;
use std::collections::BTreeMap;

use html5ever::interface::*;
use html5ever::tendril::*;
pub use html5ever::Attribute;
use html5ever::*;

pub use self::lexical::{HTMLLexicalError, HTMLToken};
pub use self::syntax::{HTMLParser, HTMLParserError};

// --------- //
// Structure //
// --------- //

pub struct HTMLDocument
{
	next_id: usize,
	pub elements: BTreeMap<usize, HTMLElement>,
}

#[derive(Debug)]
pub struct HTMLElement
{
	pub name: QualName,
	pub attributes: Vec<Attribute>,
	pub text: Option<Tendril<fmt::UTF8>>,
	pub parent: usize,
}

// -------------- //
// Implémentation //
// -------------- //

impl HTMLDocument
{
	pub fn from_file(
		filepath: impl AsRef<std::path::Path>,
	) -> Result<Self, HTMLParserError>
	{
		let this = Self {
			next_id: 1,
			elements: BTreeMap::new(),
		};

		let this = html5ever::parse_document(this, Default::default())
			.from_utf8()
			.from_file(filepath.as_ref())?;

		Ok(this)
	}

	pub fn from_slice(
		slice: &mut &[u8],
	) -> Result<Self, HTMLParserError>
	{
		let this = Self {
			next_id: 1,
			elements: BTreeMap::new(),
		};

		let this = html5ever::parse_document(this, Default::default())
			.from_utf8()
			.read_from(slice)?;

		Ok(this)
	}

	pub fn from_stream(
		stream: &mut impl std::io::Read,
	) -> Result<Self, HTMLParserError>
	{
		let this = Self {
			next_id: 1,
			elements: BTreeMap::new(),
		};

		let this = html5ever::parse_document(this, Default::default())
			.from_utf8()
			.read_from(stream)?;

		Ok(this)
	}

	fn get_id(&mut self) -> usize
	{
		let id = self.next_id;
		self.next_id += 2;
		id
	}
}

impl TreeSink for HTMLDocument
{
	type Handle = usize;
	type Output = Self;

	fn finish(self) -> Self
	{
		self
	}

	fn parse_error(&mut self, _: Cow<'static, str>) {}

	fn get_document(&mut self) -> usize
	{
		0
	}

	fn get_template_contents(&mut self, target: &usize) -> usize
	{
		if let Some(expanded_name!(html "template")) =
			self.elements.get(target).map(|el| el.name.expanded())
		{
			target + 1
		} else {
			panic!("n'est pas un élément template")
		}
	}

	fn set_quirks_mode(&mut self, _: QuirksMode) {}

	fn same_node(&self, x: &usize, y: &usize) -> bool
	{
		x == y
	}

	fn elem_name(&self, target: &usize) -> ExpandedName
	{
		self.elements
			.get(target)
			.expect("n'est pas un élément")
			.name
			.expanded()
	}

	fn create_element(
		&mut self,
		name: QualName,
		attributes: Vec<Attribute>,
		_: ElementFlags,
	) -> usize
	{
		let id = self.get_id();
		self.elements.insert(
			id,
			HTMLElement {
				name,
				attributes,
				text: Default::default(),
				parent: Default::default(),
			},
		);
		id
	}

	fn create_comment(&mut self, _: StrTendril) -> usize
	{
		self.get_id()
	}

	fn create_pi(&mut self, _: StrTendril, _: StrTendril) -> usize
	{
		unimplemented!()
	}

	fn append(&mut self, parent: &usize, child: NodeOrText<usize>)
	{
		match child {
			| AppendNode(node_id) => {
				match self.elements.get_mut(&node_id) {
					| Some(node_el) => node_el.parent = *parent,
					| None => {}
				}
			}
			| AppendText(text) => {
				match self.elements.get_mut(parent) {
					| Some(parent_el) => {
						if let Some(tendril) = parent_el.text.as_mut() {
							tendril.push_tendril(&text);
						} else {
							parent_el.text.replace(text);
						}
					}
					| None => {}
				}
			}
		}
	}

	fn append_before_sibling(&mut self, _: &usize, _: NodeOrText<usize>) {}

	fn append_based_on_parent_node(
		&mut self,
		element: &Self::Handle,
		_: &Self::Handle,
		child: NodeOrText<Self::Handle>,
	)
	{
		self.append_before_sibling(element, child);
	}

	fn append_doctype_to_document(
		&mut self,
		_: StrTendril,
		_: StrTendril,
		_: StrTendril,
	)
	{
	}

	fn add_attrs_if_missing(&mut self, target: &usize, attrs: Vec<Attribute>)
	{
		if !self.elements.contains_key(target) {
			return;
		}

		for attr in attrs.into_iter() {
			println!("    {:?} = {}", attr.name, attr.value);
		}
	}

	fn associate_with_form(
		&mut self,
		_: &usize,
		_: &usize,
		_: (&usize, Option<&usize>),
	)
	{
	}

	fn remove_from_parent(&mut self, _: &usize) {}

	fn reparent_children(&mut self, _: &usize, _: &usize) {}

	fn mark_script_already_started(&mut self, _: &usize) {}

	fn set_current_line(&mut self, _: u64) {}

	fn pop(&mut self, _: &usize) {}
}

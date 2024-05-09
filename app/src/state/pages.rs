// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::collections::BTreeMap;
use std::io::Read;
use std::path;

use reqwest::StatusCode;
use sky_floem::cosmic_text::Style;
use sky_floem::peniko::Color;
use sky_floem::reactive::{create_rw_signal, RwSignal};
use sky_floem::views::{button, stack_from_iter, text, Decorators, Stack};
use sky_floem::{AnyView, IntoView};
use sky_html::{Attribute, HTMLDocument, HTMLElement};

// --------- //
// Structure //
// --------- //

pub struct PagesData
{
	pub current_page: RwSignal<Page>,
	// pub pages: Vec<Page>,
	// pages: (ReadSignal<String>, WriteSignal<String>),
}

pub struct PageView
{
	pub raw_content: String,
	pub new_title: String,
	pub dyn_content: Option<Stack>,
	pub debugging: bool,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Clone)]
pub enum Page
{
	File(path::PathBuf),
	Url(url::Url),
}

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum PageError
{
	#[error("Le fichier n'a pas pu être chargé, raison: « {0} ».")]
	Fs(#[from] std::io::Error),
	#[error("L'URL n'a pas pu être chargée, raison: {0}")]
	Req(#[from] reqwest::Error),
	#[error("L'URL n'a pas pu être chargée, status code: {status}")]
	InvalidReq
	{
		status: StatusCode
	},
	#[error("Impossible d'analyser l'HTML: {0}")]
	ParseHTML(#[from] sky_html::HTMLParserError),
	#[error("Impossible de convertir en UTF-8: {0}")]
	Utf8(#[from] std::str::Utf8Error),
}

// -------------- //
// Implémentation //
// -------------- //

impl PagesData
{
	pub fn new() -> Self
	{
		let blank_path = path::Path::new("./pages/blank.html");
		Self {
			current_page: create_rw_signal(Page::File(blank_path.to_owned())),
			// pages: Default::default(),
		}
	}
}

impl Page
{
	pub fn render(&self) -> Result<PageView, PageError>
	{
		match self {
			| Page::File(page_path) => self.open_file(page_path),
			| Page::Url(url) => self.fetch(url),
		}
	}

	pub fn open_file(
		&self,
		filepath: impl AsRef<path::Path>,
	) -> Result<PageView, PageError>
	{
		// TODO: autoriser plusieurs extensions.
		if filepath
			.as_ref()
			.extension()
			.filter(|ext| *ext == "html")
			.is_none()
		{
			return Err(PageError::Fs(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"n'est pas un fichier HTML",
			)));
		}

		let page_view = if cfg!(debug_assertions) {
			let mut file = std::fs::File::open(filepath)?;

			let mut buf = Vec::new();
			file.read_to_end(&mut buf)?;
			let raw_content = std::str::from_utf8(&buf)?;

			let doc = HTMLDocument::from_slice(&mut buf.as_slice())?;
			let mut page_view = self.build_page_view(&doc.elements)?;
			page_view.raw_content = raw_content.to_string();
			page_view.debugging = true;
			page_view
		} else {
			let doc = HTMLDocument::from_file(filepath)?;
			self.build_page_view(&doc.elements)?
		};

		Ok(page_view)
	}

	pub fn fetch(&self, url: impl ToString) -> Result<PageView, PageError>
	{
		reqwest::blocking::get(url.to_string())
			.map_err(PageError::Req)
			.and_then(|mut response| {
				let content_type =
					response.headers().get("content-type").unwrap();
				let content_type_str = content_type.to_str().unwrap();

				let status = response.status();

				if !status.is_success() {
					return Err(PageError::InvalidReq { status });
				}

				if content_type_str.contains("text/html") {
					let doc = HTMLDocument::from_stream(&mut response)?;
					let page_view = self.build_page_view(&doc.elements)?;
					return Ok(page_view);
				}

				if content_type_str.contains("text/plain") {
					let content = response.text()?;
					let text_el = text(&content).into_any();
					let page_view = PageView {
						dyn_content: Some(stack_from_iter([text_el])),
						new_title: Default::default(),
						raw_content: Default::default(),
						debugging: false,
					};
					return Ok(page_view);
				}

				Err(PageError::InvalidReq { status })
			})
	}
}

impl Page
{
	// TODO: a améliorer
	fn build_page_view(
		&self,
		tree: &BTreeMap<usize, HTMLElement>,
	) -> Result<PageView, PageError>
	{
		let mut list = vec![];

		let mut temp_page_view = PageView {
			raw_content: Default::default(),
			new_title: Default::default(),
			dyn_content: Default::default(),
			debugging: Default::default(),
		};

		let mut make_element =
			|el_name: &str, attrs: &[Attribute], maybe_text: Option<&str>| {
				match el_name {
					| "title" => {
						if let Some(t) = maybe_text {
							temp_page_view.new_title = t.trim().to_string();
						}
						None
					}

					| "button" => {
						Self::make_button_element(
							maybe_text.map(|s| s.to_string()),
						)
					}

					| "h1" => Self::make_heading(maybe_text, 34.0),
					| "h2" => Self::make_heading(maybe_text, 30.0),
					| "h3" => Self::make_heading(maybe_text, 26.0),
					| "h4" => Self::make_heading(maybe_text, 22.0),
					| "h5" => Self::make_heading(maybe_text, 20.0),
					| "h6" => Self::make_heading(maybe_text, 18.0),

					| "strong" | "b" => Self::make_bold_element(maybe_text),
					| "em" | "i" => Self::make_italic_element(maybe_text),

					| "a" => Self::make_anchor_element(maybe_text, attrs),

					| name => {
						if let Some(t) = maybe_text {
							let t = t.trim();
							if !t.is_empty() {
								return Some(text(t).into_any());
							}
						}

						let warning = format!("Élément « {name} » non rendu");

						println!("WARN: {} / {attrs:?}", &warning);

						Some(
							text(warning)
								.style(|style| {
									style
										.padding(4)
										.background(Color::DARK_RED)
										.color(Color::WHITE)
										.border(1)
										.border_radius(2.0)
										.border_color(Color::RED)
										.font_style(Style::Italic)
								})
								.into_any(),
						)
					}
				}
			};

		for (&element_id, element) in tree {
			let parent_id = element.parent;

			let element_name = element.name.local.to_string();

			if ["html", "body", "head", "meta", "link", "script", "style"]
				.contains(&element_name.as_str())
			{
				continue;
			}

			if element_id > parent_id && parent_id != 0 {
				// Traiter le parent?
				//let parent = tree.get(&parent_id).unwrap();

				let maybe_floem_element = make_element(
					&element_name,
					&element.attributes,
					element.text.as_ref().map(|t| t.as_ref()),
				);

				if let Some(floem_element) = maybe_floem_element {
					list.push(floem_element);
				}
			} else {
				dbg!(&element);
			}
		}

		temp_page_view
			.dyn_content
			.replace(stack_from_iter(list).style(|style| style.flex_col()));

		Ok(temp_page_view)
	}

	// FIXME: le clique fonctionne, mais ne change pas de page.
	// Mettre en place, un système de d'échange de message (Actor Pattern?).
	fn make_anchor_element(
		maybe_text: Option<&str>,
		attrs: &[Attribute],
	) -> Option<AnyView>
	{
		maybe_text.filter(|s| !s.trim().is_empty()).map(move |s| {
			let href = attrs.iter().find_map(|attr| {
				attr.name.local.eq("href").then_some(attr.value.to_string())
			});

			let mut element =
				text(s.trim()).style(|style| style.color(Color::STEEL_BLUE));

			if let Some(url) = href {
				element = element.on_click_cont(move |_| {
					println!("Clique sur le lien: {url}");
				});
			}

			element.into_any()
		})
	}

	fn make_bold_element(maybe_text: Option<&str>) -> Option<AnyView>
	{
		maybe_text.filter(|s| !s.trim().is_empty()).map(move |s| {
			text(s.trim()).style(|style| style.font_bold()).into_any()
		})
	}

	fn make_button_element(maybe_text: Option<String>) -> Option<AnyView>
	{
		maybe_text
			.filter(|s| !s.trim().is_empty())
			.map(move |s| button(move || s.trim().to_owned()).into_any())
	}

	fn make_italic_element(maybe_text: Option<&str>) -> Option<AnyView>
	{
		maybe_text.filter(|s| !s.trim().is_empty()).map(move |s| {
			text(s.trim())
				.style(|style| style.font_style(Style::Italic))
				.into_any()
		})
	}

	fn make_heading(maybe_text: Option<&str>, font_size: f32)
		-> Option<AnyView>
	{
		maybe_text.filter(|s| !s.trim().is_empty()).map(move |s| {
			text(s.trim())
				.style(move |style| style.font_size(font_size))
				.into_any()
		})
	}
}

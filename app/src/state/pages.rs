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

use floem::cosmic_text::Style;
use floem::peniko::Color;
use floem::reactive::{create_rw_signal, RwSignal};
use floem::view::View;
use floem::views::{stack_from_iter, text, Decorators, Stack};
use floem::widgets::button;
use reqwest::StatusCode;
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
	pub dyn_content: Stack,
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
				if response.status().is_success() {
					let page_view = if cfg!(debug_assertions) {
						let mut buf = Vec::new();

						// NOTE: le fait de lire la réponse d'une seule traite
						// fait que le système devient un peu plus lent sur des
						// gros jeux de données. Cela va de soi.
						response.read_to_end(&mut buf)?;

						let raw_content = std::str::from_utf8(&buf)?;

						let doc =
							HTMLDocument::from_slice(&mut buf.as_slice())?;

						let mut page_view =
							self.build_page_view(&doc.elements)?;

						page_view.raw_content = raw_content.to_string();
						page_view
					} else {
						let doc = HTMLDocument::from_stream(&mut response)?;
						self.build_page_view(&doc.elements)?
					};

					return Ok(page_view);
				}

				Err(PageError::InvalidReq {
					status: response.status(),
				})
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
			dyn_content: stack_from_iter([text(0)]),
		};

		let mut make_element =
			|el_name: &str, attrs: &[Attribute], maybe_text: Option<String>| {
				match el_name {
					| "title" => {
						let t = maybe_text.unwrap_or_default();
						temp_page_view.new_title = t;
						text("").any()
					}

					| "button" => {
						let t = maybe_text.unwrap_or_default();
						button(move || t.trim().to_owned()).any()
					}

					| "h1" => {
						let t = maybe_text.unwrap_or_default();
						text(t.trim())
							.style(|style| style.font_size(34.0))
							.any()
					}
					| "h2" => {
						let t = maybe_text.unwrap_or_default();
						text(t.trim())
							.style(|style| style.font_size(30.0))
							.any()
					}
					| "h3" => {
						let t = maybe_text.unwrap_or_default();
						text(t.trim())
							.style(|style| style.font_size(26.0))
							.any()
					}
					| "h4" => {
						let t = maybe_text.unwrap_or_default();
						text(t.trim())
							.style(|style| style.font_size(22.0))
							.any()
					}
					| "h5" => {
						let t = maybe_text.unwrap_or_default();
						text(t.trim())
							.style(|style| style.font_size(20.0))
							.any()
					}
					| "h6" => {
						let t = maybe_text.unwrap_or_default();
						text(t.trim())
							.style(|style| style.font_size(18.0))
							.any()
					}

					| "span" | "p" => {
						let t = maybe_text.unwrap_or_default();
						text(t.trim()).any()
					}

					| "strong" | "b" => {
						let t = maybe_text.unwrap_or_default();
						text(t.trim()).style(|style| style.font_bold()).any()
					}

					| "em" | "i" => {
						let t = maybe_text.unwrap_or_default();
						text(t.trim())
							.style(|style| {
								style.font_style(
									floem::cosmic_text::Style::Italic,
								)
							})
							.any()
					}

					| "a" => {
						let t = maybe_text.unwrap_or_default();
						let href = attrs.iter().find_map(|attr| {
							attr.name
								.local
								.eq("href")
								.then_some(attr.value.to_string())
						});

						text(t.trim())
							.style(|style| style.color(Color::STEEL_BLUE))
							.on_click_cont(move |_| {
								if let Some(url) = href.as_ref() {
									println!("Clique sur le lien: {url}");
								}
							})
							.any()
					}

					| name => {
						let warning = format!("Élément « {name} » non rendu");

						println!("WARN: {} / {attrs:?}", &warning);

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
							.any()
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

				let floem_element = make_element(
					&element_name,
					&element.attributes,
					element.text.as_ref().map(|t| t.to_string()),
				);

				list.push(floem_element);
			} else {
				dbg!(&element);
			}
		}

		temp_page_view.dyn_content =
			stack_from_iter(list).style(|style| style.flex_col());
		Ok(temp_page_view)
	}
}

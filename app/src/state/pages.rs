// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2024, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::path;

use floem::reactive::{create_rw_signal, RwSignal};
use reqwest::StatusCode;

// --------- //
// Structure //
// --------- //

pub struct PagesData
{
	pub current_page: RwSignal<Page>,
	pub pages: Vec<Page>,
	// pages: (ReadSignal<String>, WriteSignal<String>),
}

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
			pages: Default::default(),
		}
	}
}

impl Page
{
	// TODO: analyser le contenu
	pub fn fetch(&self) -> Result<String, PageError>
	{
		match self {
			| Page::File(page_path) => {
				// TODO: autoriser plusieurs extensions.
				if page_path.extension().filter(|ext| *ext == "html").is_none()
				{
					return Err(PageError::Fs(std::io::Error::new(
						std::io::ErrorKind::InvalidData,
						"n'est pas un fichier HTML",
					)));
				}

				Ok(std::fs::read_to_string(page_path)?)
			}
			| Page::Url(url) => {
				Ok(reqwest::blocking::get(url.as_ref())
					.map_err(PageError::Req)
					.and_then(|response| {
						if response.status().is_success() {
							return Ok(response.text()?);
						}

						Err(PageError::InvalidReq {
							status: response.status(),
						})
					})?)
			}
		}
	}
}

use std::collections::HashMap;

use regex::Regex;

use crate::types::{Transaction, TransactionsParsedFromMail};

use super::Mail;

pub mod ocbc;
pub mod rakuten_card;
pub mod rakuten_pay;

pub trait EmailParsingScheme {
	fn can_parse(&self, mail: &Mail) -> bool;
	fn parse(&self, mail: &Mail) -> Result<Vec<Transaction>, Box<dyn std::error::Error>>;
}

pub fn parse_emails(
	mails: Vec<Mail>,
	parsers: &Vec<Box<dyn EmailParsingScheme>>,
) -> Result<TransactionsParsedFromMail, Box<dyn std::error::Error>> {
	let mut map = HashMap::new();

	for mail in mails {
		for parser in parsers {
			if !parser.can_parse(&mail) {
				continue;
			}

			match parser.parse(&mail) {
				Ok(transactions) => {
					#[cfg(debug_assertions)]
					println!("{:#?}", transactions);

					map.insert(mail, transactions);
					break; // Break after first parse success
				}
				Err(e) => eprintln!("Could not parse mail: {}", e),
			}
		}
	}

	Ok(map)
}

fn parse_regex_first_match(
	text: &str,
	regex_literal: &str,
	capture_count: usize,
) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
	let mut captures_vec = vec![];

	let regex = Regex::new(regex_literal)?;
	while let Some(captures) = regex.captures_at(text, 0) {
		for i in 1..=capture_count {
			let capture = captures.get(i);
			if capture.is_none() {
				continue;
			}
			let capture = capture.unwrap();
			captures_vec.push(capture.as_str().to_owned());
		}
		break; // Break after first match
	}

	if captures_vec.len() == capture_count {
		return Ok(Some(captures_vec));
	}
	return Ok(None);
}

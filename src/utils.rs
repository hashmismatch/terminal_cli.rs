use core::prelude::*;

use core::num::Float;

use core::iter::repeat;

use collections::string::*;
use collections::Vec;
use collections::slice::SliceConcatExt;

/// A naive implementation. Can be implemented with a trie, but it's overkill here.
/// http://en.wikipedia.org/wiki/LCP_array
pub fn longest_common_prefix(strings: &[&str]) -> Option<String> {
	if strings.len() == 0 { return None; }
	if strings.len() == 1 { 
		let s = strings[0];
		if s.len() > 0 {
			return Some(s.to_string());
		} else {
			return None;
		}
	}

	let ref first = strings[0];

	for i in 0..first.len() {
		let c = first.chars().nth(i);
		for j in 1..strings.len() {
			let current_string = strings[j];
			if i >= current_string.len() || current_string.chars().nth(i) != c {
				// mismatch
				if i == 0 {
					return None;
				} else {
					return Some(first.chars().take(i).collect());
				}
			}
		}
	}

	None
}

/// Formats the strings in autocomplete-style column notation. Adds spaces in between.
/// Preserves the ordering. Last line will contain the newline sequence.
///
/// # Example
///
/// ```
/// #![feature(convert)]
/// # use terminal_cli::*;
/// let s = vec!["A1", "A2", "A3", "B1", "B2", "C1", "C2"];
/// let f = format_in_columns(s.as_slice(), 26, 10, "\r\n");
/// println!("{}", f);
/// ```
///
/// # Example output
///
/// ```text
/// A1          B1          C2
/// A2          B2
/// A3          C1
/// ```
pub fn format_in_columns(strings: &[&str], width: u16, min_spacing: u16, new_line: &str) -> String {
	if strings.len() == 0 { return "".to_string() };
	let max_len = strings.iter().max_by(|s| { s.len() }).unwrap().len() as u16;

	let columns = {
		let c = (width as f32 / (max_len + min_spacing) as f32).floor() as u16;
		let plus_one_width = ((max_len + min_spacing) * c) + max_len;
		if plus_one_width <= width {
			c + 1
		} else {
			c
		}
	};

	let rows = (strings.len() as f32 / columns as f32).ceil() as u16;
	
	let mut ret = String::new();
	
	for i in 0..rows {
		let mut line = String::new();

		for j in 0..columns {			
			let pos = (j as usize * rows as usize) + i as usize;
			if let Some(s) = strings.get(pos) {
				line.push_str(s);

				if j < columns-1 {
					let spaces = (max_len - s.len() as u16) + min_spacing;
					line.push_str(repeat(" ").take(spaces as usize).collect::<String>().as_slice());
				}
			};
		}

		ret.push_str(new_line);
		ret.push_str(line.as_slice());
	}

	ret.push_str(new_line);

	ret
}

#[cfg(test)]
mod tests {
	use super::*;
	use core::prelude::*;
	use std::prelude::*;
	use alloc::boxed::*;
	use collections::string::*;

	#[test]
	fn test_lcp() {
		{
			let strings = vec!["abcx", "ab1", "abb", "aaa"];
			let m = longest_common_prefix(strings.as_slice());
			assert_eq!(Some("a".to_string()), m)
		}

		{
			let strings = vec!["abcx", "ab1", "abb", "aaa", ""];
			let m = longest_common_prefix(strings.as_slice());
			assert_eq!(None, m)
		}

		{
			let strings = vec!["abcx", "ab1", "abb"];
			let m = longest_common_prefix(strings.as_slice());
			assert_eq!(Some("ab".to_string()), m)
		}

		{
			let strings = vec![];
			let m = longest_common_prefix(strings.as_slice());
			assert_eq!(None, m)	
		}

		{
			let strings = vec![""];
			let m = longest_common_prefix(strings.as_slice());
			assert_eq!(None, m)	
		}
	}

	#[test]
	fn test_column_format() {
		let s = vec!["A1", "A2", "A3", "B1", "B2", "C1", "C2"];
		let f = format_in_columns(s.as_slice(), 26, 10, "\r\n");
		println!("{}", f);
	}

}
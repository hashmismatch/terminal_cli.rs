use prelude::v1::*;

/// A naive implementation. Can be implemented with a trie, but it's overkill here.
/// http://en.wikipedia.org/wiki/LCP_array
pub fn longest_common_prefix<'a>(strings: &'a [&'a str]) -> Option<&'a str> {
	if strings.len() == 0 { return None; }
	if strings.len() == 1 { 
		let s = strings[0];
		if s.len() > 0 {
			return Some(s);
		} else {
			return None;
		}
	}

	let ref first = strings[0].as_bytes();

	for i in 0..first.len() {
		let c = first[i];
		for other in 1..strings.len() {
			let other = strings[other].as_bytes();
			if i >= other.len() || (c != other[i]) {
				if i == 0 {					
					return None;
				} else {
					let b = &first[0..i];
					return match str::from_utf8(b) {
						Ok(s) => Some(s),
						Err(_) => None
					};
				}
			}
		}
	}

	if strings[0].len() > 0 {
		Some(strings[0])
	} else {
		None
	}
}

/// Formats the strings in autocomplete-style column notation. Fills the width of
/// the entire line with a string plus the desired spacing characters. Preserves 
/// the ordering in columns.
///
/// # Examples
///
/// ```
/// # use terminal_cli::*;
/// let s = vec!["A1", "A2", "A3", "B1", "B2", "C1", "C2"];
/// let mut out = String::new();
/// format_in_columns(s.as_slice(), 26, 10, "\r\n", &mut out).unwrap();
/// println!("{}", out);
/// # assert_eq!("A1          B1          C2\r\nA2          B2          \r\nA3          C1          \r\n", out);
/// ```
/// ```text
/// A1          B1          C2
/// A2          B2
/// A3          C1
/// ```
pub fn format_in_columns<W: FmtWrite>(strings: &[&str], width: u16, spacing: u16, new_line: &str, write: &mut W) -> Result<(), FmtError> {
	if strings.len() == 0 { return Ok(()); }

	let max_len = strings.iter().max_by_key(|s| { s.len() }).unwrap().len() as u16;

	let columns = {
		let c = ((width as f32 / (max_len + spacing) as f32)).floor() as u16;
		let plus_one_width = ((max_len + spacing) * c) + max_len;
		if plus_one_width <= width {
			c + 1
		} else {
			c
		}
	};

	let rows = (strings.len() as f32 / columns as f32).ceil() as u16;

	for i in 0..rows {
		for j in 0..columns {			
			let pos = (j as usize * rows as usize) + i as usize;
			if let Some(s) = strings.get(pos) {
				try!(write.write_str(&s));

				if j < columns-1 {
					let spaces = (max_len - s.len() as u16) + spacing;
					for i in 0..spaces {
						try!(write.write_str(" "));
					}
				}
			}
		}
		write.write_str(new_line)?;
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use prelude::v1::*;

	#[test]
	fn test_lcp() {
		{
			let strings = vec!["p1", "p1/hello"];
			let m = longest_common_prefix(&strings);
			assert_eq!(Some("p1"), m)
		}

		{
			let strings = vec!["abcx", "ab1", "abb", "aaa"];
			let m = longest_common_prefix(&strings);
			assert_eq!(Some("a"), m)
		}

		{
			let strings = vec!["abcx", "ab1", "abb", "aaa", ""];
			let m = longest_common_prefix(&strings);
			assert_eq!(None, m)
		}

		{
			let strings = vec!["abcx", "ab1", "abb"];
			let m = longest_common_prefix(&strings);
			assert_eq!(Some("ab"), m)
		}

		{
			let strings = vec![];
			let m = longest_common_prefix(&strings);
			assert_eq!(None, m)	
		}

		{
			let strings = vec![""];
			let m = longest_common_prefix(&strings);
			assert_eq!(None, m)	
		}		

		{
			let strings = vec!["", "a"];
			let m = longest_common_prefix(&strings);
			assert_eq!(None, m)	
		}
	}

	#[test]
	fn test_column_format() {
		let s = vec!["A1", "A2", "A3", "B1", "B2", "C1", "C2"];
		let mut out = String::new();
		format_in_columns(&s, 26, 10, "\r\n", &mut out).unwrap();
		assert_eq!("A1          B1          C2\r\nA2          B2          \r\nA3          C1          \r\n", out);
	}

}
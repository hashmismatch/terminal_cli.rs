use core::prelude::*;

use alloc::rc::Rc;
use alloc::boxed::Box;

use collections::string::*;
use collections::Vec;
use collections::slice::SliceConcatExt;

use cli::*;
use utils::*;

/// Simple keyword command, like ```help``` with no arguments.
pub struct CliCommandKeyword<Fo> where Fo: Fn(&str, &mut CliTerminal) -> ()  {
	/// The keyword.
	pub keyword: String,
	/// Action to be executed when the input matches.
	pub action: Fo
}


impl<Fo> CliCommand for CliCommandKeyword<Fo> where Fo: Fn(&str, &mut CliTerminal) -> () {
	fn execute(&mut self, cli: &mut CliTerminal, line: &str) {
		self.action.call((line, cli));
	}

	fn is_match(&self, line: &str) -> bool {
		self.keyword == line
	}

	fn autocomplete(&self, line_start: &str) -> Option<Vec<AutocompleteOption>> {
		if self.keyword.starts_with(line_start) {
			Some(vec! [
				AutocompleteOption::FullCommand { line: self.keyword.clone() },
				AutocompleteOption::Hint { hint: self.keyword.clone() },
			])
		} else {
			None
		}
	}
}

/// Owned property that can be changed with ```set var_name <value>``` and retrieved with 
/// ```get var_name```.
pub struct CliPropertyVar<T, Fo, Fi> where Fo: Fn(&T) -> String, Fi: Fn(&str) -> Option<T>
{
	/// Name of the property
	pub var_name: String,
	/// Initial value of the property
	pub var_value: T,

	/// Output formatter
	pub var_output: Fo,
	/// Input parser
	pub var_input: Fi,
	/// Hint for the setter explanation.
	pub val_hint: String
}

/// Retrieved property that can be changed with ```set var_name <value>``` and retrieved with 
/// ```get var_name```. Useful for values that are changed by other parts of the system, like RTC
/// clock or some other counter.
pub struct CliPropertyFn<Fo, Fi> where Fo: Fn() -> String, Fi: Fn(&str, &mut CliTerminal) -> ()
{
	/// Name of the property
	pub var_name: String,

	/// Output the current value of the property
	pub var_output: Fo,
	/// Try to parse and set the property
	pub var_input: Fi,
	/// Hint for the setter explanation
	pub val_hint: String
}

trait CliProperty {
	fn get_var_name(&self) -> &str;
	fn get_val_hint(&self) -> &str;

	fn set_prefix(&self) -> String {
		format!("set {}", self.get_var_name())
	}
	fn get_prefix(&self) -> String {
		format!("get {}", self.get_var_name())
	}

	fn _autocomplete(&self, line_start: &str) -> Option<Vec<AutocompleteOption>> {
		let mut ret = Vec::new();

		let l_get = self.get_prefix();
		let l_set = self.set_prefix();

		if l_get.starts_with(line_start) {
			ret.push(AutocompleteOption::FullCommand { line: l_get.clone() });
			ret.push(AutocompleteOption::Hint { hint: l_get.clone() });
		}

		if l_set.starts_with(line_start) {
			ret.push(AutocompleteOption::FullCommand { line: format!("{} ", l_set) });
			ret.push(AutocompleteOption::Hint { hint: format!("{} <{}>", l_set, self.get_val_hint()) });
		}

		if ret.len() == 0 { return None; }
		Some(ret)
	}	

	fn _is_match(&self, line: &str) -> bool {
		if line.starts_with(self.set_prefix().as_slice()) ||
	       line.starts_with(self.get_prefix().as_slice()) {
			true
		} else {
			false
		}
	}	
}

impl<T, Fo, Fi> CliProperty for CliPropertyVar<T, Fo, Fi> where Fo: Fn(&T) -> String, Fi: Fn(&str) -> Option<T> {
	fn get_var_name(&self) -> &str {
		self.var_name.as_slice()
	}
	fn get_val_hint(&self) -> &str {
		self.val_hint.as_slice()
	}
}

impl<Fo, Fi> CliProperty for CliPropertyFn<Fo, Fi> where Fo: Fn() -> String, Fi: Fn(&str, &mut CliTerminal) -> () {
	fn get_var_name(&self) -> &str {
		self.var_name.as_slice()
	}
	fn get_val_hint(&self) -> &str {
		self.val_hint.as_slice()
	}
}


impl<T, Fo, Fi> CliCommand for CliPropertyVar<T, Fo, Fi>
	where Fo: Fn(&T) -> String, Fi: Fn(&str) -> Option<T>
{
	fn execute(&mut self, cli: &mut CliTerminal, line: &str) {
		if line.starts_with(self.get_prefix().as_slice()) {
			let d = self.var_output.call((&self.var_value,));
			cli.output_line(format!("{} = {}", self.var_name, d).as_slice());
		}

		let set_pref = self.set_prefix();
		let set_pref = set_pref.as_slice();
		
		if line.starts_with(&set_pref) {

			let v = {
				if line.len() > set_pref.len() {
					let l = &line[(set_pref.len() + 1)..];
					self.var_input.call((l,))
				} else {
					None
				}
			};

			match v {
				Some(v) => { self.var_value = v; }
				None => { cli.output_line("Couldn't parse the value."); }
			}

		}
	}

	fn is_match(&self, line: &str) -> bool {
		self._is_match(line)
	}

	fn autocomplete(&self, line_start: &str) -> Option<Vec<AutocompleteOption>>  {
		self._autocomplete(line_start)
	}
}


impl<Fo, Fi> CliCommand for CliPropertyFn<Fo, Fi> where Fo: Fn() -> String, Fi: Fn(&str, &mut CliTerminal) {

	fn execute(&mut self, cli: &mut CliTerminal, line: &str) {
		if line.starts_with(self.get_prefix().as_slice()) {
			let d = self.var_output.call(());
			cli.output_line(format!("{} = {}", self.var_name, d).as_slice());
		}

		let set_pref = self.set_prefix();
		let set_pref = set_pref.as_slice();
		if line.starts_with(&set_pref) {
			if line.len() > set_pref.len() {
				let l = &line[(set_pref.len() + 1)..];
				self.var_input.call((l, cli));
			}
		}
	}


	fn is_match(&self, line: &str) -> bool {
		self._is_match(line)
	}

	fn autocomplete(&self, line_start: &str) -> Option<Vec<AutocompleteOption>>  {
		self._autocomplete(line_start)
	}
}

use super::*;
use core::prelude::*;
use std::prelude::*;
use alloc::boxed::*;
use collections::string::*;
//use super::commands::*;


struct StdoutTerminal;
impl CliTerminal for StdoutTerminal {
	fn output_line(&mut self, line: &str) {
		println!("{}", line);
	}
}

#[test]
pub fn test_suggest() {
	let show = CliCommandKeyword {
		keyword: "show".to_string(),
		action: |line, cli| {
		}
	};
	let cs = CliPropertyVar {
		var_name: "display".to_string(),
		var_value: "clock".to_string(),
		val_hint: "clock, blank".to_string(),
		var_output: |v| { v.to_string() },
		var_input: |v| {
			if v.len() > 0 { Some(v.to_string()) }
			else { None }
		}
	};
	let ct = CliPropertyVar {
		var_name: "time".to_string(),
		var_value: "11:15".to_string(),
		val_hint: "HH:mm".to_string(),
		var_output: |v| { v.to_string() },
		var_input: |v| {
			if v.len() > 0 { Some(v.to_string()) }
			else { None }
		}
	};
	let ctt = CliPropertyVar {
		var_name: "time_date".to_string(),
		var_value: "11:15 1.1.2015".to_string(),
		val_hint: "HH:mm DD.MM.YYYY".to_string(),
		var_output: |v| { v.to_string() },
		var_input: |v| {
			if v.len() > 0 { Some(v.to_string()) }
			else { None }
		}
	};
	let mut term = StdoutTerminal;
	let mut commands = vec![
		Box::new(show) as Box<CliCommand>,
		Box::new(cs) as Box<CliCommand>,
		Box::new(ct) as Box<CliCommand>,
		Box::new(ctt) as Box<CliCommand>
	];
	cli_execute("set ", commands.as_mut_slice(), &mut term);
	let autocomplete = cli_try_autocomplete("", commands.as_mut_slice());
	println!("{:?}", autocomplete);
}

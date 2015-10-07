extern crate terminal_cli;
use terminal_cli::*;

#[cfg(not(feature = "termios_support"))]
fn main() {
	println!("Termios support isn't enabled. Recompile with the feature 'termios_support'.");
}

#[cfg(feature = "termios_support")]
fn main() {
	use terminal_cli::terminal_termios::*;
	
	let mut term = TerminalTermios::new();
	let mut prompt = PromptBuffer::new(true);
	prompt.print_prompt(&mut term);

	let mut prop1 = new_property_min_max("counter".into(), 1, 1, 100);
	let mut prop2 = new_property("switch".into(), false);

	loop {
		match term.read() {
			Ok(key) => {
				
				match prompt.handle_key(key, &mut term, |mut matcher, t| {
					matcher.process_property(&mut prop1, t);
					matcher.process_property(&mut prop2, t);
					matcher.finish()
				})

				{
					Some(PromptEvent::Break) => {
						break;
					},
					_ => ()
				}
			},
			Err(_) => {
				break;
			}
		}
	}

	println!("");
}
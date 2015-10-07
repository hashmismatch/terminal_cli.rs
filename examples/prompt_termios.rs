extern crate terminal_cli;
use terminal_cli::*;

#[cfg(not(feature = "termios_support"))]
fn main() {
	println!("Termios support isn't enabled. Recompile with the feature 'termios_support'.");
}

#[cfg(feature = "termios_support")]
fn main() {
	use terminal_cli::terminal_termios::*;

	let options = PromptBufferOptions { echo: true, ..Default::default() };
	
	let mut term = TerminalTermios::new();	
	let mut prompt = PromptBuffer::new(options);
	prompt.print_prompt(&mut term);

	let mut counter = 1;
	let mut switch = false;
	
	loop {
		match term.read() {
			Ok(key) => {
				
				match prompt.handle_key(key, &mut term, |mut m| {
					if let Some(mut ctx) = m.run_property("counter", validate_property_min_max(1, 100)) {
						ctx.apply(&mut counter);
					}
					if let Some(mut ctx) = m.run_property("switch", ValueBool) {
						ctx.apply(&mut switch);
					}
					if let Some(mut ctx) = m.run_command("p1/hello") {
						ctx.get_terminal().print_line("Hello world - P1");
					}
					if let Some(mut ctx) = m.run_command("p1/s1/hello") {
						ctx.get_terminal().print_line("Hello world - P1 S1");
					}
					if let Some(mut ctx) = m.run_command("p1/s1/reset") {
						ctx.get_terminal().print_line("Reset.");
					}
					if let Some(mut ctx) = m.run_command("p2/hello") {
						ctx.get_terminal().print_line("Hello world - P2");
					}
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

	println!("Exit.");
}
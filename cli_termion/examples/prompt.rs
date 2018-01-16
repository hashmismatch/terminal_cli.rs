extern crate terminal_cli;
extern crate terminal_cli_termion;
use terminal_cli::*;
use terminal_cli_termion::*;

fn main() {
	let options = PromptBufferOptions { echo: true, ..Default::default() };
	
	let mut term = TerminalTermion::new();	
	let mut prompt = PromptBuffer::new(options);
	prompt.print_prompt(&mut term);

	let mut counter = 1;
	let mut switch = false;
	
	loop {

		let key_result = prompt.handle_terminal_key(&mut term, |m| {
			if let Some(mut m) = m.with_prefix("p3/") {
				if let Some(mut ctx) = m.command("hello") {
					ctx.get_terminal().print_line("Hello world - P3");
				}

				if let Some(mut ctx) = m.command("ping") {
					ctx.get_terminal().print_line("Pong");
				}

				if let Some(mut m) = m.with_prefix("more/") {
					if let Some(mut ctx) = m.command("here") {
						ctx.get_terminal().print_line("Yep, here");
					}
				}
			}

			if let Some(mut ctx) = m.property("counter", validate_property_min_max(1, 100)) {
				ctx.apply(&mut counter);
			}
			if let Some(mut ctx) = m.property("switch", ValueBool) {
				ctx.apply(&mut switch);
			}
			if let Some(mut ctx) = m.command("p1/hello") {
				ctx.get_terminal().print_line("Hello world - P1");
			}
			if let Some(mut ctx) = m.command("p1/s1/hello") {
				ctx.get_terminal().print_line("Hello world - P1 S1");
			}
			if let Some(mut ctx) = m.command("p1/s1/reset") {
				ctx.get_terminal().print_line("Reset.");
			}
			if let Some(mut ctx) = m.command("p2/hello") {
				ctx.get_terminal().print_line("Hello world - P2");
			}

			if let Some(mut ctx) = m.command("lines") {
				ctx.get_terminal().print_line("Line 1");
				ctx.get_terminal().print_line("Line 2");
				println!("Line 3");
				println!("Line 4");
			}
		});

		match key_result {
			Ok(PromptEvent::Break) => {
				break;
			},
			_ => ()
		}
	}

	println!("Exit.");
}
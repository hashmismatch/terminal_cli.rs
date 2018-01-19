# terminal_cli

## Terminal CLI

Need to build an interactive command prompt, with commands, properties and with full autocomplete? This is for you.

[![Build Status](https://travis-ci.org/hashmismatch/terminal_cli.rs.svg?branch=master)](https://travis-ci.org/hashmismatch/terminal_cli.rs)

[![Documentation](https://docs.rs/terminal_cli/badge.svg)](https://docs.rs/terminal_cli)


## Example, output only (Rust's ```stdout```)

```rust

// Simple ranged integer property
let mut num1 = 1;

// Rust stdout terminal
let mut terminal = StdoutTerminal;

let options = PromptBufferOptions { echo: true, ..Default::default() };
let mut prompt = PromptBuffer::new(options);

let input_keys = [Key::Character('h' as u8), Key::Character('e' as u8), Key::Character('l' as u8),
                  Key::Character('p' as u8), Key::Newline];

for key in &input_keys {
    let p = prompt.handle_key(*key, &mut terminal, |mut m| {
        if let Some(mut ctx) = m.command("help") {
            ctx.get_terminal().print_line("Help!");
        }

        // Provides "num1/get" and "num1/set", with input validation
        if let Some(mut ctx) = m.property("num1", validate_property_min_max(1, 100)) {
            ctx.apply(&mut num1);
        }
    });

    if let PromptEvent::Break = p {
        break;
    }
}
```

License: MIT/Apache-2.0

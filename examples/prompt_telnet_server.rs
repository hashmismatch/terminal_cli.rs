extern crate terminal_cli;
use terminal_cli::*;


use std::net::{TcpListener, TcpStream, Shutdown, SocketAddr};
use std::io::{Read, Write};
use std::str::FromStr;
use std::thread;
use std::fmt::Write as FmtWrite;
use std::fmt::Error as FmtError;
use std::env;
use std::sync::{Arc, Mutex};

struct TerminalBuffer {
    buffer: Vec<u8>
}
impl TerminalBuffer {
	fn new() -> Self {
		TerminalBuffer { buffer: vec![] }
	}
    fn close(self) -> Vec<u8> {
        self.buffer
    }
}

impl CharacterTerminalWriter for TerminalBuffer {
    fn print(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }
}
impl FmtWrite for TerminalBuffer {
    fn write_str(&mut self, s: &str) -> Result<(), FmtError> {
        self.print(s.as_bytes());
        Ok(())
    }
}


struct SharedData {
	val: u32
}

fn main() {
	let addr = env::args().nth(1).unwrap_or("127.0.0.1:3000".to_string());
	let listener = TcpListener::bind(SocketAddr::from_str(&addr).unwrap()).unwrap();

	println!("Telnet server listening at {}", &addr);

	let shared_data = Arc::new(Mutex::new(SharedData { val: 1 }));
	
	for stream in listener.incoming() {
		println!("Accepted connection");

		let shared_data = shared_data.clone();

	    match stream {
	        Ok(mut stream) => {
	            thread::spawn(move|| {
	                let options = PromptBufferOptions { echo: true, ..Default::default() };
	                let mut buffer = PromptBuffer::new(options);
                	let mut term = TelnetTerminal::new();
                	let mut term_keys = TerminalKeyDecoder::new();

					let mut local_prop = 1;
                	
                	// Send the telnet hello
                	let hello = term.hello();
            		stream.write_all(&TelnetStream::all_to_network(&hello)).unwrap();

            		{
            			let mut terminal_buffer = TerminalBuffer::new();
            			terminal_buffer.print_line("Rust terminal_cli telnet server demo.");
            			terminal_buffer.print_line("");
            			buffer.print_prompt(&mut terminal_buffer);
            			stream.write_all(&terminal_buffer.close());
            		}

            		loop {
            			let mut buf = [0; 64];
            			match stream.read(&mut buf) {
            				Ok(s) => {
            					if s == 0 {
            						break;
            					}

            					let buf = &buf[..s];

            					match term.handle_input(&buf) {
                        			Ok(TelnetEvent::ReceivedData(d)) => {

                        				let mut output = Vec::new();

			                            for c in d {
			                                match term_keys.decode(c) {
			                                    Ok(key) => {

			                                    	let mut terminal_buffer = TerminalBuffer::new();
			                                    	let prompt_result = buffer.handle_key(key, &mut terminal_buffer, |mut m| {
														if let Some(mut ctx) = m.run_property("local_prop", validate_property_min_max(1, 100)) {
															ctx.apply(&mut local_prop);
														}

														if let Some(mut ctx) = m.run_command("inc") {
															if let Ok(mut shared_data) = shared_data.lock() {
																shared_data.val += 1;
																ctx.get_terminal().print_line(&format!("New value: {}", shared_data.val));
															}
														}
			                                        }); 

			                                        stream.write_all(&terminal_buffer.close()).unwrap();

			                                        match prompt_result {
			                                            Some(PromptEvent::Break) => {
			                                                println!("client is ending the connection.");
			                                                stream.shutdown(Shutdown::Both).unwrap();
			                                            },
			                                            _ => ()
			                                        }

			                                    },
			                                    Err(KeyDecoderError::MoreInputRequired) | Err(KeyDecoderError::UnknownSequence) => {
			                                        continue;
			                                    }
			                                }
			                            }
			                            
			                            if output.len() > 0 {
			                                stream.write_all(&output).unwrap();
			                            }
                        			},
                        			Ok(TelnetEvent::ServerResponse(c)) => {
                            			let d = TelnetStream::all_to_network(&c);
                            			if d.len() > 0 {
                            				stream.write_all(&d).unwrap();
                            			}
                        			},
                        			Ok(TelnetEvent::None) => {
                            			// wait for more input
                        			},
                        			Err(e) => {
                            			panic!("prompt handler exception: {:?}", e);
                        			}
                        		}
            				},
            				Err(e) => {
            					panic!("Error reading from TCP: {:?}", e);
            				}
            			}
            		}
	            });
	        }
	        Err(e) => { println!("Error accepting TCP connection: {:?}", e) }
	    }
	}

}
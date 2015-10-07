extern crate terminal_cli;
use terminal_cli::*;

#[cfg(not(feature = "rotor_telnet_support"))]
fn main() {
    println!("Rotor support isn't enabled. Recompile with the feature 'rotor_telnet_support'.");
}

#[cfg(feature = "rotor_telnet_support")]
mod telnet {
    extern crate rotor;
    extern crate rotor_stream;
    extern crate bytes;

    use std::io::{Write, stderr};
    use std::error::Error;
    use std::sync::{Arc, Mutex};
    use std::ops::DerefMut;

    use terminal_cli::*;

    use self::rotor::{EventSet, PollOpt, Loop, Config, Void};
    use self::rotor::mio::{TryRead, TryWrite};
    use self::rotor::mio::tcp::{TcpListener, TcpStream};
    use self::rotor::{Machine, Response, EarlyScope, Scope};    
    use self::rotor_stream::{Accept, Stream, Protocol, Intent, Transport, Exception};
    use self::bytes::{SliceBuf, Buf};


    /* ------- */

    struct ConnectionPrompt {
        buffer: PromptBuffer,
        term: TelnetTerminal,
        term_keys: TerminalKeyDecoder,

        local_prop: Property<'static, u32, 
        ValueInputWithValidation<u32, ValueInputFromStr, ValueCombineValidators<u32, ValueMin<u32>, ValueMax<u32>>>,
        ValueOutputToString>
    }
    
    struct MioTelnetTerminal {
        buffer: Vec<u8>
    }
    impl MioTelnetTerminal {
        pub fn close(self) -> Vec<u8> {
            self.buffer
        }
    }
    impl LineTerminalWriter for MioTelnetTerminal {
        fn print_line(&mut self, line: &str) {
            self.print_str_line(line);
        }
    }

    impl CharacterTerminalWriter for MioTelnetTerminal {
        fn print(&mut self, bytes: &[u8]) {
            self.buffer.extend_from_slice(bytes);
        }
    }

    impl ConnectionPrompt {
        pub fn new() -> ConnectionPrompt {
            ConnectionPrompt {
                buffer: PromptBuffer::new(true),
                term: TelnetTerminal::new(),
                term_keys: TerminalKeyDecoder::new(),
                local_prop: new_property_min_max("local_prop".into(), 1, 1, 100)
            }
        }
    }

    /* ------- */


    /// server context with a global, mutex protected property that can be manipulated from every connection
    struct Context {
        global_prop: Arc<Mutex<Property<'static, u32, 
            ValueInputWithValidation<u32, ValueInputFromStr, ValueCombineValidators<u32, ValueMin<u32>, ValueMax<u32>>>,
            ValueOutputToString>>>
    }

    enum TelnetServer {
        Read(TelnetConnection),
        Write(TelnetConnection, Vec<u8>)
    }

    struct TelnetConnection(ConnectionPrompt);

    impl Protocol for TelnetServer {
        type Context = Context;
        type Socket = TcpStream;
        type Seed = ();

        fn create(_seed: (), _sock: &mut TcpStream, scope: &mut Scope<Context>)
            -> Intent<Self>
        {
            // send the telnet hello
            let mut prompt = ConnectionPrompt::new();
            let h = prompt.term.hello();
            let d = TelnetStream::all_to_network(&h);

            let connection = TelnetConnection(prompt);
            
            Intent::of(TelnetServer::Write(connection, d)).expect_flush()
        }


        fn bytes_read(self, transport: &mut Transport<TcpStream>,
                      _end: usize, scope: &mut Scope<Context>)
            -> Intent<Self>
        {
            match self {                
                TelnetServer::Read(mut conn) => {
                    
                    let buf = Vec::from(&transport.input()[..]);
                    transport.input().consume(buf.len());
                    
                    match conn.0.term.handle_input(&buf) {
                        Ok(TelnetEvent::ReceivedData(d)) => {
                            let mut output = Vec::new();

                            for c in d {
                                match conn.0.term_keys.decode(c) {
                                    Ok(key) => {
                                        let mut term = MioTelnetTerminal { buffer: Vec::new() };
                                        let (buffer, local_prop) = (&mut conn.0.buffer, &mut conn.0.local_prop);

                                        let prompt_result = buffer.handle_key(key, &mut term, |mut matcher, t| {
                                            matcher.process_property(local_prop, t);
                                            if let Ok(mut p) = scope.global_prop.lock() {
                                                let mut p = p.deref_mut();
                                                matcher.process_property(p, t);
                                            }
                                            matcher.finish()
                                        }); 

                                        output.extend_from_slice(&term.close());

                                        match prompt_result {
                                            Some(PromptEvent::Break) => {
                                                // todo: also send the last response
                                                println!("client is ending the connection.");
                                                return Intent::done();
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
                                return Intent::of(TelnetServer::Write(conn, output)).expect_flush();
                            }
                        },
                        Ok(TelnetEvent::ServerResponse(c)) => {
                            let d = TelnetStream::all_to_network(&c);
                            if d.len() > 0 {
                                return Intent::of(TelnetServer::Write(conn, d)).expect_flush();
                            }
                        },
                        Ok(TelnetEvent::None) => {
                            // wait for more input
                        },
                        Err(e) => {
                            println!("prompt handler exception: {:?}", e);
                            return Intent::done();
                        }
                    }
                
                    Intent::of(TelnetServer::Read(conn)).expect_bytes(1)
                },
                _ => unreachable!(),
            }
        }

        fn bytes_flushed(self, transport: &mut Transport<TcpStream>,
                         _scope: &mut Scope<Context>)
            -> Intent<Self>
        {
            match self {
                TelnetServer::Write(conn, val) => {
                    transport.output().extend(&val);
                    Intent::of(TelnetServer::Read(conn)).expect_bytes(1)
                },
                _ => unreachable!(),
            }
        }

        fn timeout(self, _transport: &mut Transport<TcpStream>,
            _scope: &mut Scope<Context>)
            -> Intent<Self>
        {
            writeln!(&mut stderr(), "Timeout happened").ok();
            Intent::done()
        }

        fn wakeup(self, _transport: &mut Transport<TcpStream>,
            _scope: &mut Scope<Context>)
            -> Intent<Self>
        {
            unreachable!();
        }
        fn exception(self, _transport: &mut Transport<Self::Socket>,
            reason: Exception, _scope: &mut Scope<Self::Context>)
            -> Intent<Self>
        {
            writeln!(&mut stderr(), "Error: {}", reason).ok();
            Intent::done()
        }
        fn fatal(self, reason: Exception, _scope: &mut Scope<Self::Context>)
            -> Option<Box<Error>>
        {
            writeln!(&mut stderr(), "Error: {}", reason).ok();
            None
        }        
    }

    pub fn main() {
        let bind_to = "127.0.0.1:3000";

        let mut event_loop = rotor::Loop::new(&rotor::Config::new()).unwrap();
        let lst = TcpListener::bind(&bind_to.parse().unwrap()).unwrap();
        let ok = event_loop.add_machine_with(|scope| {
            Accept::<Stream<TelnetServer>, _>::new(lst, (), scope)
        }).is_ok();        
        assert!(ok);        
        let context = Context {
            global_prop: Arc::new(Mutex::new(new_property_min_max("global_prop".into(), 1, 1, 100)))
        };

        println!("Telnet server listening on {}", bind_to);

        event_loop.run(context).unwrap();
    }

}

#[cfg(feature = "rotor_telnet_support")]
fn main() {
    self::telnet::main()
}
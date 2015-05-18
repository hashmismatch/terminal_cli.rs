var searchIndex = {};
searchIndex['terminal_cli'] = {"items":[[0,"","terminal_cli","A helper library for implementing low-level terminal command line interfaces,\n like those on embedded bare-bones environments with UART ports.",null,null],[3,"AutocompleteLine","","One autocomplete suggestion",null,null],[12,"full_new_line","","The entire new suggested line buffer",0,null],[12,"additional_part","","The additional suggested part of the buffer, can be sent to the terminal device",0,null],[3,"CliCommandKeyword","","Simple keyword command, like ```help``` with no arguments.",null,null],[12,"keyword","","The keyword.",1,null],[12,"action","","Action to be executed when the input matches.",1,null],[3,"CliPropertyVar","","Owned property that can be changed with ```set var_name <value>``` and retrieved with\n```get var_name```.",null,null],[12,"var_name","","Name of the property",2,null],[12,"var_value","","Initial value of the property",2,null],[12,"var_output","","Output formatter",2,null],[12,"var_input","","Input parser",2,null],[12,"val_hint","","Hint for the setter explanation.",2,null],[3,"CliPropertyFn","","Retrieved property that can be changed with ```set var_name <value>``` and retrieved with\n```get var_name```. Useful for values that are changed by other parts of the system, like RTC\nclock or some other counter.",null,null],[12,"var_name","","Name of the property",3,null],[12,"var_output","","Output the current value of the property",3,null],[12,"var_input","","Try to parse and set the property",3,null],[12,"val_hint","","Hint for the setter explanation",3,null],[3,"CliPromptAutocompleteBuffer","","Holds the current line buffer for a terminal and its possible autocomplete state.",null,null],[4,"AutocompleteOption","","A command's hints to the autocompletition",null,null],[13,"Hint","","Hint for the missing argument for the end user",4,null],[12,"hint","terminal_cli::AutocompleteOption","",4,null],[13,"FullCommand","terminal_cli","The full line buffer of the suggested command",4,null],[12,"line","terminal_cli::AutocompleteOption","",4,null],[4,"AutocompleteResult","terminal_cli","Result of the autocomplete request on a given set of commands",null,null],[13,"None","","No suggestions available",5,null],[13,"SingleMatch","","A single match has been found, the line buffer can be immediately expanded with the new command",5,null],[12,"line","terminal_cli::AutocompleteResult","",5,null],[13,"MultipleMatches","terminal_cli","Multiple matches, usually they can be presented to the end user in a column format.",5,null],[12,"lines","terminal_cli::AutocompleteResult","",5,null],[5,"cli_execute","terminal_cli","Execute the given line buffer with the set of commands.",null,null],[5,"cli_try_autocomplete","","Collect autocomplete suggestions for this line buffer",null,null],[5,"longest_common_prefix","","A naive implementation. Can be implemented with a trie, but it's overkill here.\nhttp://en.wikipedia.org/wiki/LCP_array",null,null],[5,"format_in_columns","","Formats the strings in autocomplete-style column notation. Adds spaces in between.\nPreserves the ordering. Last line will contain the newline sequence.",null,null],[11,"clone","","",4,{"inputs":[{"name":"autocompleteoption"}],"output":{"name":"autocompleteoption"}}],[11,"clone","","",5,{"inputs":[{"name":"autocompleteresult"}],"output":{"name":"autocompleteresult"}}],[11,"fmt","","",5,{"inputs":[{"name":"autocompleteresult"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",0,{"inputs":[{"name":"autocompleteline"}],"output":{"name":"autocompleteline"}}],[11,"fmt","","",0,{"inputs":[{"name":"autocompleteline"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"execute","","",1,{"inputs":[{"name":"clicommandkeyword"},{"name":"cliterminal"},{"name":"str"}],"output":null}],[11,"is_match","","",1,{"inputs":[{"name":"clicommandkeyword"},{"name":"str"}],"output":{"name":"bool"}}],[11,"autocomplete","","",1,{"inputs":[{"name":"clicommandkeyword"},{"name":"str"}],"output":{"name":"option"}}],[11,"execute","","",2,{"inputs":[{"name":"clipropertyvar"},{"name":"cliterminal"},{"name":"str"}],"output":null}],[11,"is_match","","",2,{"inputs":[{"name":"clipropertyvar"},{"name":"str"}],"output":{"name":"bool"}}],[11,"autocomplete","","",2,{"inputs":[{"name":"clipropertyvar"},{"name":"str"}],"output":{"name":"option"}}],[11,"execute","","",3,{"inputs":[{"name":"clipropertyfn"},{"name":"cliterminal"},{"name":"str"}],"output":null}],[11,"is_match","","",3,{"inputs":[{"name":"clipropertyfn"},{"name":"str"}],"output":{"name":"bool"}}],[11,"autocomplete","","",3,{"inputs":[{"name":"clipropertyfn"},{"name":"str"}],"output":{"name":"option"}}],[11,"new","","",6,{"inputs":[{"name":"clipromptautocompletebuffer"},{"name":"string"}],"output":{"name":"clipromptautocompletebuffer"}}],[11,"print_prompt","","",6,{"inputs":[{"name":"clipromptautocompletebuffer"},{"name":"t"}],"output":null}],[11,"handle_received_byte","","",6,null],[8,"CliTerminal","","Terminal trait.",null,null],[10,"output_line","","Output a string with the newline characters at the end. The implementation\nadds the newline control characters.",7,{"inputs":[{"name":"cliterminal"},{"name":"str"}],"output":null}],[8,"CliCommand","","A command that can be executed by the execution function.",null,null],[10,"execute","","Execute the command with the given line buffer",8,{"inputs":[{"name":"clicommand"},{"name":"cliterminal"},{"name":"str"}],"output":null}],[10,"is_match","","Check if the line buffer is valid for this command",8,{"inputs":[{"name":"clicommand"},{"name":"str"}],"output":{"name":"bool"}}],[10,"autocomplete","","Give auto-complete hints",8,{"inputs":[{"name":"clicommand"},{"name":"str"}],"output":{"name":"option"}}],[8,"CliPromptTerminal","","",null,null],[10,"print_bytes","","",9,null],[11,"print","","",9,{"inputs":[{"name":"clipromptterminal"},{"name":"str"}],"output":null}],[11,"print","","",9,null]],"paths":[[3,"AutocompleteLine"],[3,"CliCommandKeyword"],[3,"CliPropertyVar"],[3,"CliPropertyFn"],[4,"AutocompleteOption"],[4,"AutocompleteResult"],[3,"CliPromptAutocompleteBuffer"],[8,"CliTerminal"],[8,"CliCommand"],[8,"CliPromptTerminal"]]};
initSearch(searchIndex);
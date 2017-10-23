use prelude::v1::*;
use super::*;

#[test]
pub fn test_suggest() {
	let cmd1 = CliCommand {
		command: "status".into(),
		help: Some("Some basics about the state of the system".into())
	};
	let cmd2 = CliCommand {
		command: "status net".into(),
		help: None
	};
	let cmd3 = CliCommand {
		command: "stat".into(),
		help: None
	};
	let cmd4_args = CliCommand {
		command: "exec_args ".into(),
		help: None
	};


	let mut get_matcher = |l, m| { CliLineMatcher::new(l, m) };

	{
		let mut matcher = get_matcher("", LineMatcherMode::Execute);
		matcher.match_cmd(&cmd1);
		assert_eq!(LineBufferResult::NoMatchFound, matcher.finish());
	}

	{
		let mut matcher = get_matcher("st", LineMatcherMode::Execute);
		matcher.match_cmd(&cmd1);
		assert_eq!(LineBufferResult::NoMatchFound, matcher.finish());
	}

	{
		let mut matcher = get_matcher("status", LineMatcherMode::Execute);
		matcher.match_cmd(&cmd1);
		assert_eq!(LineBufferResult::Match { args: "".into() }, matcher.finish());
	}

	{
		let mut matcher = get_matcher("status more", LineMatcherMode::Execute);
		matcher.match_cmd(&cmd1);
		assert_eq!(LineBufferResult::Match { args: "more".into() }, matcher.finish());
	}



	
	{
		let mut matcher = get_matcher("st", LineMatcherMode::AutocompleteOnly);
		matcher.match_cmd(&cmd1);
		matcher.match_cmd(&cmd2);
		//assert_eq!(LineBufferResult::Autocomplete { result: AutocompleteResult::MultipleMatches { lines: vec![AutocompleteLine { full_new_line: "status".into(), additional_part: "atus".into() }, AutocompleteLine { full_new_line: "status net".into(), additional_part: "atus net".into() }] } }, matcher.finish());
	}

	{
		let mut matcher = get_matcher("status", LineMatcherMode::Execute);
		matcher.match_cmd(&cmd3);
		matcher.match_cmd(&cmd2);
		matcher.match_cmd(&cmd1);
		let finish = matcher.finish();
		println!("finish: {:?}", finish);
	}

	{
		let mut matcher = get_matcher("status help", LineMatcherMode::Execute);
		matcher.match_cmd(&cmd3);
		matcher.match_cmd(&cmd2);
		matcher.match_cmd(&cmd1);
		let finish = matcher.finish();
		println!("finish: {:?}", finish);
	}

	{
		let mut matcher = get_matcher("exec_args 1", LineMatcherMode::Execute);
		matcher.match_cmd(&cmd4_args);
		if let LineBufferResult::Match { args } = matcher.finish() {
			assert_eq!("1", args);
		} else {
			panic!("should match");
		}
		
	}
}

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


	let mut get_matcher = |l, m| { CliLineMatcher::new(l, m) };

	{
		let mut matcher = get_matcher("", LineMatcherMode::Execute);
		matcher.process(&cmd1);
		assert_eq!(LineBufferResult::MoreInputRequired, matcher.finish());
	}

	{
		let mut matcher = get_matcher("st", LineMatcherMode::Execute);
		matcher.process(&cmd1);
		assert_eq!(LineBufferResult::MoreInputRequired, matcher.finish());
	}

	{
		let mut matcher = get_matcher("status", LineMatcherMode::Execute);
		matcher.process(&cmd1);
		assert_eq!(LineBufferResult::Match { args: "".into() }, matcher.finish());
	}

	{
		let mut matcher = get_matcher("status more", LineMatcherMode::Execute);
		matcher.process(&cmd1);
		assert_eq!(LineBufferResult::Match { args: " more".into() }, matcher.finish());
	}



	
	{
		let mut matcher = get_matcher("st", LineMatcherMode::AutocompleteOnly);
		matcher.process(&cmd1);
		matcher.process(&cmd2);
		assert_eq!(LineBufferResult::Autocomplete { result: AutocompleteResult::MultipleMatches { lines: vec![AutocompleteLine { full_new_line: "status".into(), additional_part: "atus".into() }, AutocompleteLine { full_new_line: "status net".into(), additional_part: "atus net".into() }] } }, matcher.finish());
	}
}

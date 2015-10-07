use prelude::v1::*;
use cli::*;
use terminal::*;

#[derive(Debug, Clone, PartialEq)]
pub enum CommandError {
	InvalidInput,

	ValueTooSmall,
	ValueTooBig,

	WithMessage { error: Box<CommandError>, message: String }
}

pub trait ValueInput<T> {
	fn input(&self, s: &str) -> Result<T, CommandError>;
}

pub trait ValueInputValidate<T> {
	fn validate(&self, val: &T) -> Result<(), CommandError>;
}

pub trait ValueOutput<T> {
	fn output(&self, v: &T) -> Result<String, CommandError>;
}

pub struct ValueInputFromStr;
impl<T: FromStr> ValueInput<T> for ValueInputFromStr {
	fn input(&self, s: &str) -> Result<T, CommandError> {
		match T::from_str(s) {
			Ok(v) => Ok(v),
			Err(_) => Err(CommandError::InvalidInput)
		}
	}
}

pub struct ValueOutputToString;
impl<T: ToString> ValueOutput<T> for ValueOutputToString {
	fn output(&self, v: &T) -> Result<String, CommandError> {
		Ok(v.to_string())
	}
}

pub struct ValueInputWithValidation<T, I, V> where I: ValueInput<T>, V: ValueInputValidate<T> {
	t: PhantomData<T>,
	input: I,
	validate: V
}
impl<T, I, V> ValueInput<T> for ValueInputWithValidation<T, I, V> where I: ValueInput<T>, V: ValueInputValidate<T> {
	fn input(&self, s: &str) -> Result<T, CommandError> {
		let val = try!(self.input.input(s));
		try!(self.validate.validate(&val));
		Ok(val)
	}
}

pub struct ValueMin<T> { min: T }
impl<T: PartialOrd> ValueInputValidate<T> for ValueMin<T> {
	fn validate(&self, val: &T) -> Result<(), CommandError> {
		if *val < self.min {
			Err(CommandError::ValueTooSmall)
		} else {
			Ok(())
		}
	}
}

pub struct ValueMax<T> { max: T }
impl<T: PartialOrd> ValueInputValidate<T> for ValueMax<T> {
	fn validate(&self, val: &T) -> Result<(), CommandError> {
		if *val > self.max {
			Err(CommandError::ValueTooBig)
		} else {
			Ok(())
		}
	}
}

pub struct ValueCombineValidators<T, A, B> where A: ValueInputValidate<T>, B: ValueInputValidate<T> {
	t: PhantomData<T>,
	a: A,
	b: B
}

impl<T, A, B> ValueInputValidate<T> for ValueCombineValidators<T, A, B> where A: ValueInputValidate<T>, B: ValueInputValidate<T> {
	fn validate(&self, val: &T) -> Result<(), CommandError> {
		try!(self.a.validate(val));
		try!(self.b.validate(val));
		Ok(())
	}
}

pub trait CliProperty<'a> {
	fn set<T: LineTerminalWriter>(&mut self, args: &str, terminal: &mut T) -> Result<(), CommandError>;
	fn get<T: LineTerminalWriter>(&self, terminal: &mut T) -> Result<(), CommandError>;

	fn command_get(&self) -> &CliCommand<'a>;
	fn command_set(&self) -> &CliCommand<'a>;
}

pub trait CliPropertyData<T> {
	fn get_value(&self) -> &T;
	fn set_value(&mut self, value: T);
}

/// Owned property that can be changed with ```set var_name <value>``` and retrieved with 
/// ```get var_name```.
pub struct Property<'a, T, I: ValueInput<T>, O: ValueOutput<T>>
{
	id: Cow<'a, str>,
	/// Current value of the property
	value: T,

	input: I,
	output: O,

	command_get: CliCommand<'a>,
	command_set: CliCommand<'a>
}

impl<'a, T, P: ValueInput<T>, O: ValueOutput<T>> CliProperty<'a> for Property<'a, T, P, O> {
	fn set<Term: LineTerminalWriter>(&mut self, args: &str, terminal: &mut Term) -> Result<(), CommandError> {
		match self.input.input(args) {
			Ok(v) => {
				self.value = v;
				Ok(())
			},
			Err(e) => {
				let s = format!("Can't set property {}: {:?}", self.id, e);
				terminal.print_line(&s);
				Err(e)
			}
		}
	}

	fn get<Term: LineTerminalWriter>(&self, terminal: &mut Term) -> Result<(), CommandError> {
		let output = try!(self.output.output(&self.value));
		let s = format!("{} = {}", self.id, output);
		terminal.print_line(&s);
		Ok(())
	}

	fn command_get(&self) -> &CliCommand<'a> {
		&self.command_get
	}
	fn command_set(&self) -> &CliCommand<'a> {
		&self.command_set
	}
}

pub fn new_property<'a, T: FromStr + ToString>(id: Cow<'a, str>, value: T) -> 
	Property<'a, T, ValueInputFromStr, ValueOutputToString>
{
	let command_get = CliCommand {
		command: format!("get {}", id).into(),
		help: None
	};

	let command_set = CliCommand {
		command: format!("set {} ", id).into(),
		help: None
	};

	Property {
		id: id.into(),
		value: value,
		input: ValueInputFromStr,
		output: ValueOutputToString,

		command_get: command_get,
		command_set: command_set
	}
}

pub fn new_property_min_max<'a, T: FromStr + ToString + PartialOrd>(id: Cow<'a, str>, value: T, min: T, max: T) -> 
	Property
		<'a, T, 
		ValueInputWithValidation<T, ValueInputFromStr, ValueCombineValidators<T, ValueMin<T>, ValueMax<T>>>,
		ValueOutputToString>
{
	let min = ValueMin { min: min };
	let max = ValueMax { max: max };
	let validate = ValueCombineValidators { t: PhantomData, a: min, b: max };
	let input = ValueInputWithValidation { t: PhantomData, input: ValueInputFromStr, validate: validate };

	let command_get = CliCommand {
		command: format!("get {}", id).into(),
		help: None
	};

	let command_set = CliCommand {
		command: format!("set {} ", id).into(),
		help: None
	};

	Property {
		id: id.into(),
		value: value,
		input: input,
		output: ValueOutputToString,

		command_get: command_get,
		command_set: command_set
	}
}
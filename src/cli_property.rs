use prelude::v1::*;
use terminal::*;

pub enum PropertyCommandStyle {
	DelimitedGetSet
}

pub enum PropertyContext<'b, V> {
	Get(PropertyContextGet<'b>),
	Set(PropertyContextSet<'b, V>)
}

impl<'b, V> PropertyContext<'b, V> {
	/// Retrieve or update the variable. The type of the property has to implement the Display
	/// and Copy traits.
	pub fn apply(&mut self, property_value: &mut V) where V: Display + Copy {
	    match self {
	    	&mut PropertyContext::Get(ref mut get) => {
				get.print_value_display(property_value);
            },
            &mut PropertyContext::Set(ref mut set) => {
                *property_value = set.value;
				set.common.terminal.print_line("Value set.");
            }
	    }
	}
}

pub struct PropertyContextGet<'b> {
	pub common: PropertyContextCommon<'b>
}

impl<'b> PropertyContextGet<'b> {
	pub fn print_value_display<V: Display>(&mut self, val: V) {
		self.common.terminal.print_line(&format!("{} = {}", self.common.id, val));
	}

	pub fn print_value_debug<V: Debug>(&mut self, val: V) {
		self.common.terminal.print_line(&format!("{} = {:?}", self.common.id, val));
	}
}

pub struct PropertyContextSet<'b, V> {
	pub common: PropertyContextCommon<'b>,
	pub value: V
}

pub struct PropertyContextCommon<'b> {
	pub args: Cow<'b, str>,
	pub terminal: &'b mut CharacterTerminalWriter,
	pub current_path: &'b str,
	pub id: Cow<'b, str>,
	pub style: PropertyCommandStyle
}

impl<'b> PropertyContextCommon<'b> {
	pub fn get_args(&self) -> &str {
		&self.args
	}

	pub fn get_terminal(&mut self) -> &mut CharacterTerminalWriter {
		self.terminal
	}

	pub fn get_current_path(&self) -> &str {
		&self.current_path
	}

	pub fn get_property_id(&self) -> &str {
		&self.id
	}
}

use prelude::v1::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValidationError<T> {
	InvalidInput,

	ValueTooSmall { min: T, val: T },
	ValueTooBig { max: T, val: T }
}

pub trait ValueInput<T> {
	fn input(&self, s: &str) -> Result<T, PropertyValidationError<T>>;
}

pub trait ValueInputValidate<T> {
	fn validate(&self, val: &T) -> Result<(), PropertyValidationError<T>>;
}

pub trait ValueOutput<T> {
	fn output(&self, v: &T) -> Result<String, PropertyValidationError<T>>;
}

pub struct ValueInputFromStr;
impl<T: FromStr> ValueInput<T> for ValueInputFromStr {
	fn input(&self, s: &str) -> Result<T, PropertyValidationError<T>> {
		match T::from_str(s) {
			Ok(v) => Ok(v),
			Err(_) => Err(PropertyValidationError::InvalidInput)
		}
	}
}

pub struct ValueOutputToString;
impl<T: ToString> ValueOutput<T> for ValueOutputToString {
	fn output(&self, v: &T) -> Result<String, PropertyValidationError<T>> {
		Ok(v.to_string())
	}
}

/// A convenience parser for boolean properties. Parses 0/1, false/true,
/// off/on and no/yes.
pub struct ValueBool;
impl ValueInput<bool> for ValueBool {
	fn input(&self, s: &str) -> Result<bool, PropertyValidationError<bool>> {
		match s.to_lowercase().trim() {
			"0" | "false" | "off" | "no" => Ok(false),
			"1" | "true" | "on" | "yes" => Ok(true),
			_ => Err(PropertyValidationError::InvalidInput)
		}
	}
}
impl ValueOutput<bool> for ValueBool {
	fn output(&self, v: &bool) -> Result<String, PropertyValidationError<bool>> {
		Ok(v.to_string())
	}
}


pub struct ValueInputWithValidation<T, I, V> where I: ValueInput<T>, V: ValueInputValidate<T> {
	t: PhantomData<T>,
	input: I,
	validate: V
}
impl<T, I, V> ValueInput<T> for ValueInputWithValidation<T, I, V> where I: ValueInput<T>, V: ValueInputValidate<T> {
	fn input(&self, s: &str) -> Result<T, PropertyValidationError<T>> {
		let val = try!(self.input.input(s));
		try!(self.validate.validate(&val));
		Ok(val)
	}
}

pub struct ValueMin<T> { min: T }
impl<T: PartialOrd + Copy> ValueInputValidate<T> for ValueMin<T> {
	fn validate(&self, val: &T) -> Result<(), PropertyValidationError<T>> {
		if *val < self.min {
			Err(PropertyValidationError::ValueTooSmall { min: self.min, val: *val })
		} else {
			Ok(())
		}
	}
}

pub struct ValueMax<T> { max: T }
impl<T: PartialOrd + Copy> ValueInputValidate<T> for ValueMax<T> {
	fn validate(&self, val: &T) -> Result<(), PropertyValidationError<T>> {
		if *val > self.max {
			Err(PropertyValidationError::ValueTooBig { max: self.max, val: *val })
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
	fn validate(&self, val: &T) -> Result<(), PropertyValidationError<T>> {
		try!(self.a.validate(val));
		try!(self.b.validate(val));
		Ok(())
	}
}



pub fn validate_property_min_max<T>(min: T, max: T) -> ValueInputWithValidation<T, ValueInputFromStr, ValueCombineValidators<T, ValueMin<T>, ValueMax<T>>>
	where T: FromStr + Copy + PartialOrd
{
	let min = ValueMin { min: min };
	let max = ValueMax { max: max };
	let validate = ValueCombineValidators { t: PhantomData, a: min, b: max };
	let input = ValueInputWithValidation { t: PhantomData, input: ValueInputFromStr, validate: validate };
	input
}

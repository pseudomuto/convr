use std::fmt;
use std::num;
use std::process;
use std::result;

/// A custom Result for the library.
pub type Result = anyhow::Result<Value>;

/// A family of measurements (e.g. Lengths, Temperatures, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct Family {
    pub id: String,
    pub units: Vec<Unit>,
    pub base_unit: String,
}

impl Family {
    /// Returns true when this family contains the specified unit.
    pub fn can_convert(&self, unit: &str) -> bool {
        self.find_unit(unit).is_some()
    }

    /// Converts the value into the specified unit. This is done by first
    /// ensuring that the value is in the base unit, and then converting it
    /// into the target unit.
    pub fn convert(&self, v: Value, u: &str) -> Result {
        // Short circuit if the units are the same.
        if v.unit == u {
            return Ok(v);
        }

        // Ensure we're working from the base unit.
        let mut base_val = v.clone();
        if v.unit != self.base_unit {
            base_val = self.to_base_unit(v)?;
        }

        // Convert to the destination unit.
        self.to_dest_unit(base_val.quantity, u)
    }

    fn to_base_unit(&self, v: Value) -> Result {
        self.find_unit(&v.unit)
            .map(|u| Value::new((v.quantity + u.difference) * u.ratio, &self.base_unit))
            .ok_or(anyhow!("unknown unit: {}", &v.unit))
    }

    fn to_dest_unit(&self, base_qty: f64, unit: &str) -> Result {
        self.find_unit(unit)
            .map(|c| Value::new(base_qty * (1.0 / c.ratio) - c.difference, unit))
            .ok_or(anyhow!(
                "failed to convert {} from {} to {}",
                base_qty,
                self.base_unit,
                unit
            ))
    }

    fn find_unit(&self, unit: &str) -> Option<&Unit> {
        let unit = unit.to_lowercase();
        self.units
            .iter()
            .find(|c| c.names.contains(&unit) || c.symbol == unit)
    }
}

/// Defines a single unit of measurement (within a Family).
///
/// Conversions leverage the ratio and difference fields to convert to and from
/// the family's base unit.
///
/// To base: (qty + unit.difference) * unit.ratio
/// From base: qty * 1.0 / unit.ratio - unit.difference
///
/// See temperature.rs for examples.
#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    /// The singular and plural (optional) names of the unit.
    pub names: Vec<String>,
    /// The symbol for the unit (e.g. `m` for meters).
    pub symbol: String,
    /// The scale ratio used to convert a value from the base unit.
    pub ratio: f64,
    /// The difference to add when converting to the base unit.
    pub difference: f64,
}

impl Unit {
    pub fn new(names: Vec<&str>, sym: &str, ratio: f64, difference: f64) -> Self {
        Self {
            names: names.iter().map(|n| n.to_lowercase()).collect(),
            symbol: sym.to_lowercase(),
            ratio,
            difference,
        }
    }
}

/// A custom error used to signify errors during parsing.
#[derive(Debug, PartialEq, Eq)]
pub struct ParseValueError {
    description: String,
}

impl ParseValueError {
    /// Creates a new ParseValueError, cloning the supplied message in the
    /// process.
    fn new(msg: &str) -> Self {
        Self {
            description: msg.into(),
        }
    }
}

/// Marks ParseValueError as an Error.
impl std::error::Error for ParseValueError {}

/// Implements fmt::Display for ParseValueError.
///
/// This simply prints the error description.
impl fmt::Display for ParseValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

/// Implements From<num::ParseFloatError> for ParseValueError.
impl From<num::ParseFloatError> for ParseValueError {
    fn from(err: num::ParseFloatError) -> ParseValueError {
        ParseValueError::new(&err.to_string())
    }
}

/// Defines a Value as a quantity and unit.
#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub quantity: f64,
    pub unit: String,
}

impl Value {
    /// Constructs a new Value from the supplied arguments. The second argument
    /// will be cloned.
    pub fn new(quantity: f64, unit: &str) -> Self {
        Self {
            quantity,
            unit: unit.into(),
        }
    }

    /// Returns a default Result which can be used as a return from main and/or
    /// testing functions.
    pub fn ok() -> Result {
        Ok(Self::new(0.0, ""))
    }
}

/// Implements fmt::Display for Value.
///
/// This will print the value (rounded to 2 decimal places) and the unit.
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}{}", self.quantity, self.unit)
    }
}

/// Implements str::FromStr for Value.
///
/// This makes the following possible:
///
/// ```
/// use std::str::FromStr;
///
/// let val1 = core::Value::from_str("100c");
/// let val2 = "100c".parse::<core::Value>();
/// ```
impl std::str::FromStr for Value {
    type Err = ParseValueError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: regex::Regex =
                regex::Regex::new(r"^\s*(-?\d+\.?\d*)\s*([^s]+)\s*$").unwrap();
        }

        if let Some(cap) = RE.captures_iter(s).next() {
            let val = &cap[1].parse::<f64>()?;
            return Ok(Self::new(*val, &cap[2].to_lowercase()));
        }

        Err(ParseValueError::new("invalid value"))
    }
}

/// Implements process::Termination for Value.
///
/// This allows main and/or test functions to use the `?` operator.
///
/// # Example
///
/// ```
/// fn main() -> core::Result {
///     let val = "100f".parse::<core::Value>()?;
///     println!("{}", val);
///     core::Value::ok()
/// }
/// ```
impl process::Termination for Value {
    fn report(self) -> process::ExitCode {
        process::ExitCode::SUCCESS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::assertions::*;

    #[test]
    fn value_from_str() {
        let cases = [
            ("1m", Value::new(1.0, "m")),
            ("1 m", Value::new(1.0, "m")),
            ("1M", Value::new(1.0, "m")),
            ("-12.3km", Value::new(-12.3, "km")),
        ];

        cases.map(|(given, want)| {
            assert_eq!(want, given.parse().unwrap());
        });
    }

    #[test]
    fn unit() {
        let unit = Unit::new(vec!["one", "TWO", "tHrEe"], "u", 1.0 / 3.9, 43.5);
        assert_eq!(vec!["one", "two", "three"], unit.names);
        assert_eq!("u", unit.symbol);
        assert_eq!(1.0 / 3.9, unit.ratio);
        assert_eq!(43.5, unit.difference);
    }

    #[test]
    fn family_convert() {
        let fam = Family {
            id: "test".into(),
            base_unit: "k".into(),
            units: vec![
                Unit::new(vec!["kelvin", "kelvins"], "K", 1.0, 0.0),
                Unit::new(vec!["celsius"], "C", 1.0, 273.15),
                Unit::new(vec!["fahrenheit"], "F", 5.0 / 9.0, 459.67),
            ],
        };

        assert_eq!(true, fam.can_convert("k"));
        assert_eq!(true, fam.can_convert("c"));
        assert_eq!(true, fam.can_convert("f"));
        assert_eq!(false, fam.can_convert("r"));

        let cases = [
            ("100k", "100k"),
            ("100c", "100c"),
            ("100f", "100f"),
            ("100c", "373.15k"),
            ("212f", "373.15k"),
            ("373.15k", "212f"),
            ("373.15k", "100c"),
        ];

        _ = cases.map(|(given, want)| -> Result {
            let given = given.parse::<Value>()?;
            let want = want.parse::<Value>()?;
            assert_in_delta(&want, &fam.convert(given.clone(), &want.unit)?)
        });
    }
}

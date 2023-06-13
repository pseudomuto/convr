#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;
extern crate regex;

mod length;
mod prelude;
mod temperature;
mod testutil;

use prelude::Family;
pub use prelude::{Result, Unit, Value};
use std::collections;

lazy_static! {
    static ref FAMILIES: Vec<Family> = vec![length::family(), temperature::family(),];
}

/// Returns a new Conversion object which can be used to convert the given value
/// into another unit (via the `to` function).
///
/// # Examples
///
/// ```
/// // Parse a string and convert it.
/// # fn main() -> core::Result {
/// let val = "100c".parse()?;
/// println!("{}", core::convert(val, "f")?);
/// // 212.00f
/// # core::Value::ok()
/// # }
/// ```
///
/// ```
/// // Write your own conversion functions.
/// fn meters_to_kms(m: f64) -> core::Result {
///   let val = core::Value::new(m, "m");
///   core::convert(val, "km")
/// }
///
/// # fn main() -> core::Result {
/// println!("{}", meters_to_kms(10.0)?);
/// // 0.01km
/// # core::Value::ok()
/// # }
/// ```
pub fn convert(v: Value, to_unit: &str) -> Result {
    FAMILIES
        .iter()
        .find(|f| f.can_convert(&v.unit))
        .map(|f| f.convert(v, to_unit))
        .unwrap()
}

/// Returns all available units in this library, keyed by the family (e.g. length, temp, etc.).
pub fn units<'a>() -> collections::HashMap<&'a str, &'a Vec<Unit>> {
    FAMILIES
        .iter()
        .fold(collections::HashMap::new(), |mut acc, f| {
            _ = acc.insert(&f.id, &f.units);
            acc
        })
}

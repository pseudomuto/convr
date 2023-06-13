#[cfg(test)]
pub mod assertions {
    use crate::prelude::{Family, Result, Value};

    /// EPSILON defines the value used in assert_in_delta to compare values.
    /// Two values are "equivalent" if their difference is less than this
    /// value.
    const EPSILON: f64 = 0.001;

    /// TestCase defines a unit conversion test case. It's a tuple of two strings,
    /// namely a unit and it's expected value in the base unit for the family.
    pub type TestCase<'a> = (&'a str, &'a str);

    /// Ensures that converting the given value of each TestCase results in the equivalent
    /// value.
    pub fn assert_identities(fam: &Family, cases: &Vec<TestCase>) {
        let mut res = cases.iter().map(|(given, _)| -> Result {
            let given = given.parse::<Value>()?;
            assert_in_delta(&given, &fam.convert(given.clone(), &given.unit)?)
        });

        assert_eq!(true, res.all(|r| r.is_ok()));
    }

    /// Ensures that each value can be converted from the given unit to the base unit.
    pub fn assert_to_base_unit(fam: &Family, cases: &Vec<TestCase>) {
        let mut res = cases.iter().map(|(given, want)| -> Result {
            let given = given.parse::<Value>()?;
            let want = want.parse::<Value>()?;
            assert_in_delta(&want, &fam.convert(given.clone(), &want.unit)?)
        });

        assert_eq!(true, res.all(|r| r.is_ok()));
    }

    /// Ensures that each value can be converted from the base unit to the given unit.
    pub fn assert_from_base_unit(fam: &Family, cases: &Vec<TestCase>) {
        let mut res = cases.iter().map(|(want, given)| -> Result {
            let given = given.parse::<Value>()?;
            let want = want.parse::<Value>()?;
            assert_in_delta(&want, &fam.convert(given.clone(), &want.unit)?)
        });

        assert_eq!(true, res.all(|r| r.is_ok()));
    }

    /// Ensures that the difference between values is < EPSILON.
    pub fn assert_in_delta(exp: &Value, got: &Value) -> Result {
        assert_eq!(
            true,
            exp.quantity - got.quantity < EPSILON && got.quantity - exp.quantity < EPSILON,
            "expected: {}, got: {}",
            exp,
            got,
        );

        Value::ok()
    }
}

use super::prelude::{Family, Unit};

/// Returns a family that can convert between temperature units.
pub fn family() -> Family {
    Family {
        id: "Temperature".into(),
        base_unit: "K".into(),
        units: vec![
            Unit::new(vec!["kelvin", "kelvins"], "K", 1.0, 0.0),
            Unit::new(vec!["celsius"], "C", 1.0, 273.15),
            Unit::new(vec!["fahrenheit"], "F", 5.0 / 9.0, 459.67),
            Unit::new(vec!["rankine"], "R", 5.0 / 9.0, 0.0),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::assertions::*;

    #[test]
    fn convert() {
        let fam = family();
        let cases = vec![
            ("100k", "100k"),
            ("100c", "373.15k"),
            ("212f", "373.15k"),
            ("671.67r", "373.15k"),
        ];

        assert_identities(&fam, &cases);
        assert_to_base_unit(&fam, &cases);
        assert_from_base_unit(&fam, &cases);
    }
}

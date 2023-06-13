use super::prelude::{Family, Unit};

/// Returns a Family that converts between units of length (e.g. m, km, ft, etc.).
pub fn family() -> Family {
    Family {
        id: "Lengths".into(),
        base_unit: "M".into(),
        units: vec![
            // metric units
            Unit::new(vec!["meter", "meters"], "M", 1.0, 0.0),
            Unit::new(vec!["centimeter", "centimeters"], "CM", 1.0 / 100.0, 0.0),
            Unit::new(vec!["millimeter", "millimeters"], "MM", 1.0 / 1000.0, 0.0),
            Unit::new(vec!["kilometer", "kilometers"], "KM", 1000.0, 0.0),
            // imperial units
            Unit::new(vec!["foot", "feet"], "ft", 0.3048, 0.0),
            Unit::new(vec!["inch", "inches"], "in", 0.0254, 0.0),
            Unit::new(vec!["yard", "yards"], "yd", 0.9144, 0.0),
            Unit::new(vec!["mile", "miles"], "mi", 1609.344, 0.0),
            Unit::new(vec!["nautical mile", "nautical miles"], "nmi", 1852.0, 0.0),
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
            ("100m", "100m"),
            ("100cm", "1m"),
            ("100mm", "0.1m"),
            ("10km", "10000m"),
            ("100ft", "30.48m"),
            ("100in", "2.54m"),
            ("100yd", "91.44m"),
            ("10mi", "16093.44m"),
            ("10nmi", "18520m"),
        ];

        assert_identities(&fam, &cases);
        assert_to_base_unit(&fam, &cases);
        assert_from_base_unit(&fam, &cases);
    }
}

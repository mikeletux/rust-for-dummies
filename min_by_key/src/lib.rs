//==========================================================
// Step 1
//==========================================================

/*
In this interview we'll pretend we're building a new database of { string: number } records with no particular order.

Write a function that returns the row containing the minimum value for a specified key.

Notes:
1) Records that do not contain the specified key are considered to have value 0 for the key.
2) If several records share the same minimum value for the chosen key, you may return any of them.
3) Records may contain negative values.

Example:
input: "a", [{"a": 1, "b": 2}, {"a": 2}]
output: {"a": 1, "b": 2}
*/

use std::collections::HashMap;

pub type Record = HashMap<String, i32>;

/// Returns the record with the minimum value for `key`,
/// or `None` if `records` is empty.
pub fn min_by_key<'a>(key: &str, records: &'a [Record]) -> Option<&'a Record> {
    records
        .iter()
        .min_by_key(|r| r.get(key).copied().unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(pairs: &[(&str, i32)]) -> Record {
        pairs.iter().map(|&(k, v)| (k.to_string(), v)).collect()
    }

    #[test]
    fn min_first() {
        let records = vec![record(&[("a", 1), ("b", 2)]), record(&[("a", 2)])];
        assert_eq!(min_by_key("a", &records), Some(&records[0]));
    }

    #[test]
    fn min_last() {
        let records = vec![record(&[("a", 2)]), record(&[("a", 1), ("b", 2)])];
        assert_eq!(min_by_key("a", &records), Some(&records[1]));
    }

    #[test]
    fn missing_key_counts_as_zero() {
        let records = vec![record(&[("a", 1), ("b", 2)]), record(&[("a", 2)])];
        assert_eq!(min_by_key("b", &records), Some(&records[1]));
    }

    #[test]
    fn empty_record() {
        let records = vec![record(&[])];
        assert_eq!(min_by_key("a", &records), Some(&records[0]));
    }

    #[test]
    fn negative_values() {
        let records = vec![record(&[("a", -1)]), record(&[("b", -1)])];
        assert_eq!(min_by_key("b", &records), Some(&records[1]));
    }

    #[test]
    fn no_records() {
        let records: Vec<Record> = vec![];
        assert_eq!(min_by_key("a", &records), None);
    }
}

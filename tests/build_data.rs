#[path = "../build_support.rs"]
mod build_support;

use build_support::{merge_years, Schema};

#[test]
fn merges_only_next_year_dates_with_next_year_precedence() {
    let input = [
        r#"{"year":2019,"days":[{"date":"2018-12-29","name":"next","isOffDay":false},{"date":"2019-01-01","name":"new year","isOffDay":true}]}"#,
        r#"{"year":2018,"days":[{"date":"2018-12-29","name":"old","isOffDay":true},{"date":"2017-12-31","name":"original","isOffDay":true}]}"#,
        r#"{"year":2016,"days":[{"date":"2018-12-30","name":"ignore","isOffDay":true}]}"#,
    ];
    let schemas = input
        .iter()
        .map(|json| {
            let schema: Schema = serde_json::from_str(json).unwrap();
            (schema.year, schema.days)
        })
        .collect();
    let merged = merge_years(schemas);
    assert_eq!(merged[&2018]["2018-12-29"], ("next".into(), false));
    assert!(merged[&2018].contains_key("2017-12-31"));
    assert!(!merged[&2018].contains_key("2019-01-01"));
    assert!(!merged[&2018].contains_key("2018-12-30"));
    assert!(!merged.contains_key(&2017));
    assert_eq!(merged[&2019]["2018-12-29"], ("next".into(), false));
    assert!(merged[&2019].contains_key("2019-01-01"));
}

#[test]
fn public_queries_include_cross_year_arrangements() {
    for (date, off) in [
        ("2018-12-29", false),
        ("2018-12-30", true),
        ("2018-12-31", true),
        ("2022-12-31", true),
    ] {
        assert_eq!(holiday_cn::is_offday(date).unwrap(), (off, Some("元旦")));
        assert_eq!(holiday_cn::is_workday(date).unwrap(), !off);
        assert_eq!(
            holiday_cn::get_year_data(date[..4].parse().unwrap()).unwrap()[date],
            ("元旦", off)
        );
    }
    assert!(holiday_cn::get_year_data(1900).is_none());
    assert_eq!(holiday_cn::is_offday("1900-01-01").unwrap(), (false, None));
}

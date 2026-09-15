use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct Schema {
    pub year: i32,
    pub days: Vec<Day>,
}

#[derive(Clone, Deserialize)]
pub struct Day {
    pub name: String,
    pub date: String,
    #[serde(rename = "isOffDay")]
    pub is_off_day: bool,
}

// Preserve original arrangements and merge only dates in the queried calendar
// year from the following arrangement. Next-year entries win on overlap.
pub fn merge_years(
    schemas: BTreeMap<i32, Vec<Day>>,
) -> BTreeMap<i32, BTreeMap<String, (String, bool)>> {
    schemas
        .iter()
        .map(|(&year, days)| {
            let mut data: BTreeMap<_, _> = days
                .iter()
                .map(|day| (day.date.clone(), (day.name.clone(), day.is_off_day)))
                .collect();
            if let Some(next) = schemas.get(&(year + 1)) {
                let prefix = format!("{year:04}-");
                for day in next.iter().filter(|day| day.date.starts_with(&prefix)) {
                    data.insert(day.date.clone(), (day.name.clone(), day.is_off_day));
                }
            }
            (year, data)
        })
        .collect()
}

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

mod build_support;
use build_support::{merge_years, Schema};

fn main() {
    let submodule_dir = "holiday-cn";
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("holiday_data.rs");

    let mut schemas = BTreeMap::new();
    let mut all_years = Vec::new();
    let mut all_data = String::new();

    // Read JSON files from submodule
    for entry in fs::read_dir(submodule_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let filename = path.file_name().unwrap().to_str().unwrap();
        if filename == "schema.json" || filename == "renovate.json" {
            continue;
        }

        let content = fs::read_to_string(&path).unwrap();
        let schema: Schema = serde_json::from_str(&content).unwrap();

        assert!(
            schemas.insert(schema.year, schema.days).is_none(),
            "duplicate year"
        );
    }

    for (year, days_map) in merge_years(schemas) {
        all_years.push(year);

        all_data.push_str(&format!(
            r#"
const YEAR_{}_DATA: &[(&str, &str, bool)] = &[
{}
];

fn load_year_{}_data() -> HashMap<String, (&'static str, bool)> {{
    YEAR_{}_DATA.iter()
        .map(|&(date, name, is_off_day)| {{
            (date.to_string(), (name, is_off_day))
        }})
        .collect()
}}
"#,
            year,
            days_map
                .into_iter()
                .map(|(date, (name, is_off_day))| {
                    format!("    ({date:?}, {name:?}, {is_off_day}),")
                })
                .collect::<Vec<_>>()
                .join("\n"),
            year,
            year,
        ));
    }

    // Generate the final code
    let generated_code = format!(
        r#"
use std::collections::HashMap;
use once_cell::sync::Lazy;
use dashmap::DashMap;

type YearData = HashMap<String, (&'static str, bool)>;

static YEAR_DATA: Lazy<DashMap<i32, YearData>> = Lazy::new(DashMap::new);

pub fn get_year_data(year: i32) -> Option<HashMap<String, (&'static str, bool)>> {{
    // First check if we already have the data
    if let Some(data) = YEAR_DATA.get(&year) {{
        return Some(data.clone());
    }}

    // If not found, load the data based on the year
    let data = match year {{
        {}
        _ => return None,
    }};

    // Store the data in the cache
    YEAR_DATA.insert(year, data.clone());
    Some(data)
}}

{}
"#,
        all_years
            .iter()
            .map(|year| format!("        {} => load_year_{}_data(),", year, year))
            .collect::<Vec<_>>()
            .join("\n"),
        all_data
    );

    fs::write(dest_path, generated_code).unwrap();
    println!("cargo:rerun-if-changed=holiday-cn");
    println!("cargo:rerun-if-changed=build_support.rs");
}

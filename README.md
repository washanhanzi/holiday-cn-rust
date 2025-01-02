# holiday-cn-rust

A Rust library for checking Chinese holidays and workdays. Data is automatically updated daily.

[![Crates.io](https://img.shields.io/crates/v/holiday-cn.svg)](https://crates.io/crates/holiday-cn)
[![Documentation](https://docs.rs/holiday-cn/badge.svg)](https://docs.rs/holiday-cn)

## Features

- Check if a date is a holiday or workday in China

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
holiday-cn = "0.1"
```

## Usage

### Basic Usage

```rust
use holiday_cn::{is_workday, is_offday};
use time::Date;

fn main() -> Result<(), time::Error> {
    // Check if a specific date is a workday
    let date = Date::from_calendar_date(2025, time::Month::January, 1)?;
    let (is_work, holiday_name) = is_workday(&date.format(&format_description::parse("[year]-[month]-[day]")?)?)?;
    println!("2025-01-01 is{} a workday", if is_work { "" } else { " not" });
    if let Some(name) = holiday_name {
        println!("Holiday name: {}", name);
    }

    // Check if current time is an off-day
    let (is_off, holiday_name) = is_now_offday();
    println!("Current time is{} an off-day", if is_off { "" } else { " not" });
    if let Some(name) = holiday_name {
        println!("Holiday name: {}", name);
    }

    Ok(())
}
```

### Error Handling

```rust
use holiday_cn::is_offday;
use time::Date;

fn main() {
    // Example with error handling
    let date_str = "2025-01-01";
    match Date::parse(date_str, &format_description::parse("[year]-[month]-[day]").unwrap()) {
        Ok(date) => {
            match is_offday(&date_str) {
                Ok((is_off, holiday_name)) => {
                    println!("{} is{} an off-day", date_str, if is_off { "" } else { " not" });
                    if let Some(name) = holiday_name {
                        println!("Holiday name: {}", name);
                    }
                }
                Err(e) => println!("Error checking holiday: {}", e),
            }
        }
        Err(e) => println!("Error parsing date: {}", e),
    }
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
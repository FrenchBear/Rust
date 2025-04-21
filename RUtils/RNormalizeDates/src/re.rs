// RNormalizeDates - Module re
// Regular expressions build and helpers
//
// 2025-04-14   PV      Moved to a separate file to reduce size of main.rs
// 2025-04-16   PV      Better normalization of n°

use regex::Regex;

pub const MONTHS: [(&str, i32); 77] = [
    ("01", 1),
    ("Janvier", 1),
    ("Janv", 1),
    ("Jan", 1),
    ("January", 1),
    ("New year", 1),
    ("Février", 2),
    ("02", 2),
    ("Fevrier", 2),
    ("F vrier", 2),
    ("Fvrier", 2),
    ("Fev", 2),
    ("Fév", 2),
    ("Feb", 2),
    ("February", 2),
    ("Mars", 3),
    ("03", 3),
    ("Mar", 3),
    ("March", 3),
    ("04", 4),
    ("Avril", 4),
    ("Avr", 4),
    ("Apr", 4),
    ("April", 4),
    ("05", 5),
    ("Mai", 5),
    ("May", 5),
    ("06", 6),
    ("Juin", 6),
    ("Jui", 6),
    ("Jun", 6),
    ("June", 6),
    ("07", 7),
    ("Juillet", 7),
    ("Juil", 7),
    ("Juill", 7),
    ("Jul", 7),
    ("July", 7),
    ("08", 8),
    ("Août", 8),
    ("Aout", 8),
    ("Ao t", 8),
    ("Aoû", 8),
    ("Aou", 8),
    ("Aug", 8),
    ("August", 8),
    ("09", 9),
    ("Septembre", 9),
    ("Sept", 9),
    ("Sep", 9),
    ("September", 9),
    ("10", 10),
    ("Octobre", 10),
    ("Oct", 10),
    ("October", 10),
    ("11", 11),
    ("Novembre", 11),
    ("Nov", 11),
    ("November", 11),
    ("12", 12),
    ("Décembre", 12),
    ("Decembre", 12),
    ("D cembre", 12),
    ("Dec", 12),
    ("Déc", 12),
    ("December", 12),
    ("Printemps", 13),
    ("Spring", 13),
    ("Été", 14),
    ("Eté", 14),
    ("Ete", 14),
    ("Summer", 14),
    ("Autonne", 15),
    ("Autumn", 15),
    ("Fall", 15),
    ("Hiver", 16),
    ("Winter", 16),
];

pub struct DatePatterns {
    pub re_date_ymd_head: Regex,
    pub re_date_ynm: Regex,
    pub re_no: Regex,

    pub re_date_ymd_std: Regex,
    pub re_date_ymm_std: Regex,

    pub re_date_mymy: Regex,
    pub re_date_ymym: Regex,

    pub re_date_mmmy: Regex,
    pub re_date_ymmm: Regex,

    pub re_date_mymmy: Regex,
    pub re_date_ymymm: Regex,

    pub re_date_mmymy: Regex,
    pub re_date_ymmym: Regex,

    pub re_date_mmy: Regex,
    pub re_date_ymm: Regex,

    pub re_date_dmdmy: Regex,

    pub re_date_dmy: Regex,
    pub re_date_ymd: Regex,

    pub re_date_my: Regex,
    pub re_date_ym: Regex,
}

impl DatePatterns {
    pub fn new() -> Self {
        // Prepare regex
        // Note: \b is a word-limit anchor, but backspace in a [class]
        let mut month = String::new();
        let mut months_sorted = MONTHS.to_vec();
        months_sorted.sort_by_key(|k| -(k.0.len() as i32));
        for &(month_name, _) in months_sorted.iter() {
            month.push_str(if month.is_empty() { r"\b(" } else { "|" });
            month.push_str(month_name);
        }
        month.push_str(r")\b");
        let month = month.as_str();
        // Years from 1920
        let year = r"\b((?:19[2-9][0-9]|20[0-2][0-9])|(?:2[0-9]))(?:B?)\b"; // New version, 2020 and more only, absorb a B following a year
        let day = r"\b(1er|30|31|(?:0?[1-9])|[12][0-9])\b";

        // Dates already valid, to ignore
        let re_date_ymd_std = Regex::new((String::from("(?i)") + year + "-" + month + "-" + day + r"(\.\." + day + ")?").as_str()).unwrap();
        let re_date_ymm_std = Regex::new((String::from("(?i)") + year + "-" + month + r"(\.\." + month + ")?").as_str()).unwrap();

        // Special patterns
        let re_date_ymd_head = Regex::new((String::from("(?i)") + "^ " + year + "[ -]" + "(0[1-9]|10|11|12)" + "[ -]" + day + " ").as_str()).unwrap();
        let re_date_ynm = Regex::new((String::from("(?i)") + year + r" +(Volume \d+ +)?№(\d+) +" + month).as_str()).unwrap();
        let re_no = Regex::new(r"(?i) n[o°]? *(\d+)").unwrap();

        let re_date_mymy = Regex::new((String::from("(?i)") + month + "[ -]+" + year + "[ à-]+" + month + "[ -]+" + year).as_str()).unwrap();
        let re_date_ymym = Regex::new((String::from("(?i)") + year + "[ -]+" + month + "[ -]+" + year + "[ -]+" + month).as_str()).unwrap();

        let re_date_mmmy = Regex::new((String::from("(?i)") + month + "[ -]+" + month + "[ -]+" + month + "[ -]+" + year).as_str()).unwrap();
        let re_date_ymmm = Regex::new((String::from("(?i)") + year + "[ -]+" + month + "[ -]+" + month + "[ -]+" + month).as_str()).unwrap();

        let re_date_mymmy =
            Regex::new((String::from("(?i)") + month + "[ -]+" + year + "[ -]+" + month + "[ -]+" + month + "[ -]+" + year).as_str()).unwrap();
        let re_date_ymymm =
            Regex::new((String::from("(?i)") + year + "[ -]+" + month + "[ -]+" + year + "[ -]+" + month + "[ -]+" + month).as_str()).unwrap();

        let re_date_mmymy =
            Regex::new((String::from("(?i)") + month + "-" + month + "[ -]+" + year + "[ -]+" + month + "[ -]+" + year).as_str()).unwrap();
        let re_date_ymmym =
            Regex::new((String::from("(?i)") + year + "[ -]+" + month + "[ -]+" + month + "[ -]+" + year + "[ -]+" + month).as_str()).unwrap();

        let re_date_mmy = Regex::new((String::from("(?i)") + month + "[ à-]+" + month + "[ -]+" + year).as_str()).unwrap();
        let re_date_ymm = Regex::new((String::from("(?i)") + year + "[ -]+" + month + "[ à-]+" + month).as_str()).unwrap();

        let re_date_dmdmy =
            Regex::new((String::from("(?i)") + day + " +" + month + " *(?:au)? *" + day + " +" + month + " +" + year).as_str()).unwrap();

        let re_date_dmy = Regex::new((String::from("(?i)") + day + " +" + month + " +" + year).as_str()).unwrap();
        let re_date_ymd = Regex::new((String::from("(?i)") + year + " +" + month + " +" + day).as_str()).unwrap();

        let re_date_my = Regex::new((String::from("(?i)") + month + "[ -]+" + year).as_str()).unwrap();
        let re_date_ym = Regex::new((String::from("(?i)") + year + "[ -]+" + month).as_str()).unwrap();

        DatePatterns {
            re_date_ymd_head,
            re_date_ynm,
            re_no,

            re_date_ymm_std,
            re_date_ymd_std,

            re_date_mymy,
            re_date_ymym,

            re_date_mmmy,
            re_date_ymmm,

            re_date_mymmy,
            re_date_ymymm,

            re_date_mmymy,
            re_date_ymmym,

            re_date_mmy,
            re_date_ymm,

            re_date_dmdmy,

            re_date_dmy,
            re_date_ymd,

            re_date_my,
            re_date_ym,
        }
    }
}

// ---------------------------------------------------
// Helpers

// Case-insensitive version of contains
pub fn icontains(s: &str, pattern: &str) -> bool {
    s.to_lowercase().contains(&pattern.to_lowercase())
}

// Case-insensitive version of replace
pub fn ireplace(s: &str, search: &str, replace: &str) -> String {
    if search.is_empty() {
        panic!("search can't be empty");
    }
    let mut result = String::new();
    let lower_s = s.to_lowercase();
    let lower_search = search.to_lowercase();
    let mut i = 0;

    while i < s.len() {
        if lower_s[i..].starts_with(&lower_search) {
            result.push_str(replace);
            i += search.len();
        } else {
            let ch = &s[i..].chars().next().unwrap();
            result.push(*ch);
            i += ch.len_utf8();
        }
    }

    result
}

// 2-digit pivot at 50: <50=19xx, >50=20xx
pub fn get_year_num(year: &str) -> i32 {
    let y = year.parse::<i32>().unwrap();
    if y > 100 {
        y
    } else if y > 50 {
        y + 1900
    } else {
        y + 2000
    }
}

// Converts all strings of MONTHS into 1..12 plus 13..16: Spring..Winter
pub fn get_month_num(month: &str) -> i32 {
    if let Ok(m) = month.parse::<i32>() {
        if (1..=12).contains(&m) {
            return m;
        }
    } else {
        let month_lc = month.to_lowercase();
        for &(month_name, month_num) in MONTHS.iter() {
            if month_lc == month_name.to_lowercase() {
                return month_num;
            }
        }
    }
    panic!("Invalid month {}", month);
}

// Conversion &str -> i32 for day
// Note that day 31 is always accepted regardless of the month; dates are never validated
pub fn get_day_num(day: &str) -> i32 {
    if let Ok(d) = day.parse::<i32>() {
        if (1..=31).contains(&d) {
            return d;
        }
    } else if day.to_lowercase() == "1er" {
        return 1;
    }
    panic!("Invalid day {}", day);
}

// Conversion 1..16 -> &str for months
pub fn get_month_name(m: i32) -> &'static str {
    assert!((1..=16).contains(&m));
    [
        "01",
        "02",
        "03",
        "04",
        "05",
        "06",
        "07",
        "08",
        "09",
        "10",
        "11",
        "12",
        "Printemps",
        "Été",
        "Autonne",
        "Hiver",
    ][(m - 1) as usize]
}

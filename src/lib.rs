/// Parses a human-readable date format:
/// 4th of July 2018
/// 2nd of March 2006
/// 3rd of November 1975

extern crate regex;

use std::collections::HashMap;
use regex::Regex;

fn months_map() -> HashMap<&'static str, &'static str> {
    let mut months = HashMap::new();

    months.insert("January", "01");
    months.insert("February", "02");
    months.insert("March", "03");
    months.insert("April", "04");
    months.insert("May", "05");
    months.insert("June", "06");
    months.insert("July", "07");
    months.insert("August", "08");
    months.insert("September", "09");
    months.insert("October", "10");
    months.insert("November", "11");
    months.insert("December", "12");

    months
}

pub fn date_parser(date: &str) -> String {
    // split string on spaces and ignore index 1 'of'
    let mut iter = date.split_whitespace();
    let day = iter.next().unwrap();
    iter.next();
    let month = iter.next().unwrap();
    let year = iter.next().unwrap();
    
    let months = months_map();

    let mut date = String::new();

    // Append the month
    date.push_str(months.get(month).unwrap());
    date.push('/');

    // Append the day
    let digit = Regex::new(r"^\d+").unwrap();
    let mut day_num = String::from(digit.find(day).unwrap().as_str());
    if day_num.len() < 2 { day_num = format!("0{}", day_num)  };
    date.push_str(&day_num);
    date.push('/');
    
    // Append the year
    date.push_str(year);

    date
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(date_parser("4th of January 2018"), "01/04/2018".to_string());
    }
}

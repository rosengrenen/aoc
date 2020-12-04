use crate::lib::Solver;
use regex::Regex;
use std::collections::HashMap;

pub struct Day4Solver;

impl Solver for Day4Solver {
  fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
    let passports = parse_passports(&lines);
    if !part_two {
      passports
        .iter()
        .fold(0, |valid_passports, passport| {
          if is_valid_passport(passport, false) {
            valid_passports + 1
          } else {
            valid_passports
          }
        })
        .to_string()
    } else {
      passports
        .iter()
        .fold(0, |valid_passports, passport| {
          if is_valid_passport(passport, true) {
            valid_passports + 1
          } else {
            valid_passports
          }
        })
        .to_string()
    }
  }
}

fn parse_passports(lines: &Vec<String>) -> Vec<HashMap<&str, &str>> {
  let mut passports: Vec<HashMap<&str, &str>> = Vec::new();
  let mut current_passport = HashMap::new();
  for line in lines.iter() {
    if line == "" {
      passports.push(current_passport);
      current_passport = HashMap::new();
      continue;
    }

    let key_value_pairs: Vec<&str> = line.split(" ").collect();
    for &pair in key_value_pairs.iter() {
      let split: Vec<&str> = pair.split(":").collect();
      current_passport.insert(split[0], split[1]);
    }
  }
  passports.push(current_passport);
  passports
}

fn is_valid_passport(passport: &HashMap<&str, &str>, validate_fields: bool) -> bool {
  if !has_required_fields(passport) {
    return false;
  }

  if validate_fields {
    for (key, value) in passport.iter() {
      if !is_valid_field(key, value) {
        return false;
      }
    }
  }

  true
}

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
fn has_required_fields(passport: &HashMap<&str, &str>) -> bool {
  for required_field in REQUIRED_FIELDS.iter() {
    if let None = passport.get(required_field) {
      return false;
    }
  }
  true
}

const ALLOWED_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
fn is_valid_field(key: &str, value: &str) -> bool {
  match key {
    "byr" => {
      let birth_year: i64 = value.parse().unwrap();
      return birth_year >= 1920 && birth_year <= 2002;
    }
    "iyr" => {
      let issuance_year: i64 = value.parse().unwrap();
      return issuance_year >= 2010 && issuance_year <= 2020;
    }
    "eyr" => {
      let expiration_year: i64 = value.parse().unwrap();
      return expiration_year >= 2020 && expiration_year <= 2030;
    }
    "hgt" => {
      if value.ends_with("cm") {
        let height: i64 = value[..value.len() - 2].parse().unwrap();
        return height >= 150 && height <= 193;
      }

      if value.ends_with("in") {
        let height: i64 = value[..value.len() - 2].parse().unwrap();
        return height >= 59 && height <= 76;
      }

      return false;
    }
    "hcl" => {
      return Regex::new("^#[0-9a-f]{6}$").unwrap().is_match(value);
    }
    "ecl" => {
      return ALLOWED_EYE_COLORS.iter().find(|&&color| color == value) != None;
    }
    "pid" => {
      return Regex::new("^[0-9]{9}$").unwrap().is_match(value);
    }
    _ => return true,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one_test_cases() {
    let input: Vec<String> = vec![
      "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
      "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
      "".to_string(),
      "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
      "hcl:#cfa07d byr:1929".to_string(),
      "".to_string(),
      "hcl:#ae17e1 iyr:2013".to_string(),
      "eyr:2024".to_string(),
      "ecl:brn pid:760753108 byr:1931".to_string(),
      "hgt:179cm".to_string(),
      "".to_string(),
      "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
      "iyr:2011 ecl:brn hgt:59in".to_string(),
    ];

    let solver = Day4Solver {};
    assert_eq!(solver.solve(input, false), "2");
  }

  #[test]
  fn is_valid_field_test_cases() {
    assert_eq!(is_valid_field("byr", "2002"), true);
    assert_eq!(is_valid_field("byr", "2003"), false);

    assert_eq!(is_valid_field("hgt", "60in"), true);
    assert_eq!(is_valid_field("hgt", "190cm"), true);
    assert_eq!(is_valid_field("hgt", "190in"), false);
    assert_eq!(is_valid_field("hgt", "190"), false);

    assert_eq!(is_valid_field("hcl", "#123abc"), true);
    assert_eq!(is_valid_field("hcl", "#123abz"), false);
    assert_eq!(is_valid_field("hcl", "123abc"), false);

    assert_eq!(is_valid_field("ecl", "brn"), true);
    assert_eq!(is_valid_field("ecl", "wat"), false);

    assert_eq!(is_valid_field("pid", "000000001"), true);
    assert_eq!(is_valid_field("pid", "0123456789"), false);
  }

  #[test]
  fn part_two_test_cases() {
    let invalid_passports_input: Vec<String> = vec![
      "eyr:1972 cid:100".to_string(),
      "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".to_string(),
      "".to_string(),
      "iyr:2019".to_string(),
      "hcl:#602927 eyr:1967 hgt:170cm".to_string(),
      "ecl:grn pid:012533040 byr:1946".to_string(),
      "".to_string(),
      "hcl:dab227 iyr:2012".to_string(),
      "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".to_string(),
      "".to_string(),
      "hgt:59cm ecl:zzz".to_string(),
      "eyr:2038 hcl:74454a iyr:2023".to_string(),
      "pid:3556412378 byr:2007".to_string(),
    ];

    let valid_passports_input: Vec<String> = vec![
      "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980".to_string(),
      "hcl:#623a2f".to_string(),
      "".to_string(),
      "eyr:2029 ecl:blu cid:129 byr:1989".to_string(),
      "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm".to_string(),
      "".to_string(),
      "hcl:#888785".to_string(),
      "hgt:164cm byr:2001 iyr:2015 cid:88".to_string(),
      "pid:545766238 ecl:hzl".to_string(),
      "eyr:2022".to_string(),
      "".to_string(),
      "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string(),
    ];

    let solver = Day4Solver {};
    assert_eq!(solver.solve(invalid_passports_input, true), "0");
    assert_eq!(solver.solve(valid_passports_input, true), "4");
  }
}

use crate::lib::Solver;
use regex::Regex;

pub struct Day4Solver;

impl Solver for Day4Solver {
	fn solve(&self, lines: &[String], part_two: bool) -> String {
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

fn parse_passports(lines: &[String]) -> Vec<Vec<(&str, &str)>> {
	let mut passports = Vec::new();
	let mut current_passport = Vec::new();
	for line in lines.iter() {
		if line.is_empty() {
			passports.push(current_passport);
			current_passport = Vec::new();
			continue;
		}

		let key_value_pairs: Vec<&str> = line.split(' ').collect();
		for &pair in key_value_pairs.iter() {
			let mut split = pair.split(':');
			current_passport.push((split.next().unwrap(), split.next().unwrap()));
		}
	}
	passports.push(current_passport);
	passports
}

fn is_valid_passport(passport: &[(&str, &str)], validate_fields: bool) -> bool {
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
fn has_required_fields(passport: &[(&str, &str)]) -> bool {
	for required_field in REQUIRED_FIELDS.iter() {
		if passport
			.iter()
			.find(|(key, _)| key == required_field)
			.is_none()
		{
			return false;
		}
	}
	true
}

const ALLOWED_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
fn is_valid_field(key: &str, value: &str) -> bool {
	lazy_static! {
		static ref HAIR_COLOR_REGEX: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
	}
	lazy_static! {
		static ref PERSONAL_ID_REGEX: Regex = Regex::new("^[0-9]{9}$").unwrap();
	}
	match key {
		"byr" => (1920..=2002).contains(&value.parse::<i64>().unwrap()),
		"iyr" => (2010..=2020).contains(&value.parse::<i64>().unwrap()),
		"eyr" => (2020..=2030).contains(&value.parse::<i64>().unwrap()),
		"hcl" => HAIR_COLOR_REGEX.is_match(value),
		"ecl" => ALLOWED_EYE_COLORS.iter().find(|&&color| color == value) != None,
		"pid" => PERSONAL_ID_REGEX.is_match(value),
		"hgt" => {
			if value.ends_with("cm") {
				let height: i64 = value.strip_suffix("cm").unwrap().parse().unwrap();
				return (150..=193).contains(&height);
			}

			if value.ends_with("in") {
				let height: i64 = value.strip_suffix("in").unwrap().parse().unwrap();
				return (59..=76).contains(&height);
			}

			false
		}
		_ => true,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::read_lines;
	use test::Bencher;

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
		assert_eq!(solver.solve(&input, false), "2");
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
		assert_eq!(solver.solve(&invalid_passports_input, true), "0");
		assert_eq!(solver.solve(&valid_passports_input, true), "4");
	}

	#[bench]
	fn bench_parse_passports(bencher: &mut Bencher) {
		let input = read_lines("src/day04/input.txt");
		bencher.iter(|| parse_passports(&input));
	}

	#[bench]
	fn bench_is_valid_passport(bencher: &mut Bencher) {
		let input = read_lines("src/day04/input.txt");
		let passports = parse_passports(&input);
		bencher.iter(|| {
			for passport in passports.iter() {
				is_valid_passport(&passport, false);
			}
		});
	}

	#[bench]
	fn bench_is_valid_passport_validate_fields(bencher: &mut Bencher) {
		let input = read_lines("src/day04/input.txt");
		let passports = parse_passports(&input);
		bencher.iter(|| {
			for passport in passports.iter() {
				is_valid_passport(&passport, true);
			}
		});
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = read_lines("src/day04/input.txt");
		let solver = Day4Solver {};
		bencher.iter(|| solver.solve(&input, false));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = read_lines("src/day04/input.txt");
		let solver = Day4Solver {};
		bencher.iter(|| solver.solve(&input, true));
	}
}

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day4;

impl Solver for Day4 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let parsed_passports_iter = parse_passports(input);
		SolverOutput::Num(filter_required_fields(parsed_passports_iter).count() as i64)
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let parsed_passports_iter = parse_passports(input);
		SolverOutput::Num(
			filter_valid_fields(filter_required_fields(parsed_passports_iter)).count() as i64,
		)
	}
}

#[derive(Default)]
struct Passport<'a> {
	byr: Option<&'a str>,
	iyr: Option<&'a str>,
	eyr: Option<&'a str>,
	hgt: Option<&'a str>,
	hcl: Option<&'a str>,
	ecl: Option<&'a str>,
	pid: Option<&'a str>,
	cid: Option<&'a str>,
}

fn parse_passports(lines: &str) -> impl Iterator<Item = Passport> {
	lines.split("\n\n").map(|passport_data| {
		let mut passport = Passport::default();
		for pair in passport_data.split_whitespace() {
			let (key, value) = pair.split_once(':').unwrap();
			match key {
				"byr" => passport.byr = Some(value),
				"iyr" => passport.iyr = Some(value),
				"eyr" => passport.eyr = Some(value),
				"hgt" => passport.hgt = Some(value),
				"hcl" => passport.hcl = Some(value),
				"ecl" => passport.ecl = Some(value),
				"pid" => passport.pid = Some(value),
				"cid" => passport.cid = Some(value),
				_ => (),
			}
		}

		passport
	})
}

fn filter_required_fields<'a, I>(passports: I) -> impl Iterator<Item = Passport<'a>>
where
	I: Iterator<Item = Passport<'a>>,
{
	passports
		.filter(|Passport { byr, .. }| byr.is_some())
		.filter(|Passport { iyr, .. }| iyr.is_some())
		.filter(|Passport { eyr, .. }| eyr.is_some())
		.filter(|Passport { hgt, .. }| hgt.is_some())
		.filter(|Passport { hcl, .. }| hcl.is_some())
		.filter(|Passport { ecl, .. }| ecl.is_some())
		.filter(|Passport { pid, .. }| pid.is_some())
}

fn filter_valid_fields<'a, I>(passports: I) -> impl Iterator<Item = Passport<'a>>
where
	I: Iterator<Item = Passport<'a>>,
{
	passports
		.filter(|Passport { byr, .. }| {
			if let Some(byr) = byr {
				(1920..=2002).contains(&byr.parse::<i64>().unwrap())
			} else {
				false
			}
		})
		.filter(|Passport { iyr, .. }| {
			if let Some(iyr) = iyr {
				(2010..=2020).contains(&iyr.parse::<i64>().unwrap())
			} else {
				false
			}
		})
		.filter(|Passport { eyr, .. }| {
			if let Some(eyr) = eyr {
				(2020..=2030).contains(&eyr.parse::<i64>().unwrap())
			} else {
				false
			}
		})
		.filter(|Passport { hgt, .. }| {
			if let Some(hgt) = hgt {
				if let Some(cm) = hgt.strip_suffix("cm") {
					(150..=193).contains(&cm.parse().unwrap())
				} else if let Some(inches) = hgt.strip_suffix("in") {
					(59..=76).contains(&inches.parse().unwrap())
				} else {
					false
				}
			} else {
				false
			}
		})
		.filter(|Passport { hcl, .. }| {
			if let Some(hcl) = hcl {
				if let Some(hcl_digits) = hcl.strip_prefix('#') {
					hcl_digits.len() == 6
						&& hcl_digits.as_bytes().iter().all(|c| c.is_ascii_hexdigit())
				} else {
					false
				}
			} else {
				false
			}
		})
		.filter(|Passport { ecl, .. }| {
			if let Some(ecl) = ecl {
				["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(ecl)
			} else {
				false
			}
		})
		.filter(|Passport { pid, .. }| {
			if let Some(pid) = pid {
				pid.len() == 9 && pid.as_bytes().iter().all(|c| c.is_ascii_digit())
			} else {
				false
			}
		})
}

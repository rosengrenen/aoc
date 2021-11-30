use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day13;

impl Solver for Day13 {
	fn part_one(&self, input: &str) -> SolverOutput {
		let (first_poss, buses) = parse_buses(input);
		// Here index is used as the wait time for the first bus of the specific interval to depart
		let mut first_avail = Bus {
			index: std::i64::MAX,
			..buses[0]
		};
		for Bus { interval, .. } in buses {
			let wait_time = ((first_poss / interval) + 1) * interval - first_poss;
			if wait_time < first_avail.index {
				first_avail = Bus {
					interval,
					index: wait_time,
				};
			}
		}

		SolverOutput::Num(first_avail.interval * first_avail.index)
	}

	fn part_two(&self, input: &str) -> SolverOutput {
		let (first_poss, mut buses) = parse_buses(input);
		buses.sort_by(|left, right| right.interval.cmp(&left.interval));
		let first_avail = (first_poss / buses[0].interval + 1) * buses[0].interval;
		// Have to subtract with index since the list is sorted and 0th index is
		// not necessarily first anymore
		let mut t = first_avail - buses[0].index;
		let mut bus_index = 1;
		let mut step = buses[0].interval;
		loop {
			let bus = buses[bus_index];
			if (t + bus.index) % bus.interval == 0 {
				step *= bus.interval;
				bus_index += 1;
				if bus_index == buses.len() {
					return SolverOutput::Num(t);
				}
			}

			t += step;
		}
	}
}

#[derive(Clone, Copy, Debug)]
struct Bus {
	interval: i64,
	index: i64,
}

fn parse_buses(input: &str) -> (i64, Vec<Bus>) {
	let lines: Vec<&str> = input.lines().collect();

	let first_poss = lines[0].parse().unwrap();
	let mut bus_intervals = Vec::new();
	for (i, s) in lines[1].split(',').enumerate() {
		if s != "x" {
			bus_intervals.push(Bus {
				interval: s.parse().unwrap(),
				index: i as i64,
			})
		}
	}
	(first_poss, bus_intervals)
}

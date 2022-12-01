use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day11;

impl Solver for Day11 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let mut seat_layout = parse_seat_layout(input);
    while let Some(new_seat_layout) = simulate_behaviour(&seat_layout, 4, 1) {
      seat_layout = new_seat_layout;
    }

    SolverOutput::Num(seat_layout.occupied_seats())
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let mut seat_layout = parse_seat_layout(input);
    while let Some(new_seat_layout) = simulate_behaviour(&seat_layout, 5, std::i64::MAX) {
      seat_layout = new_seat_layout;
    }

    SolverOutput::Num(seat_layout.occupied_seats())
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Seat {
  Empty,
  Occupied,
  Floor,
}

#[derive(Clone, Debug, PartialEq)]
struct SeatLayout {
  seats: Vec<Vec<Seat>>,
}

impl SeatLayout {
  fn get_seat(&self, x: isize, y: isize) -> Option<Seat> {
    if self.is_out_of_bounds(x, y) {
      None
    } else {
      Some(self.seats[y as usize][x as usize])
    }
  }

  fn set_seat(&mut self, x: isize, y: isize, seat: Seat) {
    if !self.is_out_of_bounds(x, y) {
      self.seats[y as usize][x as usize] = seat;
    }
  }

  fn is_out_of_bounds(&self, x: isize, y: isize) -> bool {
    x < 0 || y < 0 || y as usize >= self.seats.len() || x as usize >= self.seats[y as usize].len()
  }

  fn occupied_seats(&self) -> i64 {
    self.seats.iter().fold(0, |occupied_seats, row| {
      occupied_seats
        + row.iter().fold(0, |row_occupied_seats, &seat| {
          row_occupied_seats + if seat == Seat::Occupied { 1 } else { 0 }
        })
    })
  }
}

fn parse_seat_layout(input: &str) -> SeatLayout {
  let seats = input
    .lines()
    .map(|line| {
      line
        .as_bytes()
        .iter()
        .map(|c| match c {
          b'#' => Seat::Occupied,
          b'L' => Seat::Empty,
          b'.' => Seat::Floor,
          _ => panic!(),
        })
        .collect()
    })
    .collect();
  SeatLayout { seats }
}

fn simulate_behaviour(
  seat_layout: &SeatLayout,
  toleration: i64,
  vision_range: i64,
) -> Option<SeatLayout> {
  let mut new_seat_layout = seat_layout.clone();

  let mut has_changed = false;
  for y in 0..seat_layout.seats.len() as isize {
    for x in 0..seat_layout.seats[y as usize].len() as isize {
      if let Some(new_seat_state) = calc_new_seat_state(seat_layout, x, y, toleration, vision_range)
      {
        new_seat_layout.set_seat(x, y, new_seat_state);
        has_changed = true;
      }
    }
  }

  if has_changed {
    Some(new_seat_layout)
  } else {
    None
  }
}

static DIRECTIONS: [(isize, isize); 8] = [
  (0, 1),
  (1, 1),
  (1, 0),
  (1, -1),
  (0, -1),
  (-1, -1),
  (-1, 0),
  (-1, 1),
];
fn calc_new_seat_state(
  seat_layout: &SeatLayout,
  x: isize,
  y: isize,
  toleration: i64,
  vision_range: i64,
) -> Option<Seat> {
  if seat_layout.get_seat(x, y) == Some(Seat::Floor) {
    return None;
  }

  let mut occupied_seats_around = 0;
  for &(dx, dy) in DIRECTIONS.iter() {
    let mut iteration = 1;
    loop {
      if let Some(seat) = seat_layout.get_seat(x + iteration * dx, y + iteration * dy) {
        if seat == Seat::Occupied {
          occupied_seats_around += 1;
          break;
        } else if seat == Seat::Empty {
          break;
        }
      } else {
        break;
      }
      iteration += 1;
      if iteration > vision_range as isize {
        break;
      }
    }
  }

  if occupied_seats_around == 0 && seat_layout.get_seat(x, y) == Some(Seat::Empty) {
    return Some(Seat::Occupied);
  }

  if occupied_seats_around >= toleration && seat_layout.get_seat(x, y) == Some(Seat::Occupied) {
    return Some(Seat::Empty);
  }

  None
}

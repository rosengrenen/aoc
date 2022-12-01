use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day6;

impl Solver for Day6 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let mut questions_answered: i64 = 0;
    for batch in input.split("\n\n") {
      // "bitset" that represents if an answer was used, first bit represents 'a', the second 'b', ...
      let mut answers: i64 = 0;
      // Join all answers to get set union
      for c in batch.as_bytes().iter() {
        if *c == b'\n' {
          continue;
        }
        answers |= 1 << (c - b'a');
      }
      questions_answered += answers.count_ones() as i64;
    }

    SolverOutput::Num(questions_answered)
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let mut questions_answered: i64 = 0;
    // "bitset" that represents if an answer was used, first bit represents 'a', the second 'b', ...
    let mut answers: i64 = 0;
    // Flag that signifies first loop of new batch/group of people
    let mut first = true;
    for line in input.lines() {
      if line.is_empty() {
        questions_answered += answers.count_ones() as i64;
        first = true;
        continue;
      }
      if first {
        first = false;
        // Reset bits for new batch
        answers = 0;
        // Set the bits for the first person ov the group
        for c in line.as_bytes() {
          set_bit(&mut answers, (c - b'a') as usize);
        }
        continue;
      }
      let prev_answers = answers;
      if answers == 0 {
        continue;
      }
      answers = 0;
      // Set bits for the current persons answers
      for c in line.as_bytes() {
        set_bit(&mut answers, (c - b'a') as usize);
      }
      // AND with current and previous answers to get set intersection
      answers &= prev_answers;
    }
    questions_answered += answers.count_ones() as i64;
    SolverOutput::Num(questions_answered)
  }
}

fn set_bit(bit_set: &mut i64, bit_index: usize) {
  *bit_set |= 1 << (bit_index % 64);
}

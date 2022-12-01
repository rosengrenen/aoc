use aoc_util::{Solver, SolverOutput};
use hashbrown::HashMap;

#[derive(Default)]
pub struct Day14;

impl Solver for Day14 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let instructions = parse_instructions(input);
    let mut mask = SetMaskArgs::default();
    let mut mem: HashMap<i64, i64> = HashMap::new();
    for instruction in instructions.iter() {
      match *instruction {
        Instruction::SetMask(mask_args) => mask = mask_args,
        Instruction::Write(WriteArgs { address, value }) => {
          // First part clears bits that are set by mask and second part
          // sets mask bits
          let value = (value & !mask.mask_bits) | mask.override_bits;
          mem.insert(address, value);
        }
      }
    }

    SolverOutput::Num(mem.iter().fold(0, |p, c| p + c.1))
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let instructions = parse_instructions(input);
    let mut mask = SetMaskArgs::default();
    let mut mem: HashMap<i64, i64> = HashMap::new();
    for instruction in instructions.iter() {
      match *instruction {
        Instruction::SetMask(mask_args) => mask = mask_args,
        Instruction::Write(WriteArgs { address, value }) => {
          // The first part only save only masked bits, since the others are
          // and need to be initialised to 0's
          // The second part overwrites 0's with 1's if bitmask is 1
          let address = (address & mask.mask_bits) | (mask.mask_bits & mask.override_bits);

          // Indices of floating bits
          let mut floating_bit_indices = Vec::new();
          for i in 0..36 {
            // Inverting mask_bits gets floating bits
            if (!mask.mask_bits) & 1 << i != 0 {
              floating_bit_indices.push(i);
            }
          }

          let num_floating_bits = floating_bit_indices.len();
          // All possible ways to set the floating bits is 2^n where n is number
          // of floating bits
          for n in 0..2i64.pow(num_floating_bits as u32) {
            // Make copy of address for each iteration
            let mut address = address;
            for (i, floating_bit_index) in floating_bit_indices
              .iter()
              .enumerate()
              .take(num_floating_bits)
            {
              // Bit is one, since all floating bits in address are intialised
              // to 0's we only need to upgrade from 0's to 1's
              if n & 1 << i != 0 {
                address |= 1 << floating_bit_index;
              }
            }

            mem.insert(address, value);
          }
        }
      }
    }

    SolverOutput::Num(mem.iter().fold(0, |p, c| p + c.1))
  }
}

#[derive(Clone, Copy, Default)]
struct SetMaskArgs {
  mask_bits: i64,
  override_bits: i64,
}

struct WriteArgs {
  address: i64,
  value: i64,
}

enum Instruction {
  SetMask(SetMaskArgs),
  Write(WriteArgs),
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
  input
    .lines()
    .map(|line| {
      if let Some(mask) = line.strip_prefix("mask = ") {
        // 1=masked, 0=not masked
        let mut mask_bits = 0;
        for &c in mask.as_bytes() {
          mask_bits <<= 1;
          match c {
            b'0' | b'1' => mask_bits |= 1,
            _ => (),
          }
        }
        // What the masked bit should be overwritten to, bits outside mask
        // are just ignored, since they are always used in conjuction with
        // mask
        let mut override_bits = 0;
        for &c in mask.as_bytes() {
          override_bits <<= 1;
          if c == b'1' {
            override_bits |= 1
          }
        }
        Instruction::SetMask(SetMaskArgs {
          mask_bits,
          override_bits,
        })
      } else {
        let mut split = line.split("] = ");
        let address = split
          .next()
          .unwrap()
          .strip_prefix("mem[")
          .unwrap()
          .parse()
          .unwrap();
        let value = split.next().unwrap().parse().unwrap();
        Instruction::Write(WriteArgs { address, value })
      }
    })
    .collect()
}

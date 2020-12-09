use crate::lib::Solver;

pub struct Day8Solver;

impl Solver for Day8Solver {
  fn solve(&self, lines: Vec<String>, part_two: bool) -> String {
    let input: Vec<u8> = lines[0]
      .chars()
      .map(|c| c.to_digit(10).unwrap() as u8)
      .collect();
    if part_two {
      print_layer(&combine_layers(&parse_raw(&input, 25, 6)), 25)
    } else {
      calculate_min_zeroes(&parse_raw(&input, 25, 6)).to_string()
    }
  }
}

fn parse_raw(digits: &Vec<u8>, width: usize, height: usize) -> Vec<Vec<u8>> {
  let mut layers: Vec<Vec<u8>> = Vec::new();
  let mut digits = digits.clone();
  while digits.len() >= width * height {
    let tmp = digits.split_off(width * height);
    layers.push(digits.clone());
    digits = tmp;
  }
  layers
}

fn calculate_min_zeroes(layers: &Vec<Vec<u8>>) -> i32 {
  layers
    .iter()
    .fold(
      (std::i32::MAX, std::i32::MAX),
      |(number_of_zeroes, ones_by_twos), layer| {
        let mut zeroes = 0;
        let mut ones = 0;
        let mut twos = 0;
        for pixel in layer.iter() {
          match pixel {
            0 => zeroes += 1,
            1 => ones += 1,
            2 => twos += 1,
            _ => (),
          }
        }
        if zeroes < number_of_zeroes {
          (zeroes, ones * twos)
        } else {
          (number_of_zeroes, ones_by_twos)
        }
      },
    )
    .1
}

fn combine_layers(layers: &Vec<Vec<u8>>) -> Vec<u8> {
  let mut combined = layers.last().unwrap().clone();
  for (index, layer) in layers.iter().rev().enumerate() {
    if index == 0 {
      continue;
    }
    for (pixel_index, &pixel) in layer.iter().enumerate() {
      if pixel != 2 {
        combined[pixel_index] = pixel;
      }
    }
  }
  combined
}

fn print_layer(layer: &Vec<u8>, width: usize) -> String {
  let mut output = String::new();
  for (index, &pixel) in layer.iter().enumerate() {
    if index != 0 && index % width == 0 {
      output.push('\n');
    }
    match pixel {
      0 => output.push(' '),
      1 => output.push('O'),
      2 => output.push(' '),
      _ => panic!("Unknown pixel"),
    }
  }
  output
}

use std::collections::HashMap;

use aoc_util::{Solver, SolverOutput};

#[derive(Default)]
pub struct Day7;

impl Solver for Day7 {
  fn part_one(&self, input: &str) -> SolverOutput {
    let dir = parse_commands(input);
    SolverOutput::Num(dirs_with_size(&dir))
  }

  fn part_two(&self, input: &str) -> SolverOutput {
    let dir = parse_commands(input);
    let free_space = 70000000 - dir.size();
    let space_to_free = 30000000 - free_space;
    SolverOutput::Num(dir_to_delete(&dir, space_to_free).unwrap().size())
  }
}

fn dirs_with_size(dir: &Directory) -> i64 {
  let dir_size = dir.size();
  let mut dir_size = if dir_size <= 100000 { dir_size } else { 0 };

  for dir in dir.dirs.values() {
    dir_size += dirs_with_size(&dir);
  }

  dir_size
}

fn dir_to_delete<'a>(dir: &'a Directory<'a>, space_to_free: i64) -> Option<&'a Directory<'a>> {
  if dir.size() < space_to_free {
    return None;
  }

  let mut dirs = dir.dirs.values().collect::<Vec<_>>();
  dirs.sort_unstable_by(|l, r| l.size().cmp(&r.size()));
  for dir in dirs {
    if dir.size() >= space_to_free {
      if let Some(dir) = dir_to_delete(dir, space_to_free) {
        return Some(dir);
      } else {
        return Some(dir);
      }
    }
  }

  return Some(dir);
}

fn parse_commands<'a>(input: &'a str) -> Directory<'a> {
  let mut root_dir = Directory::default();
  let mut current_path = vec![];
  for cmd in input.split("$") {
    if cmd.trim().starts_with("cd") {
      let parts = cmd.trim().split_ascii_whitespace().collect::<Vec<_>>();
      let to = parts[1];
      if to.starts_with("/") {
        current_path = to.split("/").filter(|part| !part.is_empty()).collect();
      } else if to == ".." {
        current_path.pop();
      } else {
        current_path.push(to)
      }
    } else {
      let files_size = cmd
        .trim()
        .lines()
        .skip(1)
        .filter_map(|line| {
          let (size, _) = line.split_once(" ").unwrap();
          if size == "dir" {
            None
          } else {
            Some(size.parse::<i64>().unwrap())
          }
        })
        .sum();
      root_dir.set_files_size(&current_path, files_size);
    }
  }

  root_dir
}

#[derive(Debug, Default)]
struct Directory<'a> {
  files_size: i64,
  dirs: HashMap<&'a str, Directory<'a>>,
}

impl<'a> Directory<'a> {
  fn set_files_size(&mut self, path: &[&'a str], files_size: i64) {
    if path.is_empty() {
      self.files_size = files_size;
    } else {
      let dir = self.dirs.entry(path[0]).or_default();
      dir.set_files_size(&path[1..], files_size);
    }
  }

  fn size(&self) -> i64 {
    self
      .dirs
      .values()
      .fold(self.files_size, |total, dir| total + dir.size())
  }
}

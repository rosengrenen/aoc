#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>
#include <algorithm>

auto read_to_string(const std::string &path) -> std::string
{
  std::ifstream ifs(path);
  return std::string(std::istreambuf_iterator<char>(ifs), std::istreambuf_iterator<char>());
}

auto split(const std::string &s, std::string delim) -> std::vector<std::string>
{
  std::stringstream ss(s);
  std::string item;
  std::vector<std::string> elems;

  int start = 0;
  int len = 0;
  while (start + len < s.length())
  {
    if (s.substr(start + len, delim.length()) == delim)
    {
      elems.push_back(s.substr(start, len));
      start += len + delim.length();
      len = 0;
    }
    else
    {
      len++;
    }
  }

  elems.push_back(s.substr(start, len));

  return elems;
}

int main()
{
  auto input = read_to_string("input.txt");
  std::vector<std::pair<int, int>> matches;
  for (auto match : split(input, "\n"))
  {
    matches.push_back(std::make_pair(match[0] - 'A', match[2] - 'X'));
  }

  auto score = [](int elf, int me)
  { return (me - elf + 4) % 3 * 3 + me + 1; };
  auto selectscore = [&](int elf, int me)
  { return score(elf, (elf + me + 2) % 3); };

  int part1_sum = 0;
  for (auto match : matches)
  {
    part1_sum += score(std::get<0>(match), std::get<1>(match));
  }

  int part2_sum = 0;
  for (auto match : matches)
  {
    part2_sum += selectscore(std::get<0>(match), std::get<1>(match));
  }

  std::cout << "Part 1: " << part1_sum << '\n';
  std::cout << "Part 2: " << part2_sum << '\n';
}

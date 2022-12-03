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
  std::vector<int> invs;
  for (auto inv : split(input, "\n\n"))
  {
    int cals = 0;
    for (auto item : split(inv, "\n"))
    {
      cals += std::stoi(item);
    }

    invs.push_back(cals);
  }

  std::sort(invs.begin(), invs.end(), std::greater<int>());

  std::cout << "Part 1: " << invs[0] << '\n';
  std::cout << "Part 2: " << invs[0] + invs[1] + invs[2] << '\n';
}

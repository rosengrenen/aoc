require_relative "day01/day01"

def day_runners(day)
  case day
  when 1
    return Day01.method(:part1), Day01.method(:part2)
  else
    raise "not implemented"
  end
end

def run_day(day)
  p1, p2 = day_runners(day)
  input = File.open(sprintf("day%02d/input.txt", day)).read
  puts("Day " + day.to_s)
  puts("Part 1: " + p1.call(input))
  puts("Part 2: " + p2.call(input))
end

run_day(1)

module Day01
  def part1(input)
    left_list, right_list = self.parse(input)
    left_list = left_list.sort
    right_list = right_list.sort
    return left_list.zip(right_list).map { |left, right| (left - right).abs }.sum.to_s
  end

  def part2(input)
    left_list, right_list = parse(input)
    frequency = Hash.new(0)
    right_list.each { |n| frequency[n] += 1 }
    return left_list.map { |n| n * frequency[n] }.sum.to_s
  end

  private
  
  def parse(input)
    left_list = []
    right_list = []
    for line in input.lines do
      parts = line.split("   ")
      left_list.push(parts[0].to_i)
      right_list.push(parts[1].to_i)
    end

    return left_list, right_list
  end

  module_function :part1, :part2, :parse
end

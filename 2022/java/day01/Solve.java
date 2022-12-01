import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.Scanner;
import java.util.stream.Collectors;

public class Solve {
  public static void main(String[] args) {
    String input = readInput();
    var part1Answer = solvePart1(input);
    System.out.println("Part 1: " + part1Answer);
    var part2Answer = solvePart2(input);
    System.out.println("Part 2: " + part2Answer);
  }

  static int solvePart1(String input) {
    var invs = parseInvs(input);
    return invs.stream().reduce(0, Integer::max);
  }

  static int solvePart2(String input) {
    var invs = parseInvs(input);
    return invs.stream().sorted(Comparator.reverseOrder()).limit(3).reduce(0, Integer::sum);
  }

  static List<Integer> parseInvs(String input) {
    return Arrays.asList(input.split("\n\n")).stream()
        .map(inv -> Arrays.asList(inv.split("\n")).stream().map(line -> Integer.parseInt(line))
            .reduce(0, Integer::sum))
        .collect(Collectors.toList());
  }

  static String readInput() {
    try {
      return Files.readString(Path.of("input.txt"));
    } catch (IOException e) {
      return "";
    }
  }
}
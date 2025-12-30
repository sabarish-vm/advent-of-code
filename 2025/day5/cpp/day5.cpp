#include <algorithm>
#include <array>
#include <cassert>
#include <charconv>
#include <cstdint>
#include <cstdio>
#include <filesystem>
#include <format>
#include <fstream>
#include <functional>
#include <iostream>
#include <ranges>
#include <sstream>
#include <string>
#include <string_view>
#include <vector>

auto debug_print = [](auto&&... args) {
#ifdef DEBUG
  (std::cout << ... << std::forward<decltype(args)>(args));
#endif
};

void problem1(std::string_view const sv) {
  int count = 0;
  auto lines = sv | std::views::split('\n');
  auto numbers = std::vector<std::array<std::int64_t, 2>>{};
  for (auto const& line : lines) {
    auto first = line.begin();
    auto end = line.end();
    if (first == end) continue;
    auto splitter_ptr = std::ranges::find(line, '-');
    if (splitter_ptr != line.end()) {
      std::int64_t num1;
      std::from_chars(first, splitter_ptr, num1);
      std::int64_t num2;
      std::from_chars(splitter_ptr + 1, end, num2);
      numbers.push_back(std::array<std::int64_t, 2>{num1, num2});
    } else {
      std::int64_t num;
      std::from_chars(first, end, num);

      for (auto const [a, b] : numbers) {
        if (num <= b && num >= a) {
          count++;
          break;
        }
      };
    }
  }
  std::cout << "P1 = " << count;
  std::ranges::sort(numbers, std::less<std::int64_t>{},
                    [](auto const& elem) { return elem[0]; });
  std::int64_t prev_a = -1;
  std::int64_t prev_b = -1;
  std::int64_t p2 = 0;
  std::size_t ind = 0;
  while (ind < numbers.size()) {
    auto const a = numbers[ind][0];
    auto const b = numbers[ind][1];
    assert(b >= a);
    debug_print(std::format("\n{:<18}, {:<18} <- {:<18}, {:<18} = ", prev_a,
                            prev_b, a, b));
    if (a > prev_b && b >= a) {
      p2 += b - a + 1;
      prev_a = a;
      prev_b = b;
      debug_print(std::format("{:18}", b - a + 1));
      debug_print(" => p2 = ");

    } else if (b > prev_b) {
      p2 += b - prev_b;

      debug_print(std::format("{:18}", b - prev_b));
      debug_print(" => p2 = ");

      prev_a = prev_b;
      prev_b = b;
    } else {
      debug_print(std::format("{:18}", ""));
      debug_print(" => p2 = ");
    }
    debug_print(p2);
    ind++;
  }
  std::cout << "\nP2 = " << p2;
}

int main(const int argc, const char* argv[]) {
#ifdef example
  {
    std::string data =
        "3-5\n10-14\n12-18\n13-17\n16-19\n19-19\n19-21\n1\n5\n8\n11\n17\n32";
    problem1(data);
  }
#else
  {
    if (argc < 2) {
      std::cerr << "Argument missing" << std::endl;
      return 1;
    }
    auto filepath = std::filesystem::path(argv[1]);
    if (!std::filesystem::exists(filepath)) {
      std::cerr << "File " << filepath << " not found" << std::endl;
      return 1;
    }
    auto file = std::ifstream{filepath};
    std::stringstream buffer;
    buffer << file.rdbuf();
    std::string contents{buffer.str()};
    problem1(contents);
  }
#endif  // example
}

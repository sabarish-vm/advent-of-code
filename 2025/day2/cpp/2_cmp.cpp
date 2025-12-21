// Copyright (c) 2025 Sabarish. All Rights Reserved.
#include <algorithm>
#include <array>
#include <charconv>
#include <cstddef>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <iterator>
#include <numeric>
#include <ranges>
#include <span>
#include <string>
#include <string_view>
#include <tuple>
#include <vector>

constexpr auto FACTORS = []() constexpr -> auto {
  std::array<std::array<size_t, 6>, 15> result{};
  std::array<size_t, 15> counts{};
  for (size_t n = 2; n < 15; ++n) {
    size_t count = 0;
    for (size_t i = 1; i < n; ++i) {
      if (n % i == 0) {
        result[n][count++] = i;
      }
    }
    counts[n] = count;
  }
  return std::tuple{result, counts};
}();

constexpr auto get_factors(size_t n) {
  auto &result = std::get<0>(FACTORS);
  auto &counts = std::get<1>(FACTORS);
  auto &len = counts[n];
  return std::span{result[n]}.first(len);
}

void counter(auto const &vec, auto &count, auto &count2) {
  for (auto const &num : vec) {
    std::int64_t current = 0;
    auto s1 = std::to_string(num);
    size_t slen = s1.length();
    if (slen > 1) {
      {
        auto half_len = slen / 2;
        auto s11 = s1.substr(0, half_len);
        auto s12 = s1.substr(half_len, slen);
        if (s11 == s12) {
          count += num;
          current = num;
          continue;
        }
      }

      {
        auto factors = get_factors(slen);
        for (auto const factor : factors) {
          auto first_view = s1 | std::views::take(factor);
          auto prev = std::string_view(first_view.begin(), first_view.end());

          int i = 1;
          bool break_condition = false;
          while (i * factor < slen && !break_condition) {
            auto chunk = std::views::all(s1) | std::views::drop(i * factor) |
                         std::views::take(factor);
            auto current_view = std::string_view(chunk.begin(), chunk.end());
            // std::cout << current_view << " " << chunk.size() << "\n";
            if (!(prev == current_view)) {
              break_condition = true;
            }
            i++;
          }
          if (!break_condition) {
            if (num != current) {
              count2 += num;
              current = num;
            }
          }
        }
      }
    }
  }
}

int main(int const argc, char const *argv[]) {
  std::int64_t count_1 = 0;
  std::int64_t count_2 = 0;
  std::filesystem::path file_path{argv[1]};

  if (!std::filesystem::exists(file_path)) {
    std::cerr << "Error : " << file_path << " : file not found";
    return 1;
  }
  std::ifstream file(file_path);
  std::string line;
  std::getline(file, line);
  auto split = std::string_view(line) | std::views::split(',');
  std::int64_t v1, v2;
  for (auto elem : split) {
    auto split_2 =
        std::string_view(elem.begin(), elem.end()) | std::views::split('-');

    auto x1 = split_2.front();
    std::from_chars(x1.begin(), x1.end(), v1);

    auto x2 = *std::ranges::next(split_2.begin());
    std::from_chars(x2.begin(), x2.end(), v2);

    std::vector<std::int64_t> vec(v2 - v1 + 1);
    std::iota(vec.begin(), vec.end(), v1);
    counter(vec, count_1, count_2);
  }
  std::cout << "\nCount1 = " << count_1 << '\n';
  std::cout << "\nCount2 = " << count_2 << '\n';
  std::cout << "\nCountT = " << count_1 + count_2 << '\n';
}

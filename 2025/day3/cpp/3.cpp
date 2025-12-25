// Copyright (c) 2025 Sabarish. All Rights Reserved.
#include <array>
#include <charconv>
#include <cstddef>
#include <cstdint>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>
#include <ranges>
#include <sstream>
#include <string>
#include <string_view>

template <size_t Nsub>
auto swapper(std::array<char, Nsub>& characters, std::string_view const& sv,
             std::size_t const& check_id, std::size_t const& char_id) {
  char elem = sv.at(char_id);
  bool reset = false;
  for (size_t j = check_id; j < Nsub; j++) {
    char elem2 = characters[j];
    if (elem > elem2 && !reset) {
      characters[j] = sv.at(char_id);
      reset = true;
      continue;
    }
    if (reset) {
      characters[j] = '0';
    }
  }
}

template <size_t Nsub>
auto problem(std::string_view const& str) {
  auto lines = str | std::views::split('\n');
  std::int64_t res = 0;
  for (auto line : lines) {
    std::array<char, Nsub> characters;
    characters.fill('0');
    auto line_view = std::string_view(line.begin(), line.end());
    size_t it = 0;
    while (it < line.size()) {
      std::size_t rem = line.size() - it;
      std::size_t check_from = rem > Nsub ? 0 : Nsub - rem;
      swapper(characters, line_view, check_from, it);
      it++;
    }
    std::int64_t val;
    std::from_chars(characters.begin(), characters.end(), val);
    res += val;
  }
  std::cout << res << '\n';
}

int main(int argc, const char* argv[]) {
#ifdef example
  const std::string data =
      ("987654321111111\n811111111111119\n234234234234278\n818181911112111");
  problem1(std::string_view{data});
  problem2(std::string_view{data});
#else
  if (argc < 2) {
    std::cerr << "Path to input file missing\n";
    return 1;
  }
  std::filesystem::path input_file{argv[1]};
  if (!std::filesystem::exists(input_file)) {
    std::cerr << std::format("Error : file {}, not found", input_file.string())
              << '\n';
  }
  std::ifstream file{input_file};
  std::stringstream buffer;
  buffer << file.rdbuf();
  std::string contents{buffer.str()};
  problem<2>(std::string_view{contents});
  problem<12>(std::string_view{contents});
#endif  // example
  return 0;
}

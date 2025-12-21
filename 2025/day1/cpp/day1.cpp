// Copyright (c) 2025 Sabarish. All Rights Reserved.
#include <cassert>
#include <charconv>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <string>
#include <system_error>

struct my_type {
  // Current state of the needle
  int state;

  // Everytime the state ends in 0, this will be incremented
  int m_0End = 0;

  // Everytime the needle passes through 0, this will be incremented
  int m_0Passed = 0;

  my_type& operator+=(const int& n) {
    m_0Passed += std::abs(n) >= 100 ? (std::abs(n) / 100) : 0;
    int rem = n % 100;
    int temp = state + rem;
    assert(std::abs(rem) < 100);
    assert(std::abs(temp) < 200);
#if DEBUG
    std::cout << temp << ", ";
#endif
    if (temp >= 100) {
      int new_state = temp - 100;
      new_state == 0 ? m_0End++ : 0;
      state != 0 && new_state != 0 ? m_0Passed++ : 0;
      state = new_state;
      return *this;
    } else if (temp < 0) {
      int new_state = 100 + temp;
      new_state == 0 ? m_0End++ : 0;
      state != 0 && new_state != 0 ? m_0Passed++ : 0;
      state = new_state;
      return *this;
    } else if (temp == 0) {
      state = temp;
      m_0End++;
      return *this;
    } else {
      state = temp;
      return *this;
    }
  }
};

int main(int const argc, char const *argv[]) {
  std::ios::sync_with_stdio(false);
  auto start = my_type{50};
  std::filesystem::path file_path{argv[1]};

  if (!std::filesystem::exists(file_path)) {
    std::cerr << "Error : " << file_path << " : file not found";
    return 1;
  }
  std::ifstream file(file_path);
  std::string line;

  while (std::getline(file, line)) {
    if (line[0] == 'L') {
      int val;
      if (std::from_chars(line.data() + 1, line.data() + line.size(), val).ec ==
          std::errc{}) {
        val *= -1;
#if DEBUG
        std::cout << start.state << ", " << val << ", ";
#endif
        start += val;
      }

    } else if (line[0] == 'R') {
      int val;
      if (std::from_chars(line.data() + 1, line.data() + line.size(), val).ec ==
          std::errc{}) {
#if DEBUG
        std::cout << start.state << ", " << val << ", ";
#endif
        start += val;
      }
    }

#if DEBUG
    std::cout << start.m_0End << ", " << start.m_0Passed << "\n";
#endif
  }
  file.close();

  std::cout << "\nReached state 0 : " << start.m_0End << '\n';
  std::cout << "Passed state 0 : " << start.m_0Passed << '\n';
  std::cout << "Total state 0 :" << start.m_0Passed + start.m_0End;
  std::cout << std::endl;
  return 0;
}

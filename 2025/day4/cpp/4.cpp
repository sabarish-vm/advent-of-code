// Copyright (c) 2025 Sabarish. All Rights Reserved.
#include <algorithm>
#include <cassert>
#include <cstdint>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <iterator>
#include <memory>
#include <numeric>
#include <ranges>
#include <sstream>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

enum class Cell : char { Empty = '.', Roll = '@', Liftable = 'x' };

constexpr std::array<Cell, 256> char_to_cell = [] {
  std::array<Cell, 256> table{};
  table['.'] = Cell::Empty;
  table['@'] = Cell::Roll;
  table['x'] = Cell::Liftable;
  return table;
}();

constexpr static std::array<std::array<int, 2>, 8> directions{
    std::array{-1, 1},   // NW
    std::array{0, 1},    // N
    std::array{1, 1},    // NE
    std::array{-1, 0},   // W
    std::array{1, 0},    // E
    std::array{-1, -1},  // SW
    std::array{0, -1},   // S
    std::array{1, -1}    // SE
};
std::ostream& operator<<(std::ostream& os, Cell cell) {
  switch (cell) {
    case Cell::Empty:
      os << ".";
      break;
    case Cell::Roll:
      os << "@";
      break;
    case Cell::Liftable:
      os << "x";
      break;
  }
  return os;
}

struct Matrix {
  std::size_t rows;
  std::size_t cols;
  std::vector<Cell> data;

  explicit Matrix() : cols{0}, rows(0) {}

  Matrix(const Matrix& other)
      : rows(other.rows), cols(other.cols), data(other.data) {};

  Matrix(const size_t r, const size_t c)
      : rows{r}, cols{c}, data(std::vector<Cell>{}) {
    data.reserve(r * c);
  }

  Matrix(const size_t r, const size_t c, const Cell val)
      : rows{r}, cols{c}, data(std::vector<Cell>(r * c, val)) {}

  inline void set(const std::size_t i, const std::size_t j, const Cell val) {
    data[i * cols + j] = val;
  }
};

Matrix gen_matrix(std::string_view const sv) {
  auto lines = sv | std::views::split('\n');
  std::size_t cc;
  std::size_t rc = std::ranges::count_if(
      lines, [](auto const line) { return line.size() > 1; });
  auto it = std::ranges::find_if(
      lines, [](auto const line) { return line.size() > 1; });
  if (it != lines.end()) {
    cc = (*it).size();
  } else {
    std::abort();
  };
  Matrix matrix(rc + 2, cc + 2, Cell::Empty);
  rc = 1;
  for (auto const& line : lines) {
    cc = 1;
    if (line.size() > 1) {
      for (auto const& charac : line) {
        auto val = static_cast<Cell>(charac);
        matrix.set(rc, cc, val);
        cc++;
      }
      rc++;
    }
  }
  assert(matrix.rows > 0 && matrix.rows < 1000 && matrix.cols > 0 &&
         matrix.cols < 1000);
  return matrix;
}

void check_grid(Matrix& mat) {
  std::vector<std::uint32_t> counts_vec{};
  std::uint32_t count_roll = 0;
  assert(mat.rows > 0 && mat.rows < 1000 && mat.cols > 0 && mat.cols < 1000);
  Matrix mat2(mat);
  Matrix mat_temp(mat);
  while (true) {
    std::uint32_t counts = 0;
    for (int r = 0; r < mat.rows; r++) {
      for (int c = 0; c < mat.cols; c++) {
        auto center = mat.data[r * mat.cols + c];
        mat2.data[r*mat.cols+c] =center;
        if (!(center == Cell::Roll)) {
          continue;
        }
        count_roll = 0;
        for (auto const [a, b] : directions) {
          int const x = r + a;
          int const y = c + b;
          Cell val;
          val = mat.data[x * mat.cols + y];
          if (val == Cell::Roll) {
            count_roll++;
          }
        }  // End of directions loop
        if (count_roll < 4) {
          counts++;
          mat2.set(r, c, Cell::Liftable);
        }  //
      }  // End of cols loop
    }  // End of rows loop
    if (counts == 0) {
      break;
    }
    counts_vec.push_back(counts);
    std::swap(mat.data, mat2.data);
  }
  std::cout << "\nTotal : "
            << std::accumulate(counts_vec.begin(), counts_vec.end(), 0);
}

int main(int argc, char* argv[]) {
#ifdef example
  const std::string test_data{
      "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@"
      "\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@."};
  auto mat = gen_matrix(test_data);
  std::cout << "Rows = " << mat.rows << '\n';
  std::cout << "Cols = " << mat.cols << '\n';
  std::cout << "Element 0,0 = " << mat.at_ref_safe(0, 0) << '\n';
  std::cout << "Element 2,4 = " << mat.at_ref_safe(2, 4) << '\n';
  check_grid(mat);

#else
  if (argc < 2) {
    std::cerr << "Argument missing" << std::endl;
    return 1;
  }
  auto filepath = std::filesystem::path(argv[1]);
  if (!std::filesystem::exists(filepath)) {
    std::cerr << "File " << filepath << " not found" << std::endl;
    return 1;
  }
  auto file = std::ifstream(filepath);
  std::stringstream buffer;
  buffer << file.rdbuf();
  std::string contents{buffer.str()};
  auto mat = gen_matrix(contents);
  check_grid(mat);
#endif  // example
}

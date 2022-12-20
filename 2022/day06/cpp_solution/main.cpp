#include <fstream>
#include <iostream>

constexpr size_t WINDOW_SIZE = ('z' - 'a' + 1);

class Window {
  int data_[WINDOW_SIZE] = {0};
  int overflow_ = 0;
  int size_ = 0;

  int char_to_index(char ch) {
    if (ch < 'a' || ch > 'z')
      throw std::runtime_error(
          "trying to insert a character that isn't in the valid range");
    return ch - 'a';
  }

 public:
  void add_to_window(char item) {
    auto index = char_to_index(item);

    if (++data_[index] == 2) ++overflow_;
    ++size_;
  }

  void remove_from_window(char item) {
    auto index = char_to_index(item);
    if (--data_[index] == 1) --overflow_;
    --size_;
  }

  int get_size() const { return size_; }

  bool is_start_of_packet() const { return overflow_ == 0; }
};

std::string read_input(const std::string &input_path) {
  std::string input;
  std::ifstream input_file(input_path);
  std::getline(input_file, input);
  return input;
}

int find_marker(const std::string &input, int unique_size) {
  auto ptr = input.cbegin();
  Window win;
  for (size_t i = 0; i < unique_size && ptr < input.cend(); ++i)
    win.add_to_window(*(ptr++));
  if (win.get_size() == unique_size && win.is_start_of_packet())
    return unique_size;

  auto end = input.cend();
  for (auto begin = input.cbegin(); ptr < end; ++ptr, ++begin) {
    win.add_to_window(*ptr);
    win.remove_from_window(*begin);
    if (win.is_start_of_packet()) return ptr - input.cbegin() + 1;
  }

  return -1;
}

int main(int argc, char const *argv[]) {
  if (argc != 2) {
    std::cout << "usage: " << argv[0] << " <path-to-input>" << std::endl;
    return 1;
  }
  auto input_path = argv[1];
  auto input = read_input(input_path);

  std::cout << "part 1 result: " << find_marker(input, 4) << std::endl;
  std::cout << "part 2 result: " << find_marker(input, 14) << std::endl;

  return 0;
}

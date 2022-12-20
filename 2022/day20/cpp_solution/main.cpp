#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

std::vector<std::string> read_input(const std::string &input_path) {
  std::vector<std::string> ret;

  std::string input;
  std::ifstream input_file(input_path);
  while (std::getline(input_file, input)) ret.push_back(input);

  return ret;
}

struct Node {
  Node *prev, *next;
  int64_t val;
};

class LinkedList {
 public:
  LinkedList(const std::vector<std::string> &input, int64_t key)
      : key_(key), order_(input.size(), nullptr) {
    if (input.size() < 1) return;

    Node *head = new Node();  // place holder
    auto prev = head;
    auto temp = head;
    for (size_t i = 0; i < input.size(); ++i) {
      auto next = order_[i] = new Node;
      next->val = std::stoll(input[i], nullptr, 10);
      next->prev = prev;
      prev->next = next;

      prev = next;
    }
    // remove place holder
    prev->next = head->next;
    head->next->prev = prev;
    head_ = head = head->next;
    delete temp;
  }

  ~LinkedList() {
    for (auto node : order_) {
      delete node;
    }
  }

  void mix() {
    for (auto node : order_) {
      auto val = node->val * key_;
      int64_t counter = std::abs(val);
      if (val > 0) {
        counter %= (order_.size() - 1);
        while (counter--) push_to_right(node);
      } else {
        counter %= (order_.size() - 1);
        while (counter--) push_to_left(node);
      }
    }
  }

  int64_t sum_coordinates(std::vector<size_t> coordinates) {
    auto zero_index = get_index_by_num(0);
    auto node = head_;

    for (auto &coordinate : coordinates)
      coordinate = (coordinate + zero_index) % order_.size();

    int64_t result = 0;
    for (auto i = 0; node && i < order_.size(); ++i, node = node->next) {
      if (std::find(coordinates.begin(), coordinates.end(), i) !=
          coordinates.end()) {
        result += node->val * key_;
      }
    }

    return result;
  }

  friend std::ostream &operator<<(std::ostream &out, const LinkedList &list) {
    if (!list.head_) return out;

    out << list.head_->val;
    for (auto node = list.head_->next; node && node != list.head_;
         node = node->next)
      out << ", " << node->val * list.key_;
    out << std::endl;

    return out;
  }

 private:
  void push_to_left(Node *node) {
    auto prev = node->prev;
    auto next = node->next;

    /// format: prev.prev -> prev -> node -> next
    // update prev.prev
    prev->prev->next = node;
    node->prev = prev->prev;

    // update next
    next->prev = prev;
    prev->next = next;

    // update prev & next
    prev->prev = node;
    node->next = prev;

    if (node == head_) head_ = next;
  }

  void push_to_right(Node *node) {
    auto next = node->next;

    /// format: prev -> node -> next -> next.next
    next->next->prev = node;
    node->next = next->next;

    node->prev->next = next;
    next->prev = node->prev;

    next->next = node;
    node->prev = next;

    if (node == head_) head_ = next;
    if (node->next == head_) head_ = node;
  }

  size_t get_index_by_num(int64_t num) {
    auto node = head_;
    for (auto i = 0; node && i < order_.size(); ++i, node = node->next) {
      if (node->val == num) return i;
    }

    return 0;
  }

  Node *head_;
  std::vector<Node *> order_;
  int64_t key_;
};

size_t process_part(const std::vector<std::string> input, int64_t key,
                    size_t times) {
  LinkedList list(input, key);
  for (size_t i = 0; i < times; ++i) list.mix();
  return list.sum_coordinates({1000, 2000, 3000});
}

int main(int argc, char const *argv[]) {
  if (argc != 2) {
    std::cout << "usage: " << argv[0] << " <path-to-input>" << std::endl;
    return 1;
  }
  auto input_path = argv[1];
  auto input = read_input(input_path);
  std::cout << "part 1 result: " << process_part(input, 1, 1) << std::endl;
  std::cout << "part 2 result: " << process_part(input, 811589153, 10)
            << std::endl;

  return 0;
}
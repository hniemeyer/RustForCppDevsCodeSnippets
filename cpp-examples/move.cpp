#include <iostream>
#include <string>
#include <utility>

struct Employee {
  std::string name;
  int id;

  Employee(std::string name_, int id_) : name{name_}, id{id_} {};

  // Copy constructor
  Employee(const Employee &empl) : name{empl.name}, id{empl.id} {
    std::cout << "COPY: " << name << ", " << id << '\n';
  }

  // Copy assignment operator
  Employee &operator=(const Employee &empl) {
    std::cout << "COPY ASSIGNMENT: " << name << ", " << id << '\n';
    name = empl.name;
    id = empl.id;
    return *this;
  }

  // Move constructor
  Employee(Employee &&empl) noexcept
      : name{std::move(empl.name)}, id{std::move(empl.id)} {
    std::cout << "MOVE: " << name << ", " << id << '\n';
  }

  // Move assignment operator
  Employee &operator=(Employee &&empl) noexcept {
    std::cout << "MOVE ASSIGNMENT: " << name << ", " << id << '\n';
    name = std::move(empl.name);
    id = std::move(empl.id);
    return *this;
  }
};

int main() {
  auto hendrik = Employee("Hendrik", 101);
  const auto hendrik2 = std::move(hendrik); // move
  auto hendrik3 = hendrik2;                 // Copy
  auto hendrik4 = std::move(hendrik2);      // Ooops, cannot move from const
  return 0;
}
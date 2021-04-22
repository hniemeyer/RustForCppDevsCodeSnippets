#include <vector>
#include <iostream>

void print_shopping_list(std::vector<std::string> const &shopping_list) {
    //                                            ^^^^^ explicitly immmutable
    if (shopping_list.size() == 0) {
        std::cout << "\t-- nothing\n";
    }

    for (const auto& item: shopping_list) {
        std::cout << "\t* " << item << '\n';
    }
}

void buy_groceries(std::vector<std::string> shopping_list) {
    //             ^^^^^^^^^^^^^^^^^^^^^^^^ mutable by default
    std::cout << "Going out to buy:\n";
    print_shopping_list(shopping_list);

    shopping_list.clear();

    std::cout << "things left to buy:\n";
    print_shopping_list(shopping_list);
}

int main() {
    std::vector<std::string> shopping_list {"Pasta", "Milk", "Toilet Paper"};
    shopping_list.push_back("Chocolate");
    //^^^^^^^^^^^^^^^^^^^^^ mutable by default

    buy_groceries(shopping_list);
}
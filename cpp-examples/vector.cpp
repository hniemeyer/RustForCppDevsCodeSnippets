//Compiler Explorer: https://gcc.godbolt.org/z/d1MGa1Ecf
#include <vector>
#include <ranges>
#include <numeric>
#include <iostream>

int main() {
    std::vector<int> numbers(100);
    std::iota(numbers.begin(), numbers.end(), 0);
    auto result = numbers | std::views::filter([](int i){return i % 3 != 0;}) | std::views::transform([](int i){return i*i;});
   
       for (auto i : result) {
        std::cout << "Result:" << i <<  '\n';
    }
}
//Compiler Explorer: https://gcc.godbolt.org/z/YfPs8dv4s
#include <vector>
#include <ranges>
#include <numeric>
#include <fmt/format.h>
#include <fmt/ranges.h>

int main() {
    std::vector<int> numbers(100);
    std::iota(numbers.begin(), numbers.end(), 0);
    auto result = numbers | std::views::filter([](int i){return i % 3 != 0;}) | std::views::transform([](int i){return i*i;});
   
       for (auto i : result) {
        fmt::print("Result: {}\n", i);
    }
}
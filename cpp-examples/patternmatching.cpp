#include <variant>
#include <iostream>
#include <utility>
#include <cmath>

template <class... Ts>
struct overloaded : Ts...
{
    using Ts::operator()...;
};
template <class... Ts>
overloaded(Ts...) -> overloaded<Ts...>;

using quadratic_equation_solution = std::variant<std::monostate, double, std::pair<double, double>>;

quadratic_equation_solution solve_quadratic_equation(double a, double b, double c)
{
    const auto discriminant = b * b - 4.0 * a * c;

    if (discriminant < 0.0)
        return std::monostate{};
    else if (discriminant == 0.0)
        return -b / (2.0 * a);
    else
    {
        return std::pair{(-b + std::sqrt(discriminant)) / (2.0 * a),
                         (-b - std::sqrt(discriminant)) / (2.0 * a)};
    }
}

int main()
{
    const auto res = solve_quadratic_equation(1.0, 3.0, 1.0);

    std::visit(overloaded{
                   [](std::monostate arg) { std::cout << "No solution!" << '\n'; },
                   [](double arg) { std::cout << "One solution: " << arg << 'n'; },
                   [](std::pair<double, double> arg) { std::cout << "Two solutions: "
                                                                 << arg.first << " " << arg.second << '\n'; },
               },
               res);
}
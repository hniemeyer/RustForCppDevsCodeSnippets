enum QuadraticSolution {
    NoSolution,
    OneSolition(f64),
    TwoSolutions(f64, f64),
}

fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> QuadraticSolution {
    let discriminant = b * b - 4.0 * a * c;
    if discriminant <  0.0 { QuadraticSolution::NoSolution }
    else if discriminant == 0.0 { QuadraticSolution::OneSolition(-b / (2.0 * a))} 
    else { QuadraticSolution::TwoSolutions((-b + discriminant.sqrt()) / (2.0 * a), (-b - discriminant.sqrt()) / (2.0 * a))}
}

fn main() {
    let res = solve_quadratic_equation(1.0, 3.0, 1.0);
    match res {
        QuadraticSolution::NoSolution => println!("No solution"),
        QuadraticSolution::OneSolition(x) => println!("One solution: {}", x),
        QuadraticSolution::TwoSolutions(x,y) => println!("Two solutions: {}, {}", x,y)
    }

    let res2 = if let QuadraticSolution::NoSolution = solve_quadratic_equation(1.0, 1.0, 1.0) {
        5.0
    }
    else {
        8.0
    };

    println!("{}",res2);
}

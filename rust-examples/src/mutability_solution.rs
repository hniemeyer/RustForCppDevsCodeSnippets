fn buy_groceries(mut shopping_list: Vec<&str>) {
    //           ^^^ note that we need to declare this as mutable too
    println!("Going out to buy: {:?}", shopping_list);

    shopping_list.clear();

    println!("Things left to buy: {:?}", shopping_list);
}

/// NOTE FOR INSTRUCTORS:
/// When we've arrived at this solution, we can smoothly transition to the ownership & borrowing
/// task! -> "Say that we decide we want Chocolate *after* we've gone grocery shopping so we
/// haphazardly add another `shopping_list.push()` at the end. What happens?"
fn main() {
    let mut shopping_list: Vec<&str> = vec!["Pasta", "Milk", "Toilet Paper"];
    //  ^^^
    shopping_list.push("Chocolate");

    buy_groceries(shopping_list);
}
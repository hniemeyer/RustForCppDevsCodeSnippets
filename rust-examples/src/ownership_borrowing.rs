fn buy_groceries(shopping_list: Vec<&str>) {
    //                          ^^^^^^^^^
    //                          ownership is transferred to buy_groceries() here
    //                          because we're not passing by reference
    println!("Going out to buy: {:?}", shopping_list);

    // note: shopping_list is dropped at the end of this function
}

fn main() {
    let mut shopping_list: Vec<&str> = vec!["Pasta", "Milk", "Toilet Paper"];

    buy_groceries(shopping_list);

    shopping_list.push("Chocolate");
    // how would you resolve this use afte free error?
}
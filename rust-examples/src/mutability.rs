fn buy_groceries(shopping_list: Vec<&str>) {
    println!("Going out to buy: {:?}", shopping_list);

    shopping_list.clear();

    println!("Things left to buy: {:?}", shopping_list);
}

fn main() {
    let shopping_list: Vec<&str> = vec!["Pasta", "Milk", "Toilet Paper"];
    shopping_list.push("Chocolate");

    buy_groceries(shopping_list);
}
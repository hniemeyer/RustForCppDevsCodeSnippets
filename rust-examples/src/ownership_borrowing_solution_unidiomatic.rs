fn buy_groceries(shopping_list: &mut Vec<&str>) {
    println!("Going out to buy: {:?}", shopping_list);
    shopping_list.clear();
}

fn main() {
    let mut shopping_list: Vec<&str> = vec!["Pasta", "Milk", "Toilet Paper"];

    buy_groceries(&mut shopping_list);

    shopping_list.push("Chocolate");
    buy_groceries(&mut shopping_list);
}
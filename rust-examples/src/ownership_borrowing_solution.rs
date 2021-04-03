
// returns groceries that you couldn't find, if any
fn buy_groceries(mut shopping_list: Vec<&str>) -> Option<Vec<&str>>{

    println!("Going out to buy: {:?}", shopping_list);
    shopping_list.remove(0);

    println!("Couldn't find: {:?}", shopping_list);

    Some(shopping_list)
}

fn main() {
    let mut shopping_list: Vec<&str> = vec!["Milk", "Pasta", "Toilet Paper"];

    shopping_list = match buy_groceries(shopping_list) {
        Some(remaining_items) => remaining_items,
        None => vec![],
    };

    shopping_list.push("Chocolate");
}
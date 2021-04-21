#[derive(PartialEq)]
enum FarmAnimal {
    Worm,
    Cow,
    Bull, // <- new Moo-ing animal!
    Chicken { num_eggs: usize },
    Dog { name: String },
}

fn what_does_the_animal_say(animal: &FarmAnimal) -> Option<String> {
    match animal {
        FarmAnimal::Cow | FarmAnimal::Bull => Some("moo".to_string()),
        //              ^^^^^^^^^^^^^^^^^^ match arm can have patterns
        FarmAnimal::Chicken { num_eggs: _ } => Some("cluck, cluck!".to_string()),
        FarmAnimal::Dog { name } if name == "Lassie" => {
            Some(format!("✨🐕 ✨ {} says: woof, woof!", name))
        }
        //                            ^^^^^^^^^^^^^^^^^^ note the match guard
        FarmAnimal::Dog { name } => Some(format!("{} says: woof, woof!", name)),
        // ‼️ match conditions are avaluated in order! => write from most to least specific.
        _ => None,
    }
}

fn main() {
    println!(
        "Dog 1: {:?}",
        what_does_the_animal_say(&FarmAnimal::Dog {
            name: "Lassie".to_string()
        })
    );
    println!(
        "Dog 2: {:?}",
        what_does_the_animal_say(&FarmAnimal::Dog {
            name: "Emma".to_string()
        })
    );

    println!("Cow: {:?}", what_does_the_animal_say(&FarmAnimal::Cow));
    println!("Bull: {:?}", what_does_the_animal_say(&FarmAnimal::Bull));
}

#[derive(PartialEq)]
enum FarmAnimal {
    Worm,
    Cow,
    Chicken { num_eggs: usize },
    Dog { name: String },
}

// NOTE TO INSTRUCTORS: Can be extended to show match guards and matching on boolean expressions;
// see `pattern_matching_intro_solution_extension.rs` for example
fn what_does_the_animal_say(animal: &FarmAnimal) -> Option<String> {
    match animal {
        //    ^^^^^ if someone notes the missing `*` here we can also
        //          talk about Deref coercions
        FarmAnimal::Cow => Some("moo".to_string()),
        FarmAnimal::Chicken { num_eggs: _ } => Some("cluck, cluck!".to_string()),
        FarmAnimal::Dog { name } => Some(format!("{} says: woof, woof!", name)),
        _ => None, // This covers worms too, as they're silent.
    }
}

fn main() {
    let animal = FarmAnimal::Dog {
        name: "Lassie".to_string(),
    };
    let sound = what_does_the_animal_say(&animal);
    println!("{:?}", sound);
}

#[derive(PartialEq)]
enum FarmAnimal {
    Worm,
    Cow,
    Chicken { num_eggs: usize },
    Dog { name: String },
}

fn what_does_the_animal_say(animal: &FarmAnimal) -> Option<String> {
    if *animal == FarmAnimal::Worm {
        return None;
    } else if *animal
        == (FarmAnimal::Dog {
            name: "Lassie".to_string(),
        })
    {
        //                                      ^^^^^^^^^^^^^^^^^^^^
        //                                      TODO: use `match` statement
        //                                      to make other dogs bark
        return Some("woof! woof!".to_string());
    } else {
        // TODO: note that we've forgotten some animals here
        return None;
    }
}

fn main() {
    let animal = FarmAnimal::Dog {
        name: "Lassie".to_string(),
    };
    let sound = what_does_the_animal_say(&animal);
    println!("{:?}", sound);
}

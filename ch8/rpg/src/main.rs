use rand::{self, seq::SliceRandom, Rng};

#[derive(Debug)]
struct Dwarf {
}

#[derive(Debug)]
struct Elf {
}

#[derive(Debug)]
struct Human {
}

#[derive(Debug)]
enum Thing {
    Sword,
    Trinket,
}

trait Enchanter: std::fmt::Debug {
    fn competency(&self) -> f64;

    fn enchant(&self, thing: &mut Thing) {
        let probability_of_success = self.competency();
        // gen_bool() generates a Boolean value, where true occurs in proportion to its argument.
        // For example, a value of 0.5 returns true 50% of the time.
        let spell_is_successful = rand::thread_rng().gen_bool(probability_of_success);
    
        print!("{:?} mutters incoherently. ", self);
        if spell_is_successful {
            println!("The {:?} glows brightly.", thing);
        }
        else {
            println!("The {:?} fizzes, then turns into a worthless trinket.", thing);
            *thing = Thing::Trinket {};
        }
    }
}

impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.5 // Dwarves are poor spellcasters, and their spells regularly fail. :-D
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        0.95 // Spells cast by elves rarely fail.
    }
}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.8 // Humans are proficient at enchanting things. Mistakes are uncommon.
    }
}

impl Enchanter for String {
    fn competency(&self) -> f64 {
        0.02 // Can strings cast spells?? :-D
    }
}

fn main() {
    let mut it = Thing::Sword;

    let d = Dwarf {};
    let e = Elf {};
    let h = Human {};
    let s = String::from("foobar");

    // Although d, e, and h are different types, using the type hint &dyn Enchanter tells the compiler to
    // treat each value as a trait object. These now all have the same type.
    // We can hold members of different types within the same Vec as all these implement the Enchanter trait.
    let party: Vec<&dyn Enchanter> = vec![&d, &e, &h, &s];
    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();

    spellcaster.enchant(&mut it);
}

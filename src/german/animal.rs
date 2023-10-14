use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Animal;

pub fn to_thing<'a>(num: u32, animal: Animal) -> Thing<'a> {
    let mut result: Thing;
    match animal {
        Animal::Bird => result = Thing::new( Sex::Male, false, "Vogel", "Vögel"),
        Animal::Cat => result = Thing::new( Sex::Female, false, "Katze", "Katzen"),
        Animal::Dog => result = Thing::new( Sex::Male, false, "Hund", "Hunde"),
        Animal::Fish => result = Thing::new( Sex::Male, false, "Fisch", "Fische"),
        Animal::Horse => result = Thing::new( Sex::Male, false, "Pferd", "Pferde"),
        Animal::Rabbit => result = Thing::new( Sex::Male, false, "Kaninchen", "Kaninchen"),
        Animal::Pig => result = Thing::new( Sex::Male, false, "Schwein", "Schweine"),
        Animal::Any => result = Thing::new( Sex::Male, false, "Tier","Tiere")
    }
    result.set_num(num);
    result 
}

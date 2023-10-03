use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Animal;

pub fn to_thing<'a>(num: u32, animal: Animal) -> Thing<'a> {
    let mut result: Thing;
    match animal {
        Animal::Bird => result = Thing::new( Sex::Male, "Vogel", "Vögel"),
        Animal::Cat => result = Thing::new( Sex::Female, "Katze", "Katzen"),
        Animal::Dog => result = Thing::new( Sex::Male, "Hund", "Hunde"),
        Animal::Fish => result = Thing::new( Sex::Male, "Fisch", "Fische"),
        Animal::Horse => result = Thing::new( Sex::Male, "Pferd", "Pferde"),
        Animal::Rabbit => result = Thing::new( Sex::Male, "Kaninchen", "Kaninchen"),
        Animal::Any => result = Thing::new( Sex::Male, "Tier","Tiere")
    }
    result.set_num(num);
    result 
}

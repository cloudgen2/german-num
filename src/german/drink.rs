use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Drink;

pub fn to_thing<'a>(num: u32, drink: Drink) -> Thing<'a> {
    let mut result: Thing;
    match drink {
        Drink::Beer => result = Thing::new( Sex::Male, false, "Glas Bier", "Gläser Bier"),
        Drink::Coffee => result = Thing::new( Sex::Female, false, "Tasse Kaffee", "Tassen Kaffee"),
        Drink::Milk => result = Thing::new( Sex::Male, false, "Glas Milch", "Gläser Milch"),
        Drink::Tea => result = Thing::new( Sex::Female, false, "Tasse Tee", "Tassen Tee"),
        Drink::Water => result = Thing::new( Sex::Male, false, "Glas Wasser", "Gläser Wasser"),
        Drink::Wine => result = Thing::new( Sex::Male, false, "Glas Wein", "Gläser Wein"),
        Drink::Any => result = Thing::new( Sex::Male, false, "Glas Getränk", "Gläser Getränk")
    }
    result.set_num(num);
    result 
}

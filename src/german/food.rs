use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Food;

pub fn to_thing<'a>(num: u32, food: Food) -> Thing<'a> {
    let mut result: Thing;
    match food {
        Food::Bread => result = Thing::new( Sex::Male, false, "Stück Brot", "Stücke Brot"),
        Food::Croissant => result = Thing::new( Sex::Male, false, "Croissant", "Croissants"),
        Food::Cake => result = Thing::new( Sex::Male, false, "Stück Kuchen", "Stücke Kuchen"),
        Food::Pizza => result = Thing::new( Sex::Male, false, "Stück Pizza", "Pizzastücke"),
        Food::Rice => result = Thing::new( Sex::Female, false, "Schüssel Reis", "Schüsseln Reis"),
        Food::Soup => result = Thing::new( Sex::Male, false, "Teller Suppe", "Teller Suppe"),
        Food::Any => result = Thing::new( Sex::Male, false, "Lebensmittel","Lebensmittel")
    }
    result.set_num(num);
    result 
}

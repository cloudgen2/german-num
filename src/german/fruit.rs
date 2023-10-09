use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Fruit;

pub fn to_thing<'a>(num: u32, fruit: Fruit) -> Thing<'a> {
    let mut result: Thing;
    match fruit {
        Fruit::Apple => result = Thing::new( Sex::Male, true, "Apfel", "Äpfel"),
        Fruit::Orange => result = Thing::new( Sex::Female, true, "Orange", "Orangen"),
        Fruit::Banana => result = Thing::new( Sex::Female, false, "Banane", "Bananen"),
        Fruit::Strawberry => result = Thing::new( Sex::Female, false, "Erdbeere", "Erdbeeren"),
        Fruit::Pear => result = Thing::new( Sex::Female, false, "Birne", "Birnen" ),
        Fruit::WaterMelon => result = Thing::new( Sex::Female, false, "Wassermelone", "Wassermelonen" ),
        Fruit::Cherry => result = Thing::new( Sex::Female, false, "Kirsche", "Kirschen" ),
        Fruit::Grape => result = Thing::new( Sex::Female, false, "Traube", "Trauben" ),
        Fruit::Any => result = Thing::new( Sex::Female, false, "Frucht", "Früchte")
    }
    result.set_num(num);
    result 
}
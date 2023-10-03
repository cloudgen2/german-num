use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Fruit;

pub fn to_thing<'a>(num: u32, fruit: Fruit) -> Thing<'a> {
    let mut result: Thing;
    match fruit {
        Fruit::Apple => result = Thing::new( Sex::Male, "Apfel", "Äpfel"),
        Fruit::Orange => result = Thing::new( Sex::Female, "Orange", "Orangen"),
        Fruit::Banana => result = Thing::new( Sex::Female, "Banane", "Bananen"),
        Fruit::Strawberry => result = Thing::new( Sex::Female, "Erdbeere", "Erdbeeren"),
        Fruit::Pear => result = Thing::new( Sex::Female, "Birne", "Birnen" ),
        Fruit::WaterMelon => result = Thing::new( Sex::Female, "Wassermelone", "Wassermelonen" ),
        Fruit::Cherry => result = Thing::new( Sex::Female, "Kirsche", "Kirschen" ),
        Fruit::Any => result = Thing::new( Sex::Female, "Frucht", "Früchte")
    }
    result.set_num(num);
    result 
}
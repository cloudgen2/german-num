use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Transport;

pub fn to_thing<'a>(num: u32, transport: Transport) -> Thing<'a> {
    let mut result: Thing;
    match transport {
        Transport::Bus => result = Thing::new(Sex::Male, "Bus", "Busse"),
        Transport::Car => result = Thing::new(Sex::Male, "Auto", "Autos"),
        Transport::Any => result = Thing::new(Sex::Male, "Transportmittel", "Transporte")
    }
    result.set_num(num);
    result 
}
use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Transport;

pub fn to_thing<'a>(num: u32, transport: Transport) -> Thing<'a> {
    let mut result: Thing;
    match transport {
        Transport::Ambulance => result = Thing::new(Sex::Male, true, "Krankenwagen", "Krankenwagen"),
        Transport::Bus => result = Thing::new(Sex::Male, false, "Bus", "Busse"),
        Transport::Car => result = Thing::new(Sex::Male, true, "Auto", "Autos"),
        Transport::Taxi => result = Thing::new(Sex::Male, false, "Taxi", "Taxis"),
        Transport::Any => result = Thing::new(Sex::Male, false, "Transportmittel", "Transporte")
    }
    result.set_num(num);
    result 
}
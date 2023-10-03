extern crate regex;

pub mod number;
mod animal;
mod drink;
mod food;
mod fruit;
mod transport;

use crate::entities::Animal;
use crate::entities::Food;
use crate::entities::Fruit;
use crate::entities::Drink;
use crate::entities::Transport;
use crate::entities::Sex;
use crate::thisis::ThisIs;

use colored::*;
use regex::Regex;
use number::all_num;

pub struct Thing<'a> {
    sex: Sex,
    num: u32,
    single: &'a str,
    plural: &'a str
}

impl Thing<'_> {
    pub fn new<'a> (sex: Sex, single: &'a str, plural: &'a str) -> Thing <'a>{
        Thing {
            sex: sex,
            num: 0,
            single: single, 
            plural: plural    
        }
    }
}

impl ThisIs for Thing<'_>  {
    fn number_of(&self) -> String {
        let mut result: String;
        if self.num == 1 {
            match self.sex {
                Sex::Male => result=String::from("ein "),
                Sex::Female => result=String::from("eine ")
            }
            result.push_str(self.single);
        } else {
            result=String::from(all_num(self.num));
            result.push_str(" ");
            result.push_str(self.plural)
        }
        result
    }
    fn this_is(&self) -> String {
        let mut result:String;
        if self.num == 1 {
            result=String::from("Das ist ");
        } else {
            result=String::from("Das sind ");
        }
        result.push_str(& self.number_of());
        result.push_str(".");
        result
    }
    fn set_num(&mut self, num:u32) {
        self.num = num;
    }
}

pub fn print_update_at_sub(date: &str){
    println!("# Aktualisiert am: {}", date);
}

pub fn print_type_exit_to_exit_sub() {
    println!("Geben Sie '{}' ein, um das Programm zu beenden!", "exit".red());
    println!("");
}

pub fn fruit<'a>(num: u32, fruit: Fruit) -> Thing<'a> {
    fruit::to_thing(num, fruit)
}

pub fn drink<'a>(num: u32, drink: Drink) -> Thing<'a> {
    drink::to_thing(num, drink)
}

pub fn animal<'a>(num: u32, animal: Animal) -> Thing<'a> {
    animal::to_thing(num, animal)
}

pub fn food<'a>(num: u32, food: Food) -> Thing<'a> {
    food::to_thing(num, food)
}

pub fn transport<'a>(num: u32, transport: Transport) -> Thing<'a> {
    transport::to_thing(num, transport)
}

pub fn print_level(level: u32) {
    if level == 1 {
        println!("== Erste Stufe L1 ==");
    } else {
        println!("== Ebene {} L{} ==", level, level);
    }
}

pub fn print_what_is_it(question: u32, quantity: u32, icon: &String, highlight: &str) {
    print!("{}{} {}{}{} ",
        highlight.red(), 
        format!("{})",question).blue().bold(), 
        "Was ist das? ( ".yellow(),
        format!("{}{}", quantity, icon),
        " )?".yellow());
}

pub fn print_what_num(question: u32, quantity: u32, highlight: &str) {
    print!("{}{} {}{}{} ",
        highlight.red(), 
        format!("{})",question).blue().bold(), 
        format!("Was ist die Zahl ").yellow(), 
        quantity, 
        format!(" auf Deutsch?").yellow());
}

pub fn print_correct() {
    println!("» {}","Richtig!".dimmed());
}

pub fn print_correct_ans(ans: &String) {
    println!("» {}{}{}","Die richtige Antwort ist '".dimmed(), ans.italic(),"'.".dimmed());
}

pub fn process_ans(input:&str) -> String{
    let re = Regex::new(r"\s*[\.,/!:;\?\s]+\s*").unwrap();
    let ans01=re.replace_all(input.trim()," ");
    ans01.trim().to_lowercase().to_string()
}
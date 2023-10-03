pub fn below100(number: u32) -> String {
    let numbers = vec!["null","ein","zwei","drei","vier","fünf","sechs","sieben","acht","neun","zehn","elf","zwölf","dreizehn","vierzehn","fünfzehn","sechzehn","siebzehn","achtzehn","neunzehn","zwanzig"];
    let mut result: String;
    let diff: u32;
    if number == 1 {
        result = String::from("eins");
    } else if number < 21 {
        result=String::from(numbers[number as usize]);
    } else if number < 30 {
        diff = number - 20;
        result=String::from(numbers[diff as usize]);
        result.push_str("und");
        result.push_str("zwanzig");
    } else if number == 30 {
        result=String::from("dreißig");
    } else if number < 40 {
        diff = number - 30;
        result=String::from(numbers[diff as usize]);
        result.push_str("und");
        result.push_str("dreißig");
    } else if number == 40 {
        result=String::from("vierzig");
    } else if number < 50 {
        diff = number - 40;
        result=String::from(numbers[diff as usize]);
        result.push_str("und");
        result.push_str("vierzig");
    } else if number == 50 {
        result=String::from("fünfzig");
    } else if number < 60 {
        diff = number - 50;
        result=String::from(numbers[diff as usize]);
        result.push_str("und");
        result.push_str("fünfzig");
    } else if number == 60 {
        result=String::from("sechzig");
    } else if number < 70 {
        diff = number - 60;
        result=String::from(numbers[diff as usize]);
        result.push_str("und");
        result.push_str("sechzig");
    } else if number == 70 {
        result=String::from("siebzig");
    } else if number < 80 {
        diff = number - 70;
        result=String::from(numbers[diff as usize]);
        result.push_str("und");
        result.push_str("siebzig");
    } else if number == 80 {
        result=String::from("achtzig");
    } else if number < 90 {
        diff = number - 80;
        result=String::from(numbers[diff as usize]);
        result.push_str("und");
        result.push_str("achtzig");
    } else if number == 90 {
        result=String::from("neunzig");
    } else if number < 100 {
        diff = number - 90;
        result=String::from(numbers[diff as usize]);
        result.push_str("und");
        result.push_str("neunzig");
    } else {
        result = String::new();
    }
    result
}
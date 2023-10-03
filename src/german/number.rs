mod below1000;
use below1000::below1000;

pub fn all_num(number: u32) -> String {
    let mut result=String::new();
    let diff: u32;
    if number<1000 {
        result.push_str(&below1000( number));
    } else if number == 1000 {
        result.push_str("eintausend");
    } else if number < 2000 {
        result.push_str("eintausend");
        diff = number - 1000;
        result.push_str(&below1000(diff));
    } else if number == 2000 {
        result.push_str("zweitausend");
    } else if number < 3000 {
        result.push_str("zweitausend");
        diff = number - 2000;
        result.push_str(&below1000(diff));
    } else if number == 3000 {  
        result.push_str("dreitausend");
    } else if number < 4000 {
        result.push_str("dreitausend");
        diff = number - 3000;
        result.push_str(&below1000(diff));
    } else if number == 4000 {
        result.push_str("viertausend");
    } else if number < 5000 {
        result.push_str("viertausend");
        diff = number - 4000;
        result.push_str(&below1000(diff));
    } else if number == 5000 {
        result.push_str("fünftausend");
    } else if number < 6000 {
        result.push_str("fünftausend");
        diff = number - 5000; 
        result.push_str(&below1000(diff));
    } else if number == 6000 {
        result.push_str("sechstausend");
    } else if number < 7000 {
        result.push_str("sechstausend");
        diff = number - 6000;
        result.push_str(&below1000(diff));
    } else if number == 7000 {
        result.push_str("siebentausend");
    } else if number < 8000 {
        result.push_str("siebentausend");
        diff = number - 7000;
        result.push_str(&below1000(diff));
    } else if number == 8000 {
        result.push_str("achttausend");
    } else if number < 9000 {
        result.push_str("achttausend");
        diff = number - 8000;
        result.push_str(&below1000(diff));
    } else if number == 9000 {
        result.push_str("neuntausend");
    } else if number < 10000 {
        result.push_str("neuntausend");
        diff = number - 9000;
        result.push_str(&below1000(diff));
    }
    result
}
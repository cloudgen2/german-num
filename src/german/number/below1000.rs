mod below100;
use below100::below100;

pub fn below1000(number: u32) -> String {
    let mut result=String::new();
    let diff: u32;
    if number < 100 {
        result.push_str(&below100(number));
    } else if number == 100 {
        result.push_str("einhundert");
    } else if number < 200 {
        result.push_str("einhundert");
        diff = number - 100;
        result.push_str(&below100(diff));
    } else if number == 200 {
        result.push_str("zweihundert");
    } else if number < 300 {
        result.push_str("zweihundert");
        diff = number - 200;
        result.push_str(&below100(diff));
    } else if number == 300 {
        result.push_str("dreihundert");
    } else if number < 400 {
        result.push_str("dreihundert");
        diff = number - 300;
        result.push_str(&below100(diff));
    } else if number == 400 {
        result.push_str("vierhundert");
    } else if number < 500 {
        result.push_str("vierhundert");
        diff = number - 400;
        result.push_str(&below100(diff));
    } else if number == 500 {
        result.push_str("fünfhundert");
    } else if number < 600 {
        result.push_str("fünfhundert");
        diff = number - 500;
        result.push_str(&below100(diff));
    } else if number == 600 {
        result.push_str("sechshundert");
    } else if number < 700 {
        result.push_str("sechshundert");
        diff = number - 600;
        result.push_str(&below100(diff));
    } else if number == 700 {
        result.push_str("siebenhundert");
    } else if number < 800 {
        result.push_str("siebenhundert");
        diff = number - 700;
        result.push_str(&below100(diff));
    } else if number == 800 {
        result.push_str("achthundert");
    } else if number < 900 {
        result.push_str("achthundert");
        diff = number - 800;
        result.push_str(&below100(diff));
    } else if number == 900 {
        result.push_str("neunhundert");
    } else if number < 1000 {
        result.push_str("neunhundert");
        diff = number - 900;
        result.push_str(&below100(diff));
    }
    result
}

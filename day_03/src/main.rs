use std::fs;

#[derive(PartialEq)]
enum ParsingStage {
    Start,
    M,
    Mu,
    Mul,
    MulParenthesis,
    Number1,
    Comma,
    Number2,
    Finished,
}

#[derive(PartialEq)]
enum EnablingStage {
    Start,
    D,
    Do,
    DoParenthesis,
    Finished,
}

#[derive(PartialEq)]
enum DisablingStage {
    Start,
    D,
    Do,
    Don,
    DonPlica,
    DonPlicaT,
    DonPlicaTParenthesis,
    Finished,
}

fn main() {
    let data = fs::read_to_string("day_03/src/input.txt").unwrap();

    let mut parsing_stage = ParsingStage::Start;
    let mut enabling_stage = EnablingStage::Start;
    let mut disabling_stage = DisablingStage::Start;

    let mut number_1: String = String::new();
    let mut number_2: String = String::new();

    let mut enabled = true;
    let mut total_multiplications = 0;

    for char in data.chars() {

        enabling_stage = match char {
            'd' => EnablingStage::D,
            'o' if enabling_stage == EnablingStage::D => EnablingStage::Do,
            '(' if enabling_stage == EnablingStage::Do => EnablingStage::DoParenthesis,
            ')' if enabling_stage == EnablingStage::DoParenthesis => EnablingStage::Finished,
            _ => EnablingStage::Start,
        };

        if enabling_stage == EnablingStage::Finished {
            enabled = true;
        }

        disabling_stage = match char {
            'd' => DisablingStage::D,
            'o' if disabling_stage == DisablingStage::D => DisablingStage::Do,
            'n' if disabling_stage == DisablingStage::Do => DisablingStage::Don,
            '\'' if disabling_stage == DisablingStage::Don => DisablingStage::DonPlica,
            't' if disabling_stage == DisablingStage::DonPlica => DisablingStage::DonPlicaT,
            '(' if disabling_stage == DisablingStage::DonPlicaT => DisablingStage::DonPlicaTParenthesis,
            ')' if disabling_stage == DisablingStage::DonPlicaTParenthesis => DisablingStage::Finished,
            _ => DisablingStage::Start,
        };

        if disabling_stage == DisablingStage::Finished {
            enabled = false;
        }

        if enabled {
            parsing_stage = match char {
                'm' => ParsingStage::M,
                'u' if parsing_stage == ParsingStage::M => ParsingStage::Mu,
                'l' if parsing_stage == ParsingStage::Mu => ParsingStage::Mul,
                '(' if parsing_stage == ParsingStage::Mul => ParsingStage::MulParenthesis,
                a if a.is_numeric() && parsing_stage == ParsingStage::MulParenthesis => ParsingStage::Number1,
                a if a.is_numeric() && parsing_stage == ParsingStage::Number1 && number_1.chars().count() <= 3 => ParsingStage::Number1,
                ',' if parsing_stage == ParsingStage::Number1 => ParsingStage::Comma,
                a if a.is_numeric() && parsing_stage == ParsingStage::Comma => ParsingStage::Number2,
                a if a.is_numeric() && parsing_stage == ParsingStage::Number2 && number_2.chars().count() <= 3 => ParsingStage::Number2,
                ')' if parsing_stage == ParsingStage::Number2 => ParsingStage::Finished,
                _ => ParsingStage::Start,
            };
        } else {
            parsing_stage = ParsingStage::Start;
        }

        if parsing_stage == ParsingStage::Start {
            number_1 = String::new();
            number_2 = String::new();
        }

        if parsing_stage == ParsingStage::Number1 {
            number_1.push(char);
        }

        if parsing_stage == ParsingStage::Number2 {
            number_2.push(char);
        }

        if parsing_stage == ParsingStage::Finished {
            let n1 = number_1.parse::<u32>().unwrap();
            let n2 = number_2.parse::<u32>().unwrap();
            total_multiplications += n1 * n2;
            number_1 = String::new();
            number_2 = String::new();
        }
    }

    println!("Total multiplications: {}", total_multiplications);
}

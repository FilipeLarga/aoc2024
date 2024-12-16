struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

static OPERATORS: [fn(u64, u64) -> u64; 3] = [
    |a: u64, b: u64| -> u64 { a + b },
    |a: u64, b: u64| -> u64 { a * b },
    |a: u64, b: u64| -> u64 {format!("{}{}", a, b).parse::<u64>().unwrap()}
];

impl Equation {
    fn new() -> Equation {
        Equation {
            result: 0,
            numbers: Vec::new(),
        }
    }

    fn from(line: &str) -> Result<Equation, Box<dyn std::error::Error>> {
        let mut equation = Equation::new();

        if !line.contains(":") {
            return Err("Invalid equation".into());
        }

        let result: u64 = line.split(":").collect::<Vec<&str>>()[0]
            .parse::<u64>()
            .unwrap();

        let numbers: Vec<u64> = line.split(":").collect::<Vec<&str>>()[1][1..]
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        equation.result = result;
        equation.numbers = numbers;

        Ok(equation)
    }

    fn validate(&self) -> Result<u64, Box<dyn std::error::Error>> {
        match Equation::eval(self.result, self.numbers[0], self.numbers[1..].to_vec())? {
            true => Ok(self.result),
            false => Ok(0),
        }
    }

    fn eval(expected_result: u64, acc: u64, numbers: Vec<u64>) -> Result<bool, Box<dyn std::error::Error>> {
        if numbers.is_empty() {
            return Ok(expected_result == acc);
        }

        Ok(
            OPERATORS.iter().any(|op| {
                Equation::eval(
                    expected_result, op(acc, numbers[0]), numbers[1..].to_vec()
                ).unwrap()
            })
        )
    }
}

fn main() {
    let data = std::fs::read_to_string("day_7/src/input.txt").unwrap();

    let equations: Vec<Equation> = data
        .lines()
        .map(|line| Equation::from(line).unwrap())
        .collect();

    let result: u64 = equations
        .iter()
        .map(|eq| Equation::validate(eq).unwrap())
        .sum();

    println!("Result: {:?}", result);
}
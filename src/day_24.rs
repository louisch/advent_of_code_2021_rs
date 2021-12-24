use regex::Regex;
use std::collections::HashMap;


pub fn part_1(lines: &Vec<String>) {
    for i in 0..100_000_000_000_000u64 {
        parse(lines);
    }
    println!("Parsed all");
}

pub fn part_2(lines: &Vec<String>) {
}


#[derive(Debug)]
enum Operand {
    Constant(i64),
    Var(String),
    Input(i64),
}

#[derive(Debug)]
enum Instruction {
    Just(Operand),
    Inp(Operand, Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

fn match_binary_operands(captures: &regex::Captures) -> Option<(Operand, Operand)> {
    let var_regex = Regex::new(r"^[wxyz]$").unwrap();
    let a_cap = captures.get(2)?;
    let b_cap = captures.get(3)?;
    let a = a_cap.as_str();
    let b = b_cap.as_str();
    if var_regex.is_match(b_cap.as_str()) {
        Some((Operand::Var(a.to_string()), Operand::Var(b.to_string())))
    } else {
        let b_as_constant = b.parse::<i64>().ok()?;
        Some((Operand::Var(a.to_string()), Operand::Constant(b_as_constant)))
    }
}

fn parse_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    let inp_regex = Regex::new(r"^inp (\w)$").unwrap();
    let binary_op_regex = Regex::new(r"^\(\w+\) ([wxyz]) ([wxyz0-9\-]+)$").unwrap();

    let mut nth_input = 0;
    lines.iter().filter_map(|line| {
        if line.trim().is_empty() {
            return None;
        }
        if let Some(captures) = inp_regex.captures(line) {
            let var_name = captures.get(1)?;
            let result = Some(Instruction::Inp(Operand::Var(var_name.as_str().to_string()), Operand::Input(nth_input)));
            nth_input += 1;
            result
        } else if let Some(captures) = binary_op_regex.captures(line) {
            let op = captures.get(1)?;
            let (a, b) = match_binary_operands(&captures)?;
            match op.as_str() {
                "add" => Some(Instruction::Add(a, b)),
                "mul" => Some(Instruction::Mul(a, b)),
                "div" => Some(Instruction::Div(a, b)),
                "mod" => Some(Instruction::Mod(a, b)),
                "eql" => Some(Instruction::Eql(a, b)),
                other => panic!("Binary Operator not recognised: {} in line: {}", other, line),
            }
        } else {
            panic!("Line cannot be parsed: {}", line);
        }
    }).collect()
}

fn simplify_instructions(instructions: &Vec<Instruction>) {
    let mut variables = HashMap::from([
        ("w", Instruction::Just(Operand::Constant(0))),
        ("x", Instruction::Just(Operand::Constant(0))),
        ("y", Instruction::Just(Operand::Constant(0))),
        ("z", Instruction::Just(Operand::Constant(0))),
    ]);
    let mut temps = vec![];
    for instruction in instructions {
        match instruction {
            Instruction::Inp(Operand::Var(var_name), nth_input) => {
                if !variables.contains_key(var_name.as_str()) {
                    panic!("Inputting into unknown variable {}!", var_name);
                }
                variables.insert(var_name, Instruction::Just(*nth_input));
            },
            Instruction::Add(Operand::Var(var_name1), arg2) => {
                match arg2 {
                }
            },
            Instruction::Mul(arg1, arg2) => {
            },
            Instruction::Div(arg1, arg2) => {
            },
            Instruction::Mod(arg1, arg2) => {
            },
            Instruction::Eql(arg1, arg2) => {
            },
            _ => {
                panic!("Instruction invalid: {}", instruction);
            }
        }
    }
}

fn perform_add(current: &Instruction, arg2: &Operand, var_lookup: &HashMap<&str, Instruction>) -> Instruction {
    match arg2 {
        Operand::Constant(value2) => {
            match current {
                Instruction::Just(Operand::Constant(value1)) => Instruction::Just(Operand::Constant(value1 + value2)),
                Instruction::Just(Operand::Var(var_name1)) => {
                    if let Some(inst1) = var_lookup.get(var_name1.as_str()) {
                    }
                },
                Instruction::Just(Operand::Input(nth_input)) => {
                },
                Instruction::Add(arg0, arg1) => {
                    let result = perform_add(&Instruction::Just(arg1), arg2, var_lookup);
                    Instruction::Add(*arg0, result)
                },
            }
        },
        Operand::Var(var_name2) => {
            if let Some(inst2) = var_lookup.get(var_name2.as_str()) {
            } else {
                panic!("Cannot find var {} in add instruction!", var_name2);
            }
        },
        _ => {
            panic!("Input Operands can only appear in inp instructions! {}", current);
        }
    }
}

fn parse(lines: &Vec<String>) -> String {
    let inp_regex = Regex::new(r"^inp (\w)$").unwrap();
    let add_regex = Regex::new(r"^add (\w) ([wxyz0-9\-]+)$").unwrap();
    let mul_regex = Regex::new(r"^mul (\w) ([wxyz0-9\-]+)$").unwrap();
    let div_regex = Regex::new(r"^div (\w) ([wxyz0-9\-]+)$").unwrap();
    let mod_regex = Regex::new(r"^mod (\w) ([wxyz0-9\-]+)$").unwrap();
    let eql_regex = Regex::new(r"^eql (\w) ([wxyz0-9\-]+)$").unwrap();
    let mut w = "0".to_string();
    let mut x = "0".to_string();
    let mut y = "0".to_string();
    let mut z = "0".to_string();
    let mut vars = vec!["w", "x", "y", "z"];

    let mut nth_input = 0;
    let max_lines = 1000;
    for (i, line) in lines.iter().enumerate() {
        if i > max_lines {
            break;
        }
        if line.trim().is_empty() {
            continue;
        }
        if let Some(captures) = inp_regex.captures(line) {
            let input_var = format!("i{}", nth_input);
            match captures.get(1).unwrap().as_str() {
                "w" => { w = input_var; },
                "x" => { x = input_var; },
                "y" => { y = input_var; },
                "z" => { z = input_var; },
                s => { panic!("Unknown inp {} specified on line {}", s, line) },
            }
            nth_input += 1;
        } else if let Some(captures) = add_regex.captures(line) {
            let a = captures.get(1).unwrap().as_str();
            let b = match captures.get(2).unwrap().as_str() {
                "w" => w.clone(),
                "x" => x.clone(),
                "y" => y.clone(),
                "z" => z.clone(),
                s => s.to_owned(),
            };
            match a {
                "w" => { w = calc_arith(&w, "+", &b); },
                "x" => { x = calc_arith(&x, "+", &b); },
                "y" => { y = calc_arith(&y, "+", &b); },
                "z" => { z = calc_arith(&z, "+", &b); },
                s => { panic!("Unknown add {} {} specified on line {}", s, b, line) },
            }
        } else if let Some(captures) = mul_regex.captures(line) {
            let a = captures.get(1).unwrap().as_str();
            let b = match captures.get(2).unwrap().as_str() {
                "w" => w.clone(),
                "x" => x.clone(),
                "y" => y.clone(),
                "z" => z.clone(),
                s => s.to_owned(),
            };
            match a {
                "w" => { w = calc_arith(&w, "*", &b); },
                "x" => { x = calc_arith(&x, "*", &b); },
                "y" => { y = calc_arith(&y, "*", &b); },
                "z" => { z = calc_arith(&z, "*", &b); },
                s => { panic!("Unknown mul {} {} specified on line {}", s, b, line) },
            }
        } else if let Some(captures) = div_regex.captures(line) {
            let a = captures.get(1).unwrap().as_str();
            let b = match captures.get(2).unwrap().as_str() {
                "w" => w.clone(),
                "x" => x.clone(),
                "y" => y.clone(),
                "z" => z.clone(),
                s => s.to_owned(),
            };
            match a {
                "w" => { w = calc_arith(&w, "/", &b); },
                "x" => { x = calc_arith(&x, "/", &b); },
                "y" => { y = calc_arith(&y, "/", &b); },
                "z" => { z = calc_arith(&z, "/", &b); },
                s => { panic!("Unknown div {} {} specified on line {}", s, b, line) },
            }
        } else if let Some(captures) = mod_regex.captures(line) {
            let a = captures.get(1).unwrap().as_str();
            let b = match captures.get(2).unwrap().as_str() {
                "w" => w.clone(),
                "x" => x.clone(),
                "y" => y.clone(),
                "z" => z.clone(),
                s => s.to_owned(),
            };
            match a {
                "w" => { w = calc_arith(&w, "%", &b); },
                "x" => { x = calc_arith(&x, "%", &b); },
                "y" => { y = calc_arith(&y, "%", &b); },
                "z" => { z = calc_arith(&z, "%", &b); },
                s => { panic!("Unknown mod {} {} specified on line {}", s, b, line) },
            }
        } else if let Some(captures) = eql_regex.captures(line) {
            let a = captures.get(1).unwrap().as_str();
            let b = match captures.get(2).unwrap().as_str() {
                "w" => w.clone(),
                "x" => x.clone(),
                "y" => y.clone(),
                "z" => z.clone(),
                s => s.to_owned(),
            };
            match a {
                "w" => { w = calc_arith(&w, "=", &b); },
                "x" => { x = calc_arith(&x, "=", &b); },
                "y" => { y = calc_arith(&y, "=", &b); },
                "z" => { z = calc_arith(&z, "=", &b); },
                s => { panic!("Unknown eql {} {} specified on line {}", s, b, line) },
            }
        } else {
            panic!("Line didn't match any patterns: {}", line);
        }
    }
    format!("y {}", y)
    //format!("w: {}\nx: {}\ny: {}\nz {}", w, x, y, z)
}

fn calc_arith(arg1: &str, op: &str, arg2: &str) -> String {
    if let Some(answer) = try_calc_arith(arg1, op, arg2) {
        answer
    } else {
        format!("({}) {} ({})", arg1, op, arg2)
    }
}

fn try_calc_arith(arg1: &str, op: &str, arg2: &str) -> Option<String> {
    let digits_regex = Regex::new(r"^(-?\d+)$").unwrap();

    if op == "*" && (arg1 == "0" || arg2 == "0") {
        return Some("0".to_string());
    }
    if op == "+" && arg1 == "0" {
        return Some(arg2.to_string());
    }
    if op == "+" && arg2 == "0" {
        return Some(arg1.to_string());
    }
    if op == "/" && arg2 == "0" {
        panic!("Attempting to divide {} by {}!", arg1, arg2);
    }
    if op == "/" && arg2 == "1" {
        return Some(arg1.to_string());
    }
    if op == "/" && arg1 == "0" {
        return Some("0".to_string());
    }
    if op == "%" && (arg1.chars().nth(0) == Some('-') || arg2 == "0" || arg2.chars().nth(0) == Some('-')) {
        panic!("Attempting to mod {} by {}!", arg1, arg2);
    }

    let arg1_cap = digits_regex.captures(arg1).map(|c| c.get(1)).flatten()?;
    let arg2_cap = digits_regex.captures(arg2).map(|c| c.get(1)).flatten()?;
    let arg1_as_digits = arg1_cap.as_str().parse::<i64>().ok()?;
    let arg2_as_digits = arg2_cap.as_str().parse::<i64>().ok()?;
    match op {
        "+" => Some((arg1_as_digits + arg2_as_digits).to_string()),
        "*" => Some((arg1_as_digits * arg2_as_digits).to_string()),
        "%" => Some((arg1_as_digits % arg2_as_digits).to_string()),
        "=" => Some(if arg1_as_digits == arg2_as_digits { "1".to_string() } else { "0".to_string() }),
        _ => None,
    }
}

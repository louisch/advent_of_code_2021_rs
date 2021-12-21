pub fn part_1(lines: &Vec<String>) {
    println!("Magnitude of Sum: {}", magnitude_of_sum(lines));
}

pub fn part_2(lines: &Vec<String>) {
    println!("Largest magnitude from any two of above: {}", largest_magnitude_of_any_two(lines));
}

fn magnitude_of_sum(lines: &Vec<String>) -> i64 {
    let sum = lines.iter().filter(|line| !line.trim().is_empty()).map(|line| {
        parse(line)
    }).reduce(|a, b| {
        add(&a, &b)
    }).unwrap();
    magnitude(&sum)
}

fn largest_magnitude_of_any_two(lines: &Vec<String>) -> i64 {
    let parsed_lines = lines.iter().filter(|line| !line.trim().is_empty()).map(|line| {
        parse(line)
    }).collect::<Vec<Vec<Component>>>();
    let mut largest_magnitude = 0;

    for i in 0..parsed_lines.len() {
        for j in 0..parsed_lines.len() {
            if i != j {
                let mag = magnitude(&add(&parsed_lines[i], &parsed_lines[j]));
                if mag > largest_magnitude {
                    largest_magnitude = mag;
                }
            }
        }
    }

    largest_magnitude
}


#[derive(Clone, PartialEq)]
enum Component {
    Open,
    Close,
    Separator,
    Number(i64),
}

impl std::fmt::Debug for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{}", match self {
            Component::Open => "[".to_string(),
            Component::Close => "]".to_string(),
            Component::Separator => ",".to_string(),
            Component::Number(v) => v.to_string(),
        }))
    }
}

//fn number_to_str(number: &Vec<Component>) -> String {
//    number.iter().map(|component| {
//        format!("{:?}", component)
//    }).collect::<String>()
//}

#[derive(Debug, PartialEq)]
struct ExplodingPair {
    left_value: Option<(usize, i64)>,
    right_value: Option<(usize, i64)>,
    pair: (i64, i64),
    index: usize,
}

impl ExplodingPair {
    fn new(left_value: Option<(usize, i64)>, right_value: Option<(usize, i64)>, pair: (i64, i64), index: usize) -> Self {
        Self { left_value: left_value, right_value: right_value, pair: pair, index: index }
    }
}

#[derive(Debug)]
struct ExplodingPairBuilder {
    left_value: Option<(usize, i64)>,
    right_value: Option<(usize, i64)>,
    pair: Option<(Option<i64>, Option<i64>)>,
    index: Option<usize>,
    number_window: Vec<(usize, i64)>,
}

impl ExplodingPairBuilder {
    fn build(&self) -> Option<ExplodingPair> {
        let (pair_left_opt, pair_right_opt) = self.pair?;
        let pair_left = pair_left_opt?;
        let pair_right = pair_right_opt?;
        let index = self.index?;
        Some(ExplodingPair::new(self.left_value, self.right_value, (pair_left, pair_right), index))
    }
}

impl ExplodingPairBuilder {
    fn new() -> Self {
        Self { left_value: None, right_value: None, pair: None, index: None, number_window: Vec::new() }
    }
}


fn parse(snailfish_number: &str) -> Vec<Component> {
    let digits = "0123456789";
    let mut number_builder = String::new();
    let mut new_number: Vec<Component> = Vec::new();

    for c in snailfish_number.chars() {
        if digits.contains(c) {
            number_builder.push(c);
        } else {
            if !number_builder.is_empty() {
                let number = number_builder.parse::<i64>().unwrap(); // panic here if number can't be parsed
                new_number.push(Component::Number(number));

                number_builder = String::new();
            }

            let component = match c {
                '[' => Component::Open,
                ']' => Component::Close,
                ',' => Component::Separator,
                _ => panic!("Unknown character found! {}", c),
            };
            new_number.push(component);
        }
    }

    new_number
}

fn magnitude(snailfish_number: &Vec<Component>) -> i64 {
    if snailfish_number.len() == 1 {
        if let Some(Component::Number(value)) = snailfish_number.first() {
            return *value;
        }
    }

    let mut unclosed_brackets = 0;
    let mut found_sep = false;
    let mut left: Vec<Component> = Vec::new();
    let mut right: Vec<Component> = Vec::new();

    for component in snailfish_number[1..snailfish_number.len() - 1].into_iter() {
        if found_sep {
            right.push(component.clone());
        } else {
            match component {
                Component::Open => {
                    unclosed_brackets += 1;
                    left.push(component.clone());
                },
                Component::Separator => {
                    if unclosed_brackets == 0 {
                        found_sep = true; // don't push the found_sep into left
                    } else {
                        left.push(component.clone());
                    }
                },
                Component::Close => {
                    unclosed_brackets -= 1;
                    left.push(component.clone());
                },
                c => left.push(c.clone()),
            }
        }
    }

    let left_magnitude = 3 * magnitude(&left);
    let right_magnitude = 2 * magnitude(&right);

    left_magnitude + right_magnitude
}

fn add(number1: &Vec<Component>, number2: &Vec<Component>) -> Vec<Component> {
    let mut result = vec![Component::Open];
    result.append(&mut number1.to_vec());
    result.push(Component::Separator);
    result.append(&mut number2.to_vec());
    result.push(Component::Close);
    reduce(&result)
}

fn reduce(snailfish_number: &Vec<Component>) -> Vec<Component> {
    let mut current_number = snailfish_number.to_vec();
    while let Some(reduction) = try_explode(&current_number).or(try_split(&current_number)).or(None) {
        current_number = reduction;
    }
    current_number
}

fn try_explode(snailfish_number: &Vec<Component>) -> Option<Vec<Component>> {
    let mut nesting = 0;
    let mut builder: ExplodingPairBuilder = ExplodingPairBuilder::new();

    for (i, component) in snailfish_number.iter().enumerate() {
        match component {
            Component::Number(number) => {
                if builder.number_window.len() == 4 {
                    builder.number_window.remove(0);
                }
                builder.number_window.push((i, *number));

                match builder.pair {
                    None => {
                        // No pair started
                        builder.left_value = Some((i, *number));
                        builder.right_value = None;
                    },
                    Some((None, None)) => {
                        // Pair started
                        builder.pair = Some((Some(*number), None));
                    },
                    Some((Some(pair_left), None)) => {
                        // Pair left found
                        builder.pair = Some((Some(pair_left), Some(*number)));
                    },
                    Some((Some(_), Some(_))) => {
                        // Pair completed already
                        builder.right_value = Some((i, *number));
                        break;
                    }
                    _ => {
                        panic!("Unexpected pattern {:?}", builder);
                    }
                }
            },
            Component::Open => {
                nesting += 1;
                if nesting > 4 {
                    match builder.pair {
                        Some((Some(_), Some(_))) => {
                            // Pair found already
                        },
                        Some(_) => {
                            builder.left_value = builder.number_window.last().map(|t| t.clone());
                            builder.pair = Some((None, None));
                            builder.index = Some(i);
                        },
                        None => {
                            // Open bracket found but pair not started yet
                            builder.pair = Some((None, None));
                            builder.index = Some(i);
                        }
                    }
                }
            },
            Component::Separator => {
            },
            Component::Close => {
                nesting -= 1;
            },
        }
    }

    let exploding_pair = builder.build()?;
    let mut new_number = snailfish_number.to_vec();
    let (first_value, second_value) = exploding_pair.pair;
    if let Some((i, left_value)) = exploding_pair.left_value {
        new_number[i] = Component::Number(left_value + first_value);
    }
    if let Some((i, right_value)) = exploding_pair.right_value {
        new_number[i] = Component::Number(right_value + second_value);
    }
    // Replace the Open, first_value, Separator, second_value, and Close with 0
    new_number.splice(exploding_pair.index..exploding_pair.index + 5, [Component::Number(0)]);
    Some(new_number)
}

fn try_split(snailfish_number: &Vec<Component>) -> Option<Vec<Component>> {
    let (i, value) = snailfish_number.iter().enumerate().find_map(|(i, component)| {
        match component {
            Component::Number(value) if *value >= 10 => Some((i, *value)),
            _ => None
        }
    })?;

    let mut new_number = snailfish_number.to_vec();
    let halved = value / 2;
    let halved_rem = value % 2;
    new_number.splice(i..i+1, [Component::Open, Component::Number(halved), Component::Separator, Component::Number(halved + halved_rem), Component::Close]);
    Some(new_number)
}


#[cfg(test)]
mod tests {
    use crate::day_18::*;

    const TEST_INPUT_EXAMPLE: &str = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"#;

    fn get_test_input(s: &str) -> Vec<String> {
        s.split("\n").map(str::to_string).collect()
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("[9,8]"), vec![Component::Open, Component::Number(9), Component::Separator, Component::Number(8), Component::Close]);
    }

    #[test]
    fn test_magnitude() {
        let test_cases = [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488),
        ];
        for (input, expected) in test_cases {
            assert_eq!(magnitude(&parse(input)), expected);
        }
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&parse("[1,2]"), &parse("[[3,4],5]")), parse("[[1,2],[[3,4],5]]"));
        assert_eq!(add(&parse("[[[[4,3],4],4],[7,[[8,4],9]]]"), &parse("[1,1]")), parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
        assert_eq!(add(&parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"), &parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]")), parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"));
    }

    #[test]
    fn test_reduce() {
        assert_eq!(reduce(&parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")), parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
        assert_eq!(reduce(&parse("[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]")), parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"));
    }

    #[test]
    fn test_explode() {
        let test_cases = [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
            ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"),
        ];
        for (input, expected) in test_cases {
            assert_eq!(try_explode(&parse(input)), Some(parse(expected)));
        }
        assert_eq!(try_explode(&parse("[[[[0,7],4],[15,[0,13]]],[1,1]]")), None);
    }

    #[test]
    fn test_split() {
        let test_cases = [
            ("[[[[0,7],4],[15,[0,13]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"),
            ("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"),
            ("[[[[0,7],4],[14,[0,13]]],[1,1]]", "[[[[0,7],4],[[7,7],[0,13]]],[1,1]]"),
        ];
        for (input, expected) in test_cases {
            assert_eq!(try_split(&parse(input)), Some(parse(expected)));
        }
        assert_eq!(try_split(&parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")), None);
    }

    #[test]
    fn test_magnitude_of_sum() {
        let test_input_1: &str = r#"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
"#;
        assert_eq!(magnitude_of_sum(&get_test_input(test_input_1)), 3488);
        assert_eq!(magnitude_of_sum(&get_test_input(TEST_INPUT_EXAMPLE)), 4140);
    }

    #[test]
    fn test_largest_magnitude_of_any_two() {
        assert_eq!(largest_magnitude_of_any_two(&get_test_input(TEST_INPUT_EXAMPLE)), 3993);
    }
}

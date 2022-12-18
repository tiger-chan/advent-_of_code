use std::{fs, str::FromStr};

enum Operator {
    Plus,
    Multiply,
    Pow,
}

#[derive(Default, Clone, Copy)]
struct Condition {
    divisible: u64,
    when: usize,
    otherwise: usize,
}

struct Monkey {
    pub items: Vec<u64>,
    pub op: (Operator, u64),
    pub condition: Condition,
}

impl FromStr for Monkey {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items: Vec<u64> = Vec::new();
        let mut op = (Operator::Plus, 0);
        let mut conditions: Condition = Condition {
            ..Default::default()
        };

        let mut iter = s.split("\n").skip(1); // Skip "Monkey N:"
        while let Some(line) = &iter.next() {
            match line {
                line if line.starts_with("  Starting items") => {
                    items = line[18..]
                        .split(", ")
                        .map(|x| u64::from_str_radix(x, 10).unwrap())
                        .collect();
                }
                line if line.starts_with("  Operation") => {
                    let (oper, val) = line[23..].split_at(2);
                    match oper {
                        "+ " => op.0 = Operator::Plus,
                        "* " => op.0 = Operator::Multiply,
                        _ => op.0 = Operator::Plus,
                    }

                    if val.chars().any(char::is_alphabetic) {
                        op.0 = Operator::Pow;
                        op.1 = 2;
                    } else {
                        op.1 = u64::from_str_radix(val, 10)?;
                    }
                }
                line if line.starts_with("  Test") => {
                    conditions.divisible = u64::from_str_radix(&line[21..], 10)?;
                    let cond: [Option<&str>; 2] = [iter.next(), iter.next()];

                    for i in cond {
                        if let Some(x) = i {
                            if "    If true" == &x[0..11] {
                                conditions.when = usize::from_str_radix(&x[29..], 10)?;
                            } else {
                                conditions.otherwise = usize::from_str_radix(&x[30..], 10)?;
                            }
                        }
                    }
                }
                _ => panic!("Unexpected value in stream"),
            }
        }

        Ok(Monkey {
            condition: conditions,
            items: items,
            op: op,
        })
    }
}

//const MONKEY_ROUNDS: usize = 20;
const MONKEY_ROUNDS: usize = 10000;

fn main() {
    let data_str = fs::read_to_string("./data/day_11.txt").unwrap();

    let mut monkeys: Vec<Monkey> = data_str
        .split("\n\n")
        .map(|x| Monkey::from_str(x).unwrap())
        .collect();

    let modulo: u64 = monkeys.iter().map(|x| x.condition.divisible).product();
    let mut monkey_handles: Vec<usize> = Vec::new();
    monkey_handles.resize(monkeys.len(), 0);

    for _r in 0..MONKEY_ROUNDS {
        for i in 0..monkeys.len() {
            monkey_handles[i] += monkeys[i].items.len();
            for k in 0..monkeys[i].items.len() {
                let item = monkeys[i].items[k];
                let result = match monkeys[i].op.0 {
                    Operator::Plus => item + monkeys[i].op.1,
                    Operator::Multiply => item * monkeys[i].op.1,
                    Operator::Pow => item * item,
                };
                //result /= 3;
                let result = result % modulo;

                let condition = monkeys[i].condition.clone();
                if result % condition.divisible == 0 {
                    monkeys[condition.when].items.push(result);
                } else {
                    monkeys[condition.otherwise].items.push(result);
                }
            }
            monkeys[i].items.clear();
        }

        // println!(
        //     "\n\nAfter round {}, the monkeys are holding items with these worry levels:",
        //     r + 1
        // );
        // for i in 0..monkeys.len() {
        //     println!("Monkey {}: {:?}", i, monkeys[i].items);
        // }
    }

    monkey_handles.sort_by(|a, b| b.cmp(a));

    println!(
        "[2022][11] monkey business: {:?} = {}",
        monkey_handles,
        monkey_handles.iter().take(2).product::<usize>()
    );
}

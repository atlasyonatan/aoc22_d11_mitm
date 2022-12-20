use std::collections::VecDeque;

fn main() {
    let mut monkeys = input();
    let mut inspect = vec![0usize; monkeys.len()];
    let modulus = monkeys.iter().fold(1, |a,v|a*v.divisible_by);

    let rounds = 10_000;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let monkey = &mut monkeys[i];
                //inspect
                let item = (monkey.operation)(item);
                inspect[i] += 1;

                //less worry
                let item = item % modulus;

                //for part 1:
                // let item = item / 3;

                //test
                let throw_to = match item % monkey.divisible_by {
                    0 => monkey.if_true,
                    _ => monkey.if_false,
                };

                //throw
                monkeys[throw_to].items.push_back(item);
            }
        }
    }
    println!("{:?}", inspect);
    inspect.sort();
    let monkey_business = inspect.iter().rev().take(2).fold(1, |a, v| a * v);
    println!("{}", monkey_business);
}

struct Monkey {
    items: VecDeque<usize>,
    operation: fn(old: usize) -> usize,
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

fn input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: VecDeque::from([72, 97]),
            operation: |old| old * 13,
            divisible_by: 19,
            if_true: 5,
            if_false: 6,
        },
        Monkey {
            items: VecDeque::from([55, 70, 90, 74, 95]),
            operation: |old| old * old,
            divisible_by: 7,
            if_true: 5,
            if_false: 0,
        },
        Monkey {
            items: VecDeque::from([74, 97, 66, 57]),
            operation: |old| old + 6,
            divisible_by: 17,
            if_true: 1,
            if_false: 0,
        },
        Monkey {
            items: VecDeque::from([86, 54, 53]),
            operation: |old| old + 2,
            divisible_by: 13,
            if_true: 1,
            if_false: 2,
        },
        Monkey {
            items: VecDeque::from([50, 65, 78, 50, 62, 99]),
            operation: |old| old + 3,
            divisible_by: 11,
            if_true: 3,
            if_false: 7,
        },
        Monkey {
            items: VecDeque::from([90]),
            operation: |old| old + 4,
            divisible_by: 2,
            if_true: 4,
            if_false: 6,
        },
        Monkey {
            items: VecDeque::from([88, 92, 63, 94, 96, 82, 53, 53]),
            operation: |old| old + 8,
            divisible_by: 5,
            if_true: 4,
            if_false: 7,
        },
        Monkey {
            items: VecDeque::from([70, 60, 71, 69, 77, 70, 98]),
            operation: |old| old * 7,
            divisible_by: 3,
            if_true: 2,
            if_false: 3,
        },
    ]
}

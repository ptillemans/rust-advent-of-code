#[derive(Debug, Clone)]
enum Op {
    Square,
    Add(u64),
    Multiply(u64),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    m: u64,
    jump: [usize; 2],
    count: usize,
}

fn sim_monkeys(mut monkeys: Vec<Monkey>, rounds: usize, stage: u8) {
    let mm: u64 = monkeys.iter().map(|m| m.m).product();
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            let items = std::mem::replace(&mut monkeys[i].items, Vec::new());
            monkeys[i].count += items.len();
            for mut x in items {
                x = match monkeys[i].op {
                    Op::Square => x * x,
                    Op::Add(y) => x + y,
                    Op::Multiply(y) => x * y,
                };
                if stage == 1 {
                    x /= 3;
                } else {
                    x = x % mm; // x % (m * n) % m == x % m
                }
                let target = monkeys[i].jump[(x % monkeys[i].m == 0) as usize];
                monkeys[target].items.push(x);
            }
        }
    }

    for m in monkeys.iter() {
        println!("{m:?}");
    }

    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.count).collect();
    inspections.sort();
    inspections.reverse();
    println!("stage {stage}: {}", inspections[0] * inspections[1]);
}

const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

fn main() {
    let mut monkeys = Vec::<Monkey>::new();

    for block in INPUT.trim().split("\n\n") {
        let (id, items, operation, operand, m, j1, j0) = sscanf::sscanf!(
            block,
            "Monkey {usize}:
  Starting items: {str}
  Operation: new = old {str} {str}
  Test: divisible by {u64}
    If true: throw to monkey {usize}
    If false: throw to monkey {usize}"
        )
        .unwrap();
        monkeys.push(Monkey {
            items: items.split(", ").map(|x| x.parse().unwrap()).collect(),
            op: match (operation, operand) {
                ("*", "old") => Op::Square,
                ("+", y) => Op::Add(y.parse().unwrap()),
                ("*", y) => Op::Multiply(y.parse().unwrap()),
                _ => panic!("Invalid Operation {operation} {operand}"),
            },
            m,
            jump: [j0, j1],
            count: 0,
        });
        assert!(id != j0 && id != j1);
    }

    sim_monkeys(monkeys.clone(), 20, 1);
    sim_monkeys(monkeys, 10_000, 2);

}


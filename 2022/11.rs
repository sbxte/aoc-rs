
#[derive(Debug, Clone)]
enum Term {
    Old,
    Num(u64)
}

impl Term {
    fn calc(&self, old: u64) -> u64 {
        match self {
            Old => old,
            Num(n) => *n,
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Mul(Term, Term),
    Add(Term, Term),
}

impl Operation {
    fn calc(&self, old: u64) -> u64 {
        match self {
            Mul(l, r) => l.calc(old) * r.calc(old),
            Add(l, r) => l.calc(old) + r.calc(old),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    divisor: u64,
    next_monkey: (usize, usize), // (true, false)
    inspects: u64,
}

impl Monkey {
    fn new(items: Vec<u64>, op: Operation, divisor: u64, next_monkey: (usize, usize)) -> Self {
        Self {
            items,
            op,
            divisor,
            next_monkey,
            inspects: 0,
        }
    }
}

use std::cell::RefCell;
use Operation::*;
use Term::*;

fn day11() {
    // Hard code it kek

    // let monkeys = vec![ // test sample
    //     RefCell::new(Monkey::new(vec![79,98], Mul(Old, Num(19)), 23, (2, 3))),
    //     RefCell::new(Monkey::new(vec![54,65,75,74], Add(Old, Num(6)), 19, (2,0))),
    //     RefCell::new(Monkey::new(vec![79,60,97], Mul(Old, Old), 13, (1,3))),
    //     RefCell::new(Monkey::new(vec![74], Add(Old, Num(3)), 17, (0,1))),
    // ]; // mod 96577

    let monkeys = vec![ // real sample
        RefCell::new(Monkey::new(vec![75, 63], Mul(Old, Num(3)), 11, (7, 2))),
        RefCell::new(Monkey::new(vec![65,79,98,77,56,54,83,94], Add(Old, Num(3)), 2, (2,0))),
        RefCell::new(Monkey::new(vec![66], Add(Old, Num(5)), 5, (7,5))),
        RefCell::new(Monkey::new(vec![51,89,90], Mul(Old, Num(19)), 7, (6,4))),
        RefCell::new(Monkey::new(vec![75,94,66,90,77,82,61], Add(Old, Num(1)), 17, (6,1))),
        RefCell::new(Monkey::new(vec![53,76,59,92,95], Add(Old,Num(2)), 19, (4,3))),
        RefCell::new(Monkey::new(vec![81,61,75,89,70,92], Mul(Old,Old), 3, (0,1))),
        RefCell::new(Monkey::new(vec![81,86,62,87], Add(Old,Num(8)), 13, (3,5)))
    ]; // mod 9699690

    part1(monkeys.clone());
    part2(monkeys);
}

fn part1(monkeys: Vec<RefCell<Monkey>>) {
    for _ in 0..20 {
        for monkey in &monkeys {
            let mut monkey = monkey.borrow_mut();
            // might be point of failure
            monkey.inspects += monkey.items.len() as u64;
            for item in &monkey.items {
                let item = monkey.op.calc(*item) / 3;
                if item % monkey.divisor == 0 {
                    monkeys[monkey.next_monkey.0].borrow_mut().items.push(item);
                } else {
                    monkeys[monkey.next_monkey.1].borrow_mut().items.push(item);
                }
            }
            monkey.items.clear();
        }
    }

    let mut inspections = Vec::with_capacity(monkeys.len());
    for monkey in &monkeys {
        inspections.push(monkey.borrow().inspects);
    }
    inspections.sort();
    println!("{inspections:?}");
}

fn part2(monkeys: Vec<RefCell<Monkey>>) {
    for round in 0..10_000 {
        if round % 1000 == 0 {println!("{round}")}
        for monkey in &monkeys {
            let mut monkey = monkey.borrow_mut();
            // might be point of failure
            monkey.inspects += monkey.items.len() as u64;
            for item in &monkey.items {
                let item =  monkey.op.calc(*item) % 9699690;
                if item % monkey.divisor == 0 {
                    monkeys[monkey.next_monkey.0].borrow_mut().items.push(item);
                } else {
                    monkeys[monkey.next_monkey.1].borrow_mut().items.push(item);
                }
            }
            monkey.items.clear();
        }
    }

    let mut inspections = Vec::with_capacity(monkeys.len());
    for monkey in &monkeys {
        inspections.push(monkey.borrow().inspects);
    }
    inspections.sort();
    println!("{inspections:?}");
}

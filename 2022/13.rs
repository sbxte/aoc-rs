// UNFINISHED!

fn day13() {
    // Recursion time baby
    let input = include_str!("../test.txt");

    let mut index = 1;
    let mut left = None;
    for line in input.lines() {
        // println!("{index}");

        if line.is_empty() {
            index += 1;
            left = None;
            continue;
        }

        if left.is_none() {
            left = Some(Data::new(line));
            continue;
        }

        // Actual logik lets gooo
        if let Some(ref left) = left {
            if compare(left.clone().list(), Data::new(line).list()) {
                // println!("Valid index");
                println!("Valid index: {index}");
            }
        }
    }
}

// pub fn compare(left: Vec<Data>, right: Vec<Data>) -> bool {
//     compare_internal(left, right, true)
// }

fn compare(left: Vec<Data>, right: Vec<Data>) -> bool {
    use Data::*;
    // First stack must be lists
    let mut left = left.iter();
    let mut right = right.iter();

    loop {
        let left_next = left.next();
        let right_next = right.next();

        let left_next = match left_next {
            Some(v) => v,
            _ => return true,
        };

        let right_next = match right_next {
            Some(v) => v,
            _ => return false, // okay.. whack
        };

        println!("{:?} {:?}", left_next, right_next);
        match (left_next, right_next) {
            (Num(l), Num(r)) => {
                if l > r {
                    return false;
                }
            }
            (List(l), List(r)) => {
                if !compare(l.to_vec(), r.to_vec()) {
                    return false;
                }
            }
            (List(l), r) => {
                if !compare(l.to_vec(), vec![r.clone()]) {
                    return false;
                }
            }
            (l, List(r)) => {
                if !compare(vec![l.clone()], r.to_vec()) {
                    return false;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Data {
    Num(i32),
    List(Vec<Data>),
}

impl Data {
    pub fn new(input: &str) -> Self {
        Data::new_internal(input, 1).0
    }

    fn new_internal(input: &str, index: usize) -> (Self, usize) {
        use Data::*;
        let mut v = vec![];

        let mut end = index;
        let mut start = index;

        while end < input.len() {
            // dbg!(&input[start..end], start, end);
            if &input[end..=end] == "[" {
                let result = Data::new_internal(input, end + 1);
                v.push(result.0);
                start = result.1;
                end = result.1;
                continue;
            } else if &input[end..=end] == "," {
                if &input[(end - 1)..=(end - 1)] == "]" {
                    start += 1;
                } else {
                    v.push(Num(atoi::atoi(input[start..end].as_bytes()).unwrap()));
                    start = end + 1;
                }
            } else if &input[end..=end] == "]" {
                if let Some(n) = atoi::atoi(input[start..end].as_bytes()) {
                    v.push(Num(n));
                }
                return (List(v), end + 1);
            }

            end += 1;
        }

        (List(v), end)
    }

    fn list(self) -> Vec<Data> {
        match self {
            Data::List(v) => v,
            _ => panic!("Data is not list!"),
        }
    }
}

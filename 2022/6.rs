use std::io::Read;
use std::fs::OpenOptions;


#[derive(Debug)]
struct MyArray {
    data: [char; 4],
    ptr: usize
}

impl MyArray {
    #[inline]
    pub fn new() -> MyArray {
        MyArray { data: ['0', '0', '0', '0'], ptr: 0 }
    }

    #[inline]
    pub fn write(self: &mut MyArray, value: char) -> char {
        let read = self.data[self.ptr];
        self.data[self.ptr] = value;
        self.ptr = (self.ptr + 1) % 4;
        return read;
    }

    #[inline]
    pub fn read(self: &MyArray) -> char {
        return self.data[self.ptr];
    }

    #[inline]
    pub fn get_raw(self: &MyArray) -> &[char; 4] {
        return &self.data;
    }
}

fn conv_ascii(c: char) -> usize {
    c as usize - 97
}

fn check_duplicates(arr: &MyArray, counts: &[u8; 26]) -> bool {
    for c in arr.get_raw() {
        if counts[conv_ascii(*c)] > 1 {
            return true;
        }
    }
    false
}

fn main() {
    let mut file = OpenOptions::new()
                .read(true)
                // .open("./test.txt").unwrap();
                .open("./foo.txt").unwrap();

    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string = String::from(string.trim_end());

    // Solve here

    let mut arr = MyArray::new();
    let mut counts: [u8; 26] = [0; 26];

    let mut iter = string.chars();

    for _ in 0..4 {
        let c = iter.next().unwrap();
        counts[conv_ascii(c)] += 1;
        arr.write(c);
    }

    let duplicates = check_duplicates(&arr, &counts);
    if !duplicates {
        println!("Match: {}\n{:?}", 4, arr);
        return;
    }

    let mut i = 4;
    for s in iter {
        counts[conv_ascii(s)] += 1;
        let old = arr.write(s);
        counts[conv_ascii(old)] -= 1;

        i += 1;

        let duplicates = check_duplicates(&arr, &counts);
        if !duplicates {
            println!("Match: {}\n{:?}", i, arr);
            return;
        }
    }
}

use std::{fs, iter};

struct Memory {
    mem: Box<dyn Iterator<Item = Option<usize>>>,
}

impl From<&str> for Memory {
    fn from(s: &str) -> Self {
        let mut mem: Box<dyn Iterator<Item = Option<usize>>> = Box::new(iter::empty());
        let mut cur_id = 0;
        for (i, chr) in s.chars().enumerate() {
            let num = chr.to_digit(10).unwrap();
            if i % 2 == 0 {
                mem = Box::new(mem.chain(iter::repeat(Some(cur_id)).take(num as usize)));
                cur_id += 1;
            } else {
                mem = Box::new(mem.chain(iter::repeat(None).take(num as usize)));
            }
        }

        Self { mem }
    }
}

impl Memory {
    fn checksum(self) -> usize {
        let mut mem: Vec<_> = self.mem.collect();

        compact(&mut mem);

        mem.into_iter()
            .enumerate()
            .map(|(i, val)| i * val.unwrap_or_default())
            .sum()
    }
}

fn first_empty(mem: &[Option<usize>]) -> usize {
    for (i, val) in mem.iter().enumerate() {
        if val.is_none() {
            return i;
        }
    }

    0
}

fn last_occupied(mem: &[Option<usize>]) -> usize {
    for (i, val) in mem.iter().enumerate().rev() {
        if val.is_some() {
            return i;
        }
    }

    0
}

fn compact(mem: &mut [Option<usize>]) {
    let mut first = first_empty(mem);
    let mut last = last_occupied(mem);

    while first < last {   
        mem[first] = mem[last];
        mem[last] = None;

        first = first_empty(mem);
        last = last_occupied(mem);
    }
}

fn main() {
    let data = fs::read_to_string("input9").unwrap();
    let res = Memory::from(data.as_str()).checksum();
    assert_eq!(res, 6390180901651);
    println!("{}", res);
}

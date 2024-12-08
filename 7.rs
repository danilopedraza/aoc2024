use std::{fs, iter::successors};

fn decimal_digits(n: usize) -> usize {
    successors(Some(n), |&n| (n >= 10).then(|| n / 10)).count()
}

trait Operable {
    fn operate(&self, lhs: usize, rhs: usize) -> usize;
    fn operation_tuple(index: usize, operators: usize) -> Vec<Self>
    where
        Self: Sized;
    fn potential_tuples(tuple_size: u32) -> usize;
}

#[derive(Clone, Copy, Debug)]
enum ArithmeticOp {
    Sum,
    Product,
}

impl Operable for ArithmeticOp {
    fn operate(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Self::Sum => lhs + rhs,
            Self::Product => lhs * rhs,
        }
    }

    fn operation_tuple(index: usize, operators: usize) -> Vec<Self> {
        let mut res = vec![];
        for i in 0..operators {
            res.push(if (index >> i) & 1 == 0 {
                Self::Sum
            } else {
                Self::Product
            });
        }

        res
    }

    fn potential_tuples(tuple_size: u32) -> usize {
        1 << tuple_size
    }
}

#[derive(Clone, Copy, Debug)]
enum FancyArithmeticOp {
    Sum,
    Product,
    Concatenation,
}

impl Operable for FancyArithmeticOp {
    fn operate(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Self::Sum => lhs + rhs,
            Self::Product => lhs * rhs,
            Self::Concatenation => lhs * (10_usize.pow(decimal_digits(rhs) as u32)) + rhs,
        }
    }

    fn operation_tuple(mut index: usize, operators: usize) -> Vec<Self> {
        let mut res = vec![];

        while index > 0 && res.len() < operators {
            res.push(match index % 3 {
                0 => Self::Sum,
                1 => Self::Product,
                _ /* aka 2 */ => Self::Concatenation,
            });

            index = index / 3;
        }

        while res.len() < operators {
            res.push(Self::Sum);
        }

        res
    }

    fn potential_tuples(tuple_size: u32) -> usize {
        3_usize.pow(tuple_size)
    }
}

fn combination_result<T: Operable>(nums: &[usize], operators: &[T]) -> usize {
    let mut res = nums[0];
    for i in 0..operators.len() {
        res = operators[i].operate(res, nums[i + 1]);
    }

    res
}

fn solvable<T: Operable>((val, nums): (usize, &[usize])) -> bool {
    let tuple_size = nums.len() - 1;

    for index in 0..T::potential_tuples(tuple_size as u32) {
        if val == combination_result(nums, &T::operation_tuple(index, tuple_size)) {
            return true;
        }
    }

    false
}

fn main() {
    let data: Vec<(usize, Vec<usize>)> = fs::read_to_string("input7")
        .unwrap()
        .split('\n')
        .map(|str| {
            let pair: Vec<&str> = str.split(": ").collect();
            (
                pair[0].parse().unwrap(),
                pair[1].split(' ').map(|str| str.parse().unwrap()).collect(),
            )
        })
        .collect();

    let res: usize = data
        .iter()
        .filter(|(val, nums)| solvable::<ArithmeticOp>((*val, nums)))
        .map(|(val, _)| *val)
        .sum();

    assert_eq!(res, 5837374519342);
    println!("{res}");

    let res: usize = data
        .iter()
        .filter(|(val, nums)| solvable::<FancyArithmeticOp>((*val, nums)))
        .map(|(val, _)| *val)
        .sum();

    assert_eq!(res, 492383931650959);
    println!("{res}");
}

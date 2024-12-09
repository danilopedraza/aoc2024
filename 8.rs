use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn antinode_of((y1, x1): (usize, usize), (y2, x2): (usize, usize)) -> (isize, isize) {
    let (y1, x1) = (y1 as isize, x1 as isize);
    let (y2, x2) = (y2 as isize, x2 as isize);

    let (dy, dx) = (y2 - y1, x2 - x1);

    (y2 + dy, x2 + dx)
}

#[derive(Debug)]
struct AntennaMap {
    coordinates: HashMap<char, Vec<(usize, usize)>>,
    height: usize,
    width: usize,
}

impl From<&str> for AntennaMap {
    fn from(data: &str) -> Self {
        let height = data.split('\n').count();
        let width = data.split('\n').next().unwrap().len();
        let mut coordinates: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

        for (row, str) in data.split('\n').enumerate() {
            for (col, chr) in str.chars().enumerate() {
                if chr != '.' {
                    if let Some(vec) = coordinates.get_mut(&chr) {
                        vec.push((row, col));
                    } else {
                        coordinates.insert(chr, vec![(row, col)]);
                    }
                }
            }
        }

        Self {
            coordinates,
            height,
            width,
        }
    }
}

impl AntennaMap {
    fn within_bounds(&self, (y, x): (isize, isize)) -> bool {
        0 <= y && y < self.height as isize && 0 <= x && x < self.width as isize
    }

    fn antinodes(&self) -> HashSet<(usize, usize)> {
        let mut res = HashSet::new();

        for coordinates in self.coordinates.values() {
            for (start, p1) in coordinates.iter().enumerate() {
                for p2 in &coordinates[start + 1..] {
                    let points =
                        IntoIterator::into_iter([antinode_of(*p1, *p2), antinode_of(*p2, *p1)])
                            .filter(|p| self.within_bounds(*p));

                    for (y, x) in points {
                        res.insert((y as usize, x as usize));
                    }
                }
            }
        }

        res
    }
}

fn main() {
    let data = fs::read_to_string("input8").unwrap();
    let map = AntennaMap::from(data.as_str());

    let res = map.antinodes().len();
    assert_eq!(res, 285);
    println!("{}", res);
}

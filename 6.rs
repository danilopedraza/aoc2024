use std::fs;

#[derive(Clone, Copy, Debug)]
enum Cell {
    Free,
    Obstructed,
}

#[derive(Clone, Copy, Debug)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Clone, Debug)]
struct MazeState<const HEIGHT: usize, const WIDTH: usize> {
    maze: [[Cell; WIDTH]; HEIGHT],
    guard: (usize, usize, Orientation),
}

#[derive(Clone, Copy, Debug)]
enum MoveResult {
    Moved,
    Blocked,
    FinalPosition,
}

impl<const HEIGHT: usize, const WIDTH: usize> MazeState<{ HEIGHT }, { WIDTH }> {
    fn next_guard_state(&self) -> Option<(usize, usize, Cell)> {
        let (y, x, dir) = self.guard;

        let (dy, dx): (i32, i32) = match dir {
            Orientation::Up => (-1, 0),
            Orientation::Right => (0, 1),
            Orientation::Down => (1, 0),
            Orientation::Left => (0, -1),
        };

        let (new_y, new_x) = ((y as i32 + dy) as usize, (x as i32 + dx) as usize);

        self.maze
            .get(new_y)
            .and_then(|row| row.get(new_x))
            .copied()
            .map(|dir| (new_y, new_x, dir))
    }

    fn next_state(&mut self) -> MoveResult {
        for _ in 0..4 {
            let (y, x, dir) = self.guard;
            match self.next_guard_state() {
                Some((new_y, new_x, Cell::Free)) => {
                    self.maze[y][x] = Cell::Free;
                    self.maze[new_y][new_x] = Cell::Obstructed;
                    self.guard = (new_y, new_x, dir);
                    return MoveResult::Moved;
                }
                Some((_, _, Cell::Obstructed)) => {
                    self.guard = (y, x, dir.turn_right());
                    continue;
                }
                None => return MoveResult::FinalPosition,
            }
        }

        MoveResult::Blocked
    }

    fn traversed_cells(mut self) -> usize {
        let mut traversed = [[false; WIDTH]; HEIGHT];
        let (y, x, _) = self.guard;
        let mut res = 1;
        traversed[y][x] = true;

        while matches!(self.next_state(), MoveResult::Moved) {
            let (y, x, _) = self.guard;

            if !traversed[y][x] {
                res += 1;
                traversed[y][x] = true;
            }
        }

        res
    }
}

fn get_guard_state(maze: &Vec<Vec<char>>) -> (usize, usize, Orientation) {
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            match maze[row][col] {
                '>' => return (row, col, Orientation::Right),
                '<' => return (row, col, Orientation::Left),
                '^' => return (row, col, Orientation::Up),
                'v' => return (row, col, Orientation::Down),
                _ => continue,
            }
        }
    }

    panic!();
}

impl<const HEIGHT: usize, const WIDTH: usize> From<&Vec<Vec<char>>>
    for MazeState<{ HEIGHT }, { WIDTH }>
{
    fn from(str_maze: &Vec<Vec<char>>) -> Self {
        let guard = get_guard_state(str_maze);

        let mut maze = [[Cell::Free; WIDTH]; HEIGHT];
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                maze[row][col] = match str_maze[row][col] {
                    '.' => Cell::Free,
                    _ => Cell::Obstructed,
                }
            }
        }

        Self { maze, guard }
    }
}

fn main() {
    let data = fs::read_to_string("input6").unwrap();
    let rows: Vec<Vec<char>> = data.split('\n').map(|row| row.chars().collect()).collect();

    let maze_state: MazeState<130, 130> = MazeState::from(&rows);
    let res = maze_state.clone().traversed_cells();

    assert_eq!(res, 4711);
    println!("{res}");
}

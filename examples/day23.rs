use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day23.txt");
const _TEST_INPUT: &str = include_str!("inputs/day23_test.txt");

fn try_add(co: &Co, offset: (isize, isize), rows: usize, cols: usize) -> Option<Co> {
    let row = co.0.checked_add_signed(offset.0);
    let col = co.1.checked_add_signed(offset.1);
    row.zip(col).filter(|&(row, col)| row < rows && col < cols)
}

enum Dir {
    East,
    South,
    West,
    North,
}

impl Dir {
    fn try_from_char(c: char) -> Option<Self> {
        use Dir as D;
        match c {
            '>' => Some(D::East),
            '<' => Some(D::West),
            '^' => Some(D::North),
            'v' => Some(D::South),
            _ => None,
        }
    }
    fn as_tuple(&self) -> (isize, isize) {
        match self {
            Dir::North => (-1, 0),
            Dir::East => (0, 1),
            Dir::South => (1, 0),
            Dir::West => (0, -1),
        }
    }
}

type Co = (usize, usize);

fn dfs_longest_path(
    path: Vec<Co>,
    end: Co,
    grid: &[Vec<char>],
    compelled_step: Option<(isize, isize)>,
) -> Option<usize> {
    //println!("{:?}", path);

    let start = path.last().unwrap();

    if start == &end {
        return Some(path.len() - 1);
    }

    let offsets = if let Some(ofs) = compelled_step {
        vec![ofs]
    } else {
        vec![(-1isize, 0isize), (1, 0), (0, -1), (0, 1)]
    };

    let rows = grid.len();
    let cols = grid[0].len();
    offsets
        .into_iter()
        .filter_map(|ofs| try_add(start, ofs, rows, cols))
        .filter(|co| grid[co.0][co.1] != '#')
        .filter(|co| !path.contains(co))
        .filter_map(|co| {
            let compelled_step =
                Dir::try_from_char(grid[co.0][co.1]).and_then(|d| Some(d.as_tuple()));
            let mut npath = path.clone();
            npath.push(co);
            dfs_longest_path(npath, end, grid, compelled_step)
        })
        .max()
}

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();

    let grid = lines.map(|line| line.chars().collect_vec()).collect_vec();
    let start = (0, 1);

    let longest =
        dfs_longest_path(vec![start], (grid.len() - 1, grid.len() - 2), &grid, None).unwrap();

    println!("{longest}");

    Ok(())
}

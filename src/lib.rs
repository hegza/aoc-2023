mod co2;
mod co3;

pub use co2::*;
pub use co3::*;

pub const CARDINAL_OFFSETS: &[(i64, i64)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
pub const DIAGONAL_OFFSETS: &[(i64, i64)] = &[(-1, -1), (1, -1), (1, 1), (-1, 1)];

/// Returns an iterator across the coordinates adjacent to the given coordinate, exluding edges as
/// determined by `rows` and `cols`.
pub fn adjacents(
    co: (usize, usize),
    rows: usize,
    cols: usize,
) -> impl Iterator<Item = (usize, usize)> {
    CARDINAL_OFFSETS
        .iter()
        .chain(DIAGONAL_OFFSETS.iter())
        .filter_map(move |(r_off, col_off)| {
            let r = co.0 as i64 + r_off;
            let c = co.1 as i64 + col_off;
            if r == -1 || c == -1 || r as usize == rows || c as usize == cols {
                return None;
            }
            Some((r as usize, c as usize))
        })
}

/// Look for `target` in `table` and returns its `(row, column)`
pub fn find2d<'a, T>(target: &T, table: &[Vec<T>]) -> Option<(usize, usize)>
where
    T: PartialEq,
{
    table.into_iter().enumerate().find_map(|(row, line)| {
        line.iter()
            .position(|c| c == target)
            .and_then(|col| Some((row, col)))
    })
}

/// Returns an iterator across the coordinates adjacent to the given coordinate, exluding edges as
/// determined by `rows` and `cols`.
pub fn adjacents(
    co: (usize, usize),
    rows: usize,
    cols: usize,
) -> impl Iterator<Item = (usize, usize)> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .filter_map(move |(r_off, col_off)| {
        let r = co.0 as isize + r_off;
        let c = co.1 as isize + col_off;
        if r == -1 || c == -1 || r as usize == rows || c as usize == cols {
            return None;
        }
        Some((r as usize, c as usize))
    })
}

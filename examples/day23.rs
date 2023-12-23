use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

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

/*fn dfs_longest_path_part2(path: Vec<Co>, end: Co, grid: &[Vec<char>]) -> Option<usize> {
    let start = path.last().unwrap();

    if start == &end {
        return Some(path.len() - 1);
    }

    let rows = grid.len();
    let cols = grid[0].len();
    [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|ofs| try_add(start, ofs, rows, cols))
        .filter(|co| grid[co.0][co.1] != '#')
        .filter(|co| !path.contains(co))
        .filter_map(|co| {
            let mut npath = path.clone();
            npath.push(co);
            dfs_longest_path_part2(npath, end, grid)
        })
        .max()
}
*/

fn dfs_longest_path(
    path: Vec<Co>,
    ends: &[Co],
    grid: &[Vec<char>],
    compelled_step: Option<(isize, isize)>,
) -> Option<Vec<Co>> {
    //println!("{:?}", path);

    let start = path.last().unwrap();

    if ends.contains(start) {
        return Some(path);
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
            dfs_longest_path(npath, ends, grid, compelled_step)
        })
        .max_by(|path1, path2| path1.len().cmp(&path2.len()))
}

fn find_longest_path(start: Co, ends: &[Co], grid: &[Vec<char>]) -> Vec<((usize, usize), usize)> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut q = VecDeque::new();
    q.push_back(vec![start]);

    let mut nodes = vec![(start, 0)];

    while let Some(path) = q.pop_front() {
        let here = path.last().unwrap();
        let ncoords = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|ofs| try_add(&here, ofs, rows, cols))
            .filter(|co| grid[co.0][co.1] != '#')
            .filter(|co| !path.contains(co))
            .collect_vec();

        // Have we arrived at a crossing?
        if ncoords.len() > 1 || ends.contains(&here) {
            // Resolve the longest path from previous node to this point and record it at the node
            let (pstart, plen) = nodes.last().unwrap();
            println!("Crossing: {:?}", here);
            println!("DFS from {:?} to {:?}", pstart, here);
            // TODO: compelled step from path.first to path.second OR NOT
            let longest_path_len = dfs_longest_path(vec![*pstart], &[*here], grid, None)
                .unwrap()
                .len()
                - 1;
            nodes.push((*here, longest_path_len + plen));

            if ends.contains(here) {
                return nodes;
            }

            // Collapse the queue
            q.clear();
        }

        let npaths = ncoords
            .into_iter()
            .map(|co| {
                let mut npath = path.clone();
                npath.push(co);
                npath
            })
            .collect_vec();
        q.extend(npaths);
    }

    nodes
}

fn find_crossings(grid: &[Vec<char>]) -> Vec<Co> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut crossings = vec![];
    for (row, line) in grid.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if c != &'#'
                && [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)]
                    .into_iter()
                    .filter_map(|ofs| try_add(&(row, col), ofs, rows, cols))
                    .filter(|co| grid[co.0][co.1] != '#')
                    .count()
                    > 2
            {
                crossings.push((row, col));
            }
        }
    }

    crossings
}

#[derive(Debug, Clone)]
struct Edge(Co, usize);

fn find_edges(node: Co, nodes: &[Co], grid: &[Vec<char>]) -> Vec<Edge> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut q = VecDeque::new();
    q.push_back(vec![node]);

    let mut edges = vec![];

    while let Some(path) = q.pop_front() {
        let start = path.last().unwrap();

        // If we hit an edge, store this path as an edge, otherwise keep looking
        if *start != node && nodes.contains(start) {
            edges.push(Edge(*start, path.len() - 1));
        } else {
            let cos = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|ofs| try_add(start, ofs, rows, cols))
                .filter(|co| grid[co.0][co.1] != '#')
                .filter(|co| !path.contains(co))
                .map(|co| {
                    let mut npath = path.clone();
                    npath.push(co);
                    npath
                })
                .collect_vec();
            q.extend(cos);
        }
    }

    edges
}

fn find_all_edges(nodes: &[Co], grid: &[Vec<char>]) -> HashMap<Co, Vec<Edge>> {
    let mut all_edges = HashMap::new();

    for node in nodes {
        let edges = find_edges(*node, nodes, grid);
        all_edges.insert(*node, edges);
    }

    all_edges
}

fn make_graph(start: Co, end: Co, grid: &[Vec<char>]) -> HashMap<Co, Vec<Edge>> {
    let mut crossings = find_crossings(grid);
    crossings.push(start);
    crossings.push(end);

    let edges = find_all_edges(&crossings, grid);

    edges
}

fn find_longest(start: Co, end: Co, src_graph: HashMap<Co, Vec<Edge>>) -> usize {
    use petgraph::{algo, prelude::*};

    let mut graph = DiGraph::<Co, usize>::new();

    for (node, _) in src_graph.iter() {
        graph.add_node(*node);
    }
    for (node, edges) in src_graph.iter() {
        let node = graph.node_indices().find(|i| graph[*i] == *node).unwrap();
        for Edge(dest, wgt) in edges {
            graph.add_edge(
                node,
                graph.node_indices().find(|i| graph[*i] == *dest).unwrap(),
                *wgt,
            );
        }
    }

    let start = graph.node_indices().find(|i| graph[*i] == start).unwrap();
    let end = graph.node_indices().find(|i| graph[*i] == end).unwrap();
    let paths =
        algo::all_simple_paths::<Vec<_>, _>(&graph, start, end, 0, None).collect::<Vec<_>>();
    let paths = paths
        .into_iter()
        .map(|path| path.into_iter().map(|idx| graph[idx]).collect_vec())
        .collect_vec();

    let path_lens = paths.into_iter().map(|path| {
        let len = path
            .windows(2)
            .map(|win| {
                let (src, dest) = (win[0], win[1]);
                src_graph
                    .get(&src)
                    .unwrap()
                    .iter()
                    .find_map(|edge| (edge.0 == dest).then_some(edge.1))
                    .unwrap()
            })
            .sum::<usize>();
        len
    });
    let longest_path: usize = path_lens.max().unwrap();

    longest_path
}

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();

    let grid = lines.map(|line| line.chars().collect_vec()).collect_vec();
    let start = (0, 1);
    let end = (grid.len() - 1, grid.len() - 2);

    let longest = dfs_longest_path(vec![start], &[end], &grid, None)
        .unwrap()
        .len()
        - 1;

    println!("Part 1: {longest}");

    let graph = make_graph(start, end, &grid);
    let longest = find_longest(start, end, graph);
    println!("Part 2: {longest}");

    Ok(())
}

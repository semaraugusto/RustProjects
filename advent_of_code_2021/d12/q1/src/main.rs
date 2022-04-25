use std::collections::{HashMap, HashSet};

static START: &str = "start";
static END: &str = "end";

fn main() {
    let mut graph: HashMap<&'static str, Vec<&'static str>> = HashMap::new();

    include_str!("../../test.in")
        .trim()
        .lines()
        .for_each(|line| {
            // println!("line: {:?}", line);
            let pair = line.split_terminator('-').collect::<Vec<&'static str>>();
            let from = pair[0];
            let to = pair[1];
            insert_vertex(&mut graph, from, to);
            insert_vertex(&mut graph, to, from);
            // println!("result: {:?}", graph);
        });

    let result = count_paths(graph);
    println!("result: {:?}", result);
}

fn insert_vertex<'a>(
    graph: &mut HashMap<&'static str, Vec<&'static str>>,
    from: &'static str,
    to: &'static str,
) {
    graph.entry(from).or_default().push(to);
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(|x| x.is_lowercase())
}
fn count_paths(graph: HashMap<&'static str, Vec<&'static str>>) -> u32 {
    let mut visited: HashSet<&str> = HashSet::new();
    let path = Vec::new();
    recurse_paths(&graph, &mut visited, &path, START)
}

fn recurse_paths(
    graph: &HashMap<&'static str, Vec<&'static str>>,
    visited: &mut HashSet<&'static str>,
    path: &[&'static str],
    curr_node: &'static str,
) -> u32 {
    let mut path: Vec<&'static str> = path.to_vec();
    // println!("CURRENT PATH: {:?}", path);
    path.push(curr_node);
    if curr_node == END {
        return 1;
    }
    let mut counter = 0;
    let edges = graph.get(curr_node).unwrap().clone();
    // println!("curr_node: {:?} visited: {:?}", curr_node, visited);
    // println!("EDGES: {:?}", edges);
    if is_small_cave(curr_node) {
        visited.insert(curr_node);
    }
    for edge in edges.iter() {
        if !visited.contains(edge) || !is_small_cave(edge) {
            counter += recurse_paths(graph, visited, &path, edge);
        }
    }
    visited.remove(curr_node);
    // path.pop();

    counter
}

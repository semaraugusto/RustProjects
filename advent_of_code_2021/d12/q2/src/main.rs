use std::collections::{HashMap, HashSet};

static START: &str = "start";
static END: &str = "end";

fn main() {
    let mut graph: HashMap<&'static str, Vec<&'static str>> = HashMap::new();

    include_str!("../../sample2.in")
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

    println!("graph: {:?}", graph);
    let result = count_paths(graph);
    println!("result: {:?}", result);
}

fn insert_vertex(
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
    let path = Vec::new();
    recurse_paths(&graph, &path, START, false)
}

fn invalid_move(edge: &'static str, visited: &HashSet<&'static str>, duplicate: bool) -> bool {
    (is_small_cave(edge) && visited.contains(edge) && duplicate) || edge == START
}

fn recurse_paths(
    graph: &HashMap<&'static str, Vec<&'static str>>,
    path: &[&'static str],
    curr_node: &'static str,
    duplicate: bool,
) -> u32 {
    let mut path: Vec<&'static str> = path.to_vec();
    let mut counter = 0;
    let mut duplicate = duplicate;
    let edges = graph.get(curr_node).unwrap().clone();
    let visited: HashSet<&'static str> = HashSet::from_iter(path.iter().cloned());

    if is_small_cave(curr_node)
        && visited.contains(curr_node)
        && curr_node != START
        && curr_node != END
    {
        duplicate = true;
    }
    path.push(curr_node);

    if curr_node == END {
        // if path.len() < 15 {
        println!("CURRENT PATH: {:?}", path);
        // }
        return 1;
    }
    // visited.insert(curr_node);
    for edge in edges.iter() {
        if invalid_move(edge, &visited, duplicate) {
            continue;
        } else {
            counter += recurse_paths(graph, &path, edge, duplicate);
        }
    }
    path.pop();

    counter
}

struct ParsedNode<'a> {
    children: Vec<ParsedNode<'a>>,
    metadata: &'a [u8],
}

fn create_node(inp: &[u8]) -> (ParsedNode, &[u8]) {
    let mut child_count = inp[0];
    let metadata_length = inp[1] as usize;
    let mut children = Vec::new();
    let mut rest = &inp[2..];

    while child_count > 0 {
        let (n, next_rest) = create_node(&rest);
        rest = next_rest;
        children.push(n);
        child_count -= 1;
    }
    let metadata = &rest[..metadata_length];
    (ParsedNode { children, metadata }, &rest[metadata_length..])
}

#[aoc(day8, part1)]
pub fn part1(inp: &str) -> usize {
    let nums: Vec<u8> = inp
        .split_whitespace()
        .map(|x| x.parse())
        .filter_map(|x| x.ok())
        .collect();
    let (node, rest) = create_node(&nums[..]);
    assert!(rest.len() == 0);
    part1_computation(&node)
}

fn part1_computation(n: &ParsedNode) -> usize {
    n.children.iter().map(part1_computation).sum::<usize>()
        + n.metadata.iter().map(|x| *x as usize).sum::<usize>()
}

// Because the nodes are just referring to slices of the input I can't
// use a cargo aoc generator because it doesn't pass the lifetimes on properly
#[aoc(day8, part2)]
pub fn part2(inp: &str) -> usize {
    let nums: Vec<u8> = inp
        .split_whitespace()
        .map(|x| x.parse())
        .filter_map(|x| x.ok())
        .collect();
    let (node, rest) = create_node(&nums[..]);
    assert!(rest.len() == 0);

    get_node_value(&node)
}

fn get_node_value(node: &ParsedNode) -> usize {
    if node.children.is_empty() {
        return node.metadata.iter().map(|x| *x as usize).sum();
    } else {
        let mut res = 0;
        for ind in node.metadata.into_iter() {
            if *ind == 0 {
                continue;
            }
            if let Some(child) = node.children.get(*ind as usize - 1) {
                res += get_node_value(child);
            }
        }
        return res;
    }
}

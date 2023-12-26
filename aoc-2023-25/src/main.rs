#![feature(test)]
use aoc_2023_25::{output_graphviz, AocError, InputModel, mincut_stoer_wagner};

const INPUT: &str = include_str!("../data/input.txt");

fn part1(input: &InputModel) -> Result<String, AocError> {
    let graph = input.graph.clone();
    output_graphviz(&graph)?;

    //let edges = find_most_used_edges(&graph);
    //println!("Most used edges: {:?}", edges);

    // use stoer_wagner to find min cut
    let (n, partition) = mincut_stoer_wagner(&graph);
    let f1 = partition.len();
    let f2 = graph.len() - f1;

    // cuts found with generated graphviz plot
    //let graph = cut_graph(&graph, "pxp", "nqq");
    //let graph = cut_graph(&graph, "dct", "kns");
    //let graph = cut_graph(&graph, "ksq", "jxb");
    //let f1 = flood_fill(&graph, "pxp").len();
    //let f2 = flood_fill(&graph, "nqq").len();

    println!("f1: {:?}", f1);
    println!("f2: {:?}", f2);

    
    let result = f1 * f2;
    return Ok(result.to_string());
}

fn part2(_input: &InputModel) -> Result<String, AocError> {
    return Ok("".to_string());
}

fn main() -> Result<(), AocError> {
    let input: InputModel = INPUT.parse::<InputModel>()?;
    let part1_result = part1(&input)?;
    println!("Part1: {}", part1_result);
    println!("--------------");
    let part2_result = part2(&input)?;
    println!("Part2: {}", part2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    pub fn input_data() -> InputModel {
        TEST_INPUT.parse::<InputModel>().unwrap()
    }

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let actual = part1(&input_data()).unwrap();
        let expected = "54";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "";

        assert_eq!(actual, expected);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| INPUT.parse::<InputModel>().unwrap())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let data = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part1(&data))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let data = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part2(&data))
    }
}

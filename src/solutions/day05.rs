use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(5);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

#[derive(Debug)]
struct Map {
    start: i64,
    end: i64,
}

fn part_one(input: String) -> i64 {
    let (seeds, groups) = seeds_and_groups(&input);
    let mut min = i64::MAX;

    for n in seeds {
        let mut current_value = n;
        'outer: for (_, srcs, dsts) in &groups {
            for i in 0..srcs.len() {
                if current_value >= srcs[i].start && current_value <= srcs[i].end {
                    let diff_from_start = current_value - srcs[i].start;
                    current_value = dsts[i].start + diff_from_start;

                    continue 'outer;
                }
            }
        }

        if current_value < min {
            min = current_value;
        }
    }

    min
}

fn part_two(input: String) -> i64 {
    let (seeds, groups) = seeds_and_groups(&input);
    let seed_ranges = seeds
        .chunks(2)
        .map(|chunk| {
            let start = chunk.first().unwrap().to_owned();
            let end = start + chunk.last().unwrap();

            Map { start, end }
        })
        .collect::<Vec<_>>();
    let mut min = i64::MAX;

    // This is sloooow... 😢
    // Finishes in ~115s in release mode.
    for range in seed_ranges {
        for n in range.start..=range.end {
            let mut current_value = n;
            'outer: for (_, srcs, dsts) in &groups {
                for i in 0..srcs.len() {
                    if current_value >= srcs[i].start && current_value <= srcs[i].end {
                        let diff_from_start = current_value - srcs[i].start;
                        current_value = dsts[i].start + diff_from_start;

                        continue 'outer;
                    }
                }
            }

            if current_value < min {
                min = current_value;
            }
        }
    }

    min
}

#[allow(clippy::type_complexity)]
fn seeds_and_groups(input: &str) -> (Vec<i64>, Vec<(String, Vec<Map>, Vec<Map>)>) {
    let mut parts = input.split("\n\n");

    let seeds_str = parts.next().unwrap().replace("seeds: ", "");
    let seeds = seeds_str
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let groups = parts
        .map(|group| {
            let mut lines = group.lines();
            let map_type = lines.next().unwrap().replace(" map:", "");

            let mut srcs = Vec::new();
            let mut dsts = Vec::new();

            for l in lines {
                let [dst, src, len] = l.split_whitespace().collect::<Vec<_>>()[..] else {
                    unreachable!()
                };

                let src_start = src.parse::<i64>().unwrap();
                let dst_start = dst.parse::<i64>().unwrap();
                let len = len.parse::<i64>().unwrap();

                srcs.push(Map {
                    start: src_start,
                    end: src_start + len - 1,
                });

                dsts.push(Map {
                    start: dst_start,
                    end: dst_start + len - 1,
                });
            }

            (map_type, srcs, dsts)
        })
        .collect::<Vec<_>>();

    (seeds, groups)
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 35;
    static SOLUTION_TWO: i64 = 46;
    static TEST_INPUT: &str = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn part_one() {
        let x = input::test_vec_raw(TEST_INPUT);
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::test_vec_raw(TEST_INPUT);
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}

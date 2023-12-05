#[derive(Debug)]
struct Map {
    begin: i64,
    end: i64,
    delta: i64,
}

pub fn solve(input: &str) -> i64 {
    let mut lines = input.lines();
    let seeds_line: String = lines.nth(0).unwrap().to_owned();
    let seeds: Vec<i64> = seeds_line.split_whitespace().skip(1)
        .map(|x| x.parse::<i64>().expect("couldnt parse int")).collect();

    let mut maps: Vec<Vec<Map>> = Vec::new();
    maps.push(Vec::new());
    let mut i = 0;
    for line in lines.rev() {
        if line.trim().is_empty() {
            i += 1;
            if i != 7 {
                maps.push(Vec::new());
            }
        } else if !line.contains("map") {
            let mut this_map = line.split_whitespace();
            let begin = this_map.nth(0).unwrap().parse::<i64>().expect("couldnt parse int");
            let delta = this_map.nth(0).unwrap().parse::<i64>().expect("couldnt parse int") - begin;
            let end = begin + this_map.nth(0).unwrap().parse::<i64>().expect("couldnt parse int") - 1;
            maps[i].push(Map{ begin, end, delta });
        }
    }
    for map_vec in maps.iter_mut() {
        map_vec.sort_by(|x, y| (x.begin).cmp(&y.begin));
    }

    let ans = (0..50000000).map(|loc| { // i love heuristics
        let mut id = loc;
        for map_vec in maps.iter() {
            for map in map_vec {
                if map.begin <= id && id <= map.end {
                    id += map.delta;
                    break;
                }
            }
        }

        for seed_range in seeds.chunks(2) {
            if seed_range[0] <= id && id <= seed_range[0] + seed_range[1] - 1 {
                return loc;
            }
        }
        std::i64::MAX - 1
    }).min().expect("shouldnt be empty");

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "seeds: 79 14 55 13

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
56 93 4";
        assert_eq!(solve(input), 46);
    }
}


pub fn map_element(s: &str, seed: i64) -> i64 {
    let elem_vec = s
            .split(' ')
            .map(|val| val.parse::<i64>().unwrap()).collect::<Vec<i64>>() ;
    if let [dest, source, range] = *elem_vec.as_slice() {
        if seed >= source && seed < source + range {
            return  seed - source + dest;
        } else {
            return seed;
        }
    }
    unreachable!()
}


pub fn process_part1(input: &str) -> i64 {
    let (seeds, mapper) = input.split_once("\n\n").expect("Incorect seeds format");
    let (_, seeds) = seeds.split_once(" ").unwrap();
    let mut seeds = seeds
            .split(' ')
            .map(|val| val.parse::<i64>().unwrap()).collect::<Vec<i64>>() ;
    let maps = mapper.split("\n\n").collect::<Vec<&str>>();
    let flow = maps.iter().map(|to_mapper|
         {
            let (_, lines) = to_mapper.split_once("\n").unwrap();
            lines.split("\n").collect::<Vec<&str>>()
         }).collect::<Vec<Vec<&str>>>();
    let mut min = i64::MAX;
    for seed in seeds.iter_mut() {
        for to_mapper in flow.iter() {
            let mut mapping: Vec<i64> = Vec::new();
            for line in to_mapper.iter() {
                mapping.push(map_element(line, *seed));
            }
            for element in mapping {
                if element != *seed {
                    *seed = element;
                    break;
                }
            }
        }
        if *seed < min {
            min = *seed;
        }
        };
    min
}

pub fn process_part2(input: &str) -> i64 {
    let (seeds, mapper) = input.split_once("\n\n").expect("Incorect seeds format");
    let (_, seeds) = seeds.split_once(" ").unwrap();
    let mut seeds = seeds
        .split(' ')
        .map(|val| val.parse::<i64>().unwrap()).collect::<Vec<i64>>() ;
    seeds = (seeds[0]..seeds[0]+seeds[1]).chain(seeds[2]..seeds[2]+seeds[3]).collect::<Vec<i64>>();
    let maps = mapper.split("\n\n").collect::<Vec<&str>>();
    let flow = maps.iter().map(|to_mapper|
        {
            let (_, lines) = to_mapper.split_once("\n").unwrap();
            lines.split("\n").collect::<Vec<&str>>()
        }).collect::<Vec<Vec<&str>>>();
    let mut min = i64::MAX;
    for seed in seeds.iter_mut() {
        for to_mapper in flow.iter() {
            let mut mapping: Vec<i64> = Vec::new();
            for line in to_mapper.iter() {
                mapping.push(map_element(line, *seed));
            }
            for element in mapping {
                if element != *seed {
                    *seed = element;
                    break;
                }
            }
        }
        if *seed < min {
            min = *seed;
        }
    };
    min
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() {
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
        let result = process_part1(&input);
        assert_eq!(result, 35);
    }
}
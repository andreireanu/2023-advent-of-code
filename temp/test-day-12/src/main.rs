use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
struct SpringRow {
    springs: Vec<Option<bool>>,
    ranges: Vec<usize>,
}

impl SpringRow {
    // Recursive with dynamic programming (intermediate results are cached)

    fn recurse(&self, si: usize, ri: usize, cache: &mut HashMap<(usize, usize), u64>) -> u64 {
        // Exhausted ranges
        println!("Calc for: {} {} ", si, ri);
        if ri == self.ranges.len() {
            // All ranges accounted for, no '#' may follow in the end
            if si >= self.springs.len() ||
                self.springs[si..].iter().all(|s| *s != Some(true))
            {
                return 1;
            } else {
                // Invalid result, there are still '#' that follow
                return 0;
            }
        }

        // Exhausted springs, this path was invalid
        if si >= self.springs.len() {
            return 0;
        }

        if let Some(res) = cache.get(&(si, ri)) {
            return *res;
        }

        // We are at '.', move ahead by one
        let spring = self.springs[si];
        if let Some(false) = spring {
            return self.recurse(si + 1, ri, cache);
        }

        // We are now at '#' or '?'
        let range = self.ranges[ri];
        println!("Range: {}", range);
        // Number of arrangements when assuming we are at a '#'
        let mut arrangements = if self.can_advance(si, range) {
            println!{"Can advance:{} {}", si, range };
            self.recurse(si + range + 1, ri + 1, cache)
        } else { 0 };

        // We are at a '?', consider skipping
        if spring.is_none() {
            arrangements += self.recurse(si + 1, ri, cache);
        }

        cache.insert((si, ri), arrangements);
        println!("Arr {}", arrangements);
        arrangements
    }

    fn can_advance(&self, si: usize, range: usize) -> bool {
        let end = si + range;
        if end > self.springs.len() {
            return false;
        }
        for i in si..end {
            if let Some(false) = self.springs[i] {
                return false;
            }
        }
        // Next field is '#', the continuous range is too long
        if let Some(Some(true)) = self.springs.get(end) {
            return false;
        }
        true
    }

    fn unfold(&self, n: usize) -> Self {
        let spring_repeat = vec![self.springs.clone(); n];
        let springs = spring_repeat.join(&None);
        let ranges = self.ranges.repeat(n);

        Self {
            springs,
            ranges,
        }
    }
}

impl FromStr for SpringRow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs_s, ranges_s) = s.split_once(' ').unwrap();
        let springs = springs_s.bytes().map(|b| match b {
            b'.' => Some(false),
            b'#' => Some(true),
            b'?' => None,
            _ => panic!("Invalid character"),
        }).collect();
        let ranges = ranges_s.split(',').map(|n| n.parse().unwrap()).collect();
        Ok(SpringRow {
            springs,
            ranges,
        })
    }
}


fn main( ) {
    let input = fs::read_to_string("./inputDemo.txt").unwrap();
    let rows: Vec<SpringRow> = input.lines()
        .map(|l| l.parse::<SpringRow>().unwrap() )
        .collect();
    println!("{:?}", rows);
    let mut cache: HashMap<(usize, usize), u64> = HashMap::new();
    let out = rows[0].recurse(0, 0, &mut cache);
    println!("{:?}", out);
    println!("{:?}", cache);
}


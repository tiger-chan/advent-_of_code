use std::fs;

struct Range {
    pub min: u32,
    pub max: u32,
}

impl Range {
    fn contains(&self, range: &Range) -> bool {
        self.min <= range.min && range.max <= self.max
    }
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let mut split = s.split("-");
        if split.clone().count() != 2 {
            panic!("Incorrect number of values in Range");
        }

        let v1 = if let Some(s) = split.next() {
            s.parse::<u32>().unwrap()
        } else {
            0
        };
        let v2 = if let Some(s) = split.next() {
            s.parse::<u32>().unwrap()
        } else {
            0
        };

        Range {
            min: std::cmp::min(v1, v2),
            max: std::cmp::max(v1, v2),
        }
    }
}

impl From<Option<&str>> for Range {
    fn from(s: Option<&str>) -> Self {
        match s {
            Some(s) => Range::from(s),
            None => Range { min: 0, max: 0 },
        }
    }
}

fn main() {
    let data_str = fs::read_to_string("./data/section_assignments.txt").unwrap();

    let mut conained_sets: u32 = 0;

    for entry in data_str.split('\n') {
        let mut groups = entry.split(",");
        if groups.clone().count() != 2 {
            panic!("Group is not correctly configured");
        }

        let a = Range::from(groups.next());
        let b = Range::from(groups.next());
        if a.contains(&b) || b.contains(&a) {
            conained_sets += 1;
        }
    }

    println!("Contained sets: {}", conained_sets);
}

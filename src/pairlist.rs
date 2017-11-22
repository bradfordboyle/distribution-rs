use std::cmp::Ordering;

// deriving PartialOrd depends on order of fields in struct
#[derive(Debug, Eq, PartialOrd, PartialEq)]
pub struct Pair {
    value: u64,
    key: String,
}

impl Pair {
    pub fn new(value: u64, key: &str) -> Pair {
        Pair {
            value: value,
            key: key.to_string(),
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Pair) -> Ordering {
        let value = self.value.cmp(&other.value);
        let key = self.key.cmp(&other.key);
        value.then(key)
    }
}

#[cfg(test)]
mod test {
    use pairlist::Pair;
    use std::cmp::Ordering;

    #[test]
    fn pair_cmp() {
        let x = Pair::new(1, "a");
        let y = Pair::new(2, "a");
        let z = Pair::new(1, "b");

        assert_eq!(x.cmp(&x), Ordering::Equal);
        assert_eq!(x.cmp(&y), Ordering::Less);
        assert_eq!(x.cmp(&z), Ordering::Less);
    }

    #[test]
    fn pair_sort() {
        let mut vec = Vec::new();
        vec.push(Pair::new(1, "aa"));
        vec.push(Pair::new(2, "ab"));
        vec.push(Pair::new(1, "ba"));

        // reverse sorting
        vec.sort_by(|a, b| b.cmp(a));

        assert_eq!(vec[0], Pair::new(2, "ab"));
        assert_eq!(vec[1], Pair::new(1, "ba"));
        assert_eq!(vec[2], Pair::new(1, "aa"));
    }
}

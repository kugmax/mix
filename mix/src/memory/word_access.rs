#[derive(Debug, Copy, Clone)]
pub struct WordAccess {
    pub left: u8,
    pub right: u8,
    pub spec: u8,
}

impl WordAccess {
    pub fn new(left: u8, right: u8) -> WordAccess {
        if left > 5 || right > 5 {
            panic!("wrong left, right values {}:{}", left, right)
        }

        WordAccess {
            left,
            right,
            spec: (8 * left + right),
        }
    }

    pub fn new_by_spec(spec: u8) -> WordAccess {
        let left: u8 = spec / 8;
        let right: u8 = spec - left * 8;

        if left > right {
            panic!("left can't be greather then right {left}:{right}={spec}");
        }
        if left > 5 || right > 5 {
            panic!("wrong parsed values {left}:{right}={spec}");
        }

        WordAccess { left, right, spec }
    }
}

impl PartialEq for WordAccess {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right && self.spec == other.spec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_access_new() {
        for l in 0..6 {
            for r in l..6 {
                assert_eq!(WordAccess::new(l, r), WordAccess::new_by_spec(8 * l + r));
                //println!("{l} {r}")
            }
        }
    }
}

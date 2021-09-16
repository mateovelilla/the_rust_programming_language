#[derive(Debug)]
struct Rectange {
    width: u32,
    height: u32,
}
impl Rectange {
    fn can_hold(&self,other: &Rectange) ->bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectange {
            width: 8,
            height: 7,
        };
        let smaller = Rectange {
            width: 5,
            height: 1
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectange {
            width: 8,
            height: 7,
        };
        let smaller = Rectange {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
}

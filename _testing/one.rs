#[derive(Debug)]
#[allow(dead_code)]


struct Reactangle{
    width: u32,
    height: u32,
    
}

impl Reactangle{
    fn can_hold(&self, other: &Reactangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn larger_can_hold_smaller(){
        let larger = Reactangle{
            width: 8,
            height: 7,
        };
        let smaller = Reactangle{
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }
}

#[test]
fn smaller_cannot_hold_larger(){
    let larger = Reactangle{
        width: 8,
        height: 7,
    };
    let smaller = Reactangle{
        width: 5,
        height: 1,
    };
    assert!(!smaller.can_hold(&larger));
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn adds_three(num:u32)->u32{
    num+2
}

pub struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle{
    pub fn can_hold(&self,other:&Rectangle)-> bool{
        self.width>other.width && self.height>other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore = "called for practice"]
    fn explict_fail(){
     panic!("failing the test by explict calling panic macro ");
    }


    #[test]
    fn test_can_hold(){
        let rect1=Rectangle{width:8,height:3};
        let rect2=Rectangle{width:4,height:2};
        let result = rect1.can_hold(&rect2);
        let result2=rect2.can_hold(&rect1);
        assert_eq!(result,true);
        assert!(!result2);
    }


    #[test]
    fn it_adds_three(){
        assert_eq!(adds_three(3),6);
        assert_ne!(adds_three(3),!6);
    }
}

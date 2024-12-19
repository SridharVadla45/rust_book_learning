mod vegetables;
mod flowers;
use flowers::test_flowers_modules;
use vegetables::test_vegetables;
mod garden {

    pub fn test(){
        println!("called test function in garden module ")
    }
}


fn main() {
    println!("Hello, world!");
    garden::test();
    test_vegetables();
    test_flowers_modules();

}

pub trait Summary  {
    fn summarize(&self)-> String;
    fn default_test()->String{
        String::from("Testing defualt behaviour in traits ")
    }
}
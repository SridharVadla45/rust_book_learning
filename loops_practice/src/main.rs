fn main() {
    println!("Hello, world!");


    let numbers :[i32;5] =[1,2,3,4,5];
    let mut  index =0;

    while index <5{
        println!("The value at {index} is {0}",numbers[index]);
        index+=1;
    }


    for i in numbers{
        println!("The value is : {i}");
    }
}

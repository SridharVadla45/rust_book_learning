use std::collections::HashMap;

use colored::*;
fn main() {
    // compound static data structures
    let _numbers = [1, 2, 3, 4, 5];
    let _points = (0.0, 1.0, 2.3);

    // collections
    // 1 . vectors
    // 2. String
    // 3. HashMap

    // creating a empty  new vector

    let mut list_of_movies: Vec<String> = Vec::new();
    list_of_movies.push("Orange".to_string());
    list_of_movies.push(String::from("Arya"));

    let list_of_numbers = vec![1, 2, 3, 4];

    let _third = list_of_numbers[3];

    for i in list_of_numbers {
        print!("{i} ");
    }
    println!(" ");
    println!("---------------------");
    //vector : stores values of same data types in a contiguous order

    let super_heros = vec!["Cap America", "Thor", "IronMan", "SpiderMan"];
    for i in &super_heros {
        print!("{i} ");
    }
    println!("");

    // Strings
    // Strings is collection of bytes

    let empty_string = String::new();
    let reference_empty_string = &empty_string;
    println!("{reference_empty_string:p}");

    let owner = "Sridhar_Vadla";
    let _owner_string = owner.to_string();
    let _indian_cricket_capten = String::from("Rohit Sharma ");

    let mut text = String::from("Hello");
    text.push_str("World");
    text.push_str("is a first line in programming ");
    println!(
        "text lenght  : {0}  , text capacity : {1} ",
        text.len(),
        text.capacity()
    );

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let tic_tac_toe = format!("{tic}_{tac}_{toe}"); // does it takes ownership? No it doesnt takes the ownership of the input

    println!("{0}_{1}_{2}", tic, tac, toe);

    let tic = &tic_tac_toe[..3];
    println!("Tic :{tic}");
    let tac = tic_tac_toe.chars().nth(100);
    match tac {
        Some(i) => println!("character at  index 0 is : {i}"),
        None => println!("{}", "Invalid index".red()),
    }

    //  println!("char at 0th index in tic_tac_toe variable ");

    //HashMap : collection of key: value pairs

    let mut team_scores: HashMap<&str, u32> = HashMap::new();
    team_scores.insert("india", 203);

    let india_score = team_scores.get("india");
    match india_score {
        Some(val) => println!("india score : {val}"),
        None => println!("{}", "Invalid Key".red()),
    }

    team_scores.entry("Austraila").or_insert(123);
    team_scores.entry("india").or_insert(342);
    println!("Team Scores  : {team_scores:#?}");


    let mut  list_of_numbers=vec![0,1,2,3,5,4,6,7,8,9,9,8,7,9];
    print!("{list_of_numbers:?}");
    println!(" ");
    //sort the vector
    // find the length if even : (n/2,n+1/2)values/2 if odd : n/2th value 
    list_of_numbers.sort();
    let len =list_of_numbers.len();
    match len%2 {
        0 =>{
           let n_by_2th_value = &list_of_numbers[len/2];
           let n_plus_1_by_2_value =&list_of_numbers[(len/2)+1];
           let median =( *n_by_2th_value as f32 + *n_plus_1_by_2_value as f32 )/2.0;
           println!("Median : {median}");

        }
        _ =>{
         let median = &list_of_numbers[len/2];
         println!("Median : {median}");
        }
    }


    // mode of list of numbers
    let mut hash_map_of_numbers:HashMap<&i32,i32> = HashMap::new();
    for i in &list_of_numbers{
        let count =hash_map_of_numbers.entry(i).or_insert(1);
        *count+=1;
    } 
    println!("frequency of list_of_numbers : {hash_map_of_numbers:#?}");
    let mut mode:&i32 = &0;
    for (_key,value ) in &hash_map_of_numbers {
        if value>mode {
            mode = &value;
        }
    }
    println!("frequency of list_of_numbers : {hash_map_of_numbers:#?}");
    println!("Mode of list of numbers is : {mode}");
  



}

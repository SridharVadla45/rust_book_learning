use std::{fs::File, io::ErrorKind};

fn main() {
    // errors - recoverable errors - Result<T,E>
    // unrecoverable errors - panic! macro

    // unrecoverable errors - panic! macro
    // panic!("{}","explictly calling panic to interupt the program to exit ".red());

    let list_of_numbers = vec![1, 2, 3, 4];
    let two = list_of_numbers[2];
    println!("{two}");

    let greeting_file = File::open("hello.txt");
    let hello = match greeting_file {
        Ok(file) => {
            println!("File : {file:?}");
            file
        }
        Err(e) => {
             match e.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(file) => file,
                    Err(e) => panic!("{e:?}"),
                },
                _ => {
                    panic!("problem opening the file specified : {e:?}");
                }
            }
            
            // println!("error :{}",e.to_string().red());
        }
    };

    //
}

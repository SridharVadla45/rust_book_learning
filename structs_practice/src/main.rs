// defining the struct / struct signature
#[derive(Debug)]
struct User {
    name: String,
    mobile_number: String,
    email_id: String,
}


struct Point(f32,f32,f32);


// unit like struct without fields 
struct AlwaysEquals;

#[derive(Debug)]
struct Reactangle{
    length:f32,
    breadth:f32
}


impl  Reactangle {
    fn area(&self) -> f32{
        self.length*self.breadth
    }


    fn can_hold(&self,rect:&Reactangle)->bool {
        if rect.length<self.length && rect.breadth<self.breadth {
            return true
        }
        false
    }
}

  

fn main() {
    // create a instance of the struct
    let sridhar = User {
        name: String::from("SridharVadla"),
        mobile_number: String::from("1234567890"),
        email_id: String::from("test123@gmail.com"),
    };

    // we cannot apply mutable on fields , entire struct should be mutable if so

    let mut pavan = User {
        name: String::from("PavanVadla"),
        mobile_number: "1233445566".to_string(),
        email_id: String::from("test12333@gmail.com"),
    };

    pavan.email_id = "pavan123@gmail.com".to_string();
    
    
    let mut shyamala = build_user(
        String::from("shyamala"),
        "12345588".to_string(),
        String::from("TestEmail@gmail.com"),
    );

    let satyam = User {
        name: String::from("satyam"),
        ..shyamala
    };

    shyamala.email_id = "testupdate@gmail.com".to_string();

    // println!("{:#?}",shyamala);

    let origin = Point(0.0,0.0,0.0);



    let area_of_1by2=area_of_reactangle(1.0,2.0,);

    println!("The area of rectangle :{area_of_1by2}");

    let area_of_1by3=area((1.0,3.0,));

    println!("The area of rectangle_using_tuple  :{area_of_1by3}");

    let rectangle = Reactangle{
        length:1.0,
        breadth:4.3
    };

    let area_of_1by4=area_of_rectangle_struct(&rectangle);

    println!("The area of rectangle_using_tuple  :{area_of_1by4}");


    println!("The rectangle : {rectangle:?}");




    dbg!(30*2);

    dbg!(&rectangle);

     let area_method = rectangle.area();
     

     println!("The area is {area_method}");

     let rectangle2 = Reactangle{
        length:2.0,
        breadth:7.3
    };

    let can_hold = rectangle2.can_hold(&rectangle);

    println!("is rect2 can hold rect1 ? : {can_hold}");



}

fn build_user(name: String, mobile_number: String, email_id: String) -> User {
    User {
        name,
        mobile_number,
        email_id,
    }
}


fn area_of_reactangle(len:f32,breadth:f32) ->f32{
    len*breadth
}

fn area(rect:(f32,f32))->f32{
    rect.0*rect.1
}


fn area_of_rectangle_struct(rect:&Reactangle) -> f32{
    rect.length*rect.breadth
}

use crate::traits::Summary;
#[derive(Debug)]
pub enum GenderTypes{
  Male,
  Female,
  Others,
}

#[derive(Debug)]
pub enum Roles {
    Junior,
    Associte,
    Principal
}

pub struct UserInfo{
   user_name:String,
   role:Roles,
   salary:u32,
   gender:GenderTypes
}

impl UserInfo {
    pub fn new(user_name:String,role:Roles,salary:u32,gender:GenderTypes)-> Self{
      Self{
        user_name,
        role,
        salary,
        gender
      }
    }
}


impl Summary for UserInfo {
    fn summarize(&self)-> String {
        format!("I am {} ({:#?}) working for a salary of {} as a {:#?}",self.user_name,self.gender,self.salary,self.role)
    }
}
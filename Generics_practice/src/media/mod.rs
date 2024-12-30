use crate::traits::Summary;
#[derive(Debug)]
pub struct NewsArticle{
  pub name:String,
  pub location:String,
  pub author:String,
  pub content:String
}


impl NewsArticle{
  pub fn new(name:String,location:String,author:String,content:String)->Self{
    Self{
      name,
      location,
      author,
      content
    }
  }
}



impl Summary for NewsArticle{
  fn summarize(&self)-> String{
    format!("Name: {} , location: {} ,author:{},content:{}",self.name,self.location,self.author,self.content)
  }

}
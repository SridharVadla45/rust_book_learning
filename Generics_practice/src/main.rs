mod media;
mod traits;
mod profile;



use media::NewsArticle;
use traits::Summary;
use profile::{GenderTypes,Roles,UserInfo};


fn notify(item: &impl Summary){
   println!("{}",item.summarize());
}
fn main(){
    let news=NewsArticle::new("BNB".to_string(), String::from("America"),"Sridhar".to_string(),"pushpa accused for a roadshow,National Award winner ".to_string());

    println!("{news:#?}");

    let summary = news.summarize();
    println!("{summary}");

    let sridhar=UserInfo::new("Sridhar".to_string(), Roles::Associte, 120000, GenderTypes::Male);

   let sridhar_summary = sridhar.summarize();
   println!("{sridhar_summary}");

   let default_msg=NewsArticle::default_test();
   println!("{default_msg}");

   let default_msg2= UserInfo::default_test();
   println!("{default_msg2}");


   notify(&sridhar);

   


}

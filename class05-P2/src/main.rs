use futures::executor::block_on;
use async_std::task;
use std::time::Duration;

async fn learn_song2()->String{
    task::sleep(Duration::from_secs(5)).await;
    return "Learning & Singing Song".to_string() 
 }
 //In begining in learn_song fn i was printing "Learning song" 
 //Now i will make it more clear and dependent
 async fn sing_song2(song:String){
     task::sleep(Duration::from_secs(3)).await;   
     println!("{}",song);
 }
 async fn learn_and_sing_song2(){
    let f1 =  learn_song2().await;
    let f2 =  sing_song2(f1).await;

    //here we r using .await so that async doesn't execute to completion without using them
 }
 async fn dance2(){
     task::sleep(Duration::from_secs(3)).await;
    println!("dancing on second song");
 }
 async fn learn_song_and_dance2(){
     let f1 = learn_and_sing_song2();
     let f2 = dance2();
     futures::join!(f1,f2);
     // this will remain async as we r not using await
 }
 fn main(){
    block_on(learn_song_and_dance2());
    println!("Second Heyyyy");
    block_on(f());
    block_on(f2());
} 
 
//.await makes sure that ur async fn behaves like sync fn
//executors -> join!,.await,block_on
//As One is depending on other so we r using .await on line 16,17
//Agar code Async chlana hoga tou hum srf join use kraingai leikin jb dependencies 
//hongi tou phr hum .await kou use kraingai jou future return kraiga jissai kaam ke completion ka pta chlai ga
//Below Example taken from slides
async fn negate_async(n: i32) -> i32 {
    println!("Negating {}", n);
    task::sleep(std::time::Duration::from_secs(5)).await;
    println!("Finished sleeping for {}!", n);
    n * -1
}
async fn f() -> i32 {
    let neg = negate_async(1);
    let neg_task = task::spawn(negate_async(2));
    task::sleep(std::time::Duration::from_secs(1)).await;
    neg.await + neg_task.await
}
/*task::spawn k through ignite kia h aur ye async ka executor h isliye result m isne
  pehle 2 valai value kou chlaya*/
  async fn negate_async2(n: i32)-> i32 {
      println!("Negating {}",n);
      task::sleep(std::time::Duration::from_secs(5)).await;
      println!("Finished sleeping for {}!", n);
      n * -1
  }
  async fn f2() -> i32 {
      let neg = task::spawn(negate_async2(1));// here it is executed first using spawn hence nothing in que and showing sync result
      let neg_task = negate_async2(2);
      task::sleep(std::time::Duration::from_secs(1)).await;
      neg.await + neg_task.await
  }
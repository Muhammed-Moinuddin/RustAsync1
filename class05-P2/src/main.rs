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
} 
 
//.await makes sure that ur async fn behaves like sync fn
//executors -> join!,.await
//As One is depending on other so we r using .await on line 16,17
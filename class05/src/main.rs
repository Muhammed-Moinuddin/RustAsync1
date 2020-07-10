//..............first thread example then a task example
/*async requires an executor and block_on makes sure that async part is run whole and 
sole first*/ 
use futures::executor::block_on;
use std::thread;
use async_std::task;
use std::time::Duration;

async fn learn_song1(){
    thread::sleep(Duration::from_secs(5));
     println!("learning first song")
}
/*when i first ran this program it doesn't behaved like async as i just slept the whole 
thread , According to async it should print dance first as it takes less time but it didn't
.So now i will use task and await to make it work as async*/ 
//async fn sing_song(){
    
//}
async fn dance1(){
    thread::sleep(Duration::from_secs(3));
    println!("dance on first song")
}
async fn learn_song_and_dance1(){
    let f1 = learn_song1();
    let f2 = dance1();
    futures::join!(f1, f2);
}
fn main(){
// first eg
block_on(learn_song_and_dance1());
println!("First Heyy");
//Second eg
block_on(learn_song_and_dance2());
println!("Second Heyyyy");
}
//learn song
//sing song
//dance


/*.await makes the function to become restricted and run it fully before going onwards and
 this way it makes an async function to behave like sync
 executors -> join(which gives permission to run multiple async fn run together and perhaps
 join them) , .await , block_on . 
 futures do nothing unless you `.await` or poll them*/
 //When a fuction is depending on another function onthat time we use .await
async fn learn_song2(){
   task::sleep(Duration::from_secs(5)).await;
   println!("Learning second Song");
}
//In begining in learn_song fn i was printing "Learning song" 
//Now i will make it more clear and dependent
async fn sing_song2(){
    task::sleep(Duration::from_secs(5)).await;   
    println!("Singing second Song");
}
async fn learn_and_sing_song2(){
   let f1 =  learn_song2();
   let f2 =  sing_song2();
   futures::join!(f1,f2); 
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
}


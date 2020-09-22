
        //  Class 4 19 april


// use std::thread;
// use std::time::Duration;
// // use futures::executor::block_on;

// fn get_two_sites(){
//     let t_1 = thread::spawn(||{
//     thread::sleep(Duration::from_secs(5));
//     println!("Thread One");
//     });
//     let t_2 = thread::spawn(||{
//     thread::sleep(Duration::from_secs(3));
//     println!("Thread Two");
//     });
//     t_1.join();
//     t_2.join();
        
// }

// fn main() { 
    
//     get_two_sites();
    
   

//  }

                //  ASYNC (SINGLE THREAD)(ALWAYS RUN ON SINGLE THREAD)
//  use std::thread;
//  use std::time::Duration;
//  use futures::executor::block_on;
 



//  async fn t1(){
            
//     thread::sleep(Duration::from_secs(5));
//     println!("Thread One");
    
            
// }

// async fn t2(){
    
//     thread::sleep(Duration::from_secs(5));
//     println!("Thread Two");
            
            
// }

// async fn get_two_sites_async(){
//     let thread_one = t1();
//     let thread_two = t2();
//     futures::join!(thread_one,thread_two);
// }
//  fn main() { 
    
   
    
//     block_on(get_two_sites_async());

//  }


// use std::thread;
// use std::time::Duration;
// use futures::executor::block_on;

// async fn learn_song()-> String{
//     thread::sleep(Duration::from_secs(5));
//     "Taare zameen per".to_string()
// }


// async fn sing_song(song:String){
//     println!("{}",song);
// }


// async fn dance(){
//     println!("dance");
// }

// async fn learn_and_sing_song(){
//     let song = learn_song().await;
//     sing_song(song).await;
// }
// async fn async_main(){

//     let f1 = learn_and_sing_song();
//     let f2 = dance();
//     futures::join!(f1,f2);
// }

// fn main(){
//     block_on(async_main());
// }


                    // Class 5 26 april
// use futures;
// use std::time::Duration;
// use futures::executor::block_on;
// use async_std::task;
 
// async fn learn_song(){
//      task::sleep(Duration::from_secs(3)).await;
//      println!("Learn Song")
// }
 
// async fn Sing_Song(){
//     task::sleep(Duration::from_secs(4)).await;
//     println!("NOW SING IT");
// } 


// async fn dance(){
//     task::sleep(Duration::from_secs(2)).await;
//     println!("Dance");
// } 


// async fn song(){
//     task::sleep(Duration::from_secs(2)).await;
//     println!("AEE OOO  AEEE OOOO AEEEEEOOAAAA");
// } 


// async fn learn_and_sing_song(){
//     let f1 = learn_song();
//     let f2 = Sing_Song();
//     futures::join!(f1,f2);
// }

// async fn sing_and_dance(){
//     let f1 =learn_and_sing_song();
//     let f2 = dance();
    
//     futures::join!(f1,f2);
// }
//  fn main(){
//  block_on(sing_and_dance());
//  block_on(song());
// }


               //  ASYNC (SINGLE THREAD)
 use std::thread;
 use std::time::Duration;
 use futures::executor::block_on;
 



 async fn wash_your_face(){
            
    thread::sleep(Duration::from_secs(2));
    println!("Wash Your Face 3 times");
    
            
}

async fn hands_with_elbow(){
    
    thread::sleep(Duration::from_secs(2));
    println!("Wash Your Hands Including Elbow 3 Times");
            
            
}


async fn wipe_your_head(){
    
    thread::sleep(Duration::from_secs(2));
    println!("Wipe Your Head's Forth Part Once")
}


async fn wash_your_feet(){
    
    thread::sleep(Duration::from_secs(2));
    println!("Wash Your Feet Including Ankle 3 Times");
            
            
}

async fn duties_of_ablution  (){
    let thread_one = wash_your_face();
    let thread_two = hands_with_elbow();
    let thread_three =wipe_your_head();
    let thread_four =wash_your_feet();
    futures::join!(thread_one,thread_two,thread_three,thread_four);
}


 fn main() { 
    println!("Duties of Ablution ");
   
    
    block_on(duties_of_ablution());

 }

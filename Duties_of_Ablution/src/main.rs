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
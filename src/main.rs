use tokio::{
        sync::mpsc, 
        time::{sleep, Duration}
    } ;

#[tokio::main]
async fn main() {
    
    let (tr, mut rs) = mpsc::channel::<i32>(8) ;

    tokio::spawn(async move {
        for i in 0..20 {
            println!("Before send: {i}") ;
            tr.send(i).await.unwrap() ;
            println!("After send: {i}")
        }
    }) ;

    while let Some(v) = rs.recv().await {
        println!("Readed: {v}") ;        
        sleep(Duration::from_secs(1)).await ;
    }
}

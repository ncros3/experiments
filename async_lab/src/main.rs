use futures::executor::block_on;

async fn hello_world() {
    println!("Hello, world!");
}

async fn hello_city() {
    hello_world().await;
    println!("Hello, city!");
}

fn main() {
    let future_city = hello_city();
    
    block_on(future_city);
}

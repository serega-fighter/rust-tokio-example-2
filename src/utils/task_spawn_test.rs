use tokio::task::yield_now;
use std::rc::Rc;
use tokio::task;

#[tokio::main]
async fn main() {
    tokio::spawn(async {

        {
            // The scope forces `rc` to drop before `.await`.
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // `rc` is no longer used. It is **not** persisted when
        // the task yields to the scheduler
        yield_now().await;
    });

    let mut v = vec![1, 2, 3];

    let handle = task::spawn(async move { // must use async move here
        println!("Here's a vec: {:?}", v);
        v.push(5);
        return v;
    });
    let value = handle.await.unwrap();
    println!("Here's a moved vec: {:?}", value);
}
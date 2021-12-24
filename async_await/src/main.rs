fn main() {
    // Async Tokio Runtime Loop Example
    let runtime = tokio::runtime::Runtime::new();
    runtime.block_on(async {
        let network = read_from_network();
        let terminal = read_from_terminal();
        let mut foo = foo2();

        // std::fs::File does not contain async/await fns
        // Therefore, we must use tokio's executer 
        let mut f1 = tokio::fs::File::open("foo");
        let mut f2 = tokio::fs::File::open("bar");
        let copy = tokio::io::copy(&mut f1, &mut f2);
        
        select! {
            stream <- (&mut network).await => {
                // do something on stream
            }
            line <- (&mut terminal).await => {
                // do something on line
            }
            foo <- (&mut foo).await => {
            }
            _ <- copy.await => {

            }
        }    

        // _some_ bytes have been copied from foo to bar, but not all
    });
}

// Original approaches scaling fast in complexity
/*
    fn complexMain() {
        let read_from_terminal = std::thread::spawn(move || {
            let mut x = std::io::Stdin::lock();
            for line in x.lines() {
                // do something on user input
            }
        });

        let read_from_network = std::thread::spawn(move || {
            let mut x = std::net::TcpListener::bind("0.0.0.0:8080").unwrap();
            while let Ok(stream) = x.accept() {
                // do soemthing on stream
                let handle = std::thread::spawn(move || {
                    handle_connection(stream);
                });
            }
        });
    }
*/

async fn read_to_string(_: &str) {}
fn expensive_function(_: ()) {}

fn foo2(cancel: tokio::sync::mpsc::Receiver<()>) -> impl Future<Output = usize> {
    async {
        println!("foo1");
        read_to_string("file1").await;
        println!("foo1");
        race! {
            done <- read_to_string("file2").await => {
                // continue; fall-through to println below
            }
            cancel <- cancel.await => {
                return 0;
            }
        }
        read_to_string("file2").await;
        println!("foo1");
        read_to_string("file3").await;
        println!("foo2");
        0
    }
}

// Yielding desugared examples:
/*
    fn foo2Desugared() -> () {
        println!("foo1");
        let fut = read_to_string("file1");
        while !fut.is_ready() {
            std::thread::yield_now();
            fut.try_complete();
        }

        In reference to line 16:
        let fut = read_to_string("file");
        loop {
            if let Some(result) = fut.try_check_complete() {
                break result;
            } else {
                fut.try_make_progress();
                yield;
            }
        }
    }
*/

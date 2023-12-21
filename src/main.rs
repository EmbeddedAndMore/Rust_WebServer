use std::str;
use std::sync::atomic::Ordering;
use tokio;
use std::thread;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime;

async fn process_socket(mut socket:TcpStream)  {
    let mut buffer  = [0u8; 1024];
    socket.read(&mut buffer).await.unwrap();

    // println!("connection made: {}", str::from_utf8(&buffer).unwrap());

    let response = "HTTP/1.1 200 OK\r\nServer: rustws/1.0.0 (Ubuntu)\r\nDate: Fri, 15 Dec 2023 20:55:08 GMT\r\nContent-Type: text/html\r\nLast-Modified: Thu, 14 Dec 2023 12:08:16 GMT\r\nTransfer-Encoding: chunked\r\nConnection: keep-alive\r\nContent-Encoding: gzip\r\n\r\n";
    socket.write(&response.as_bytes()).await.unwrap();
    socket.flush().await.unwrap();
    socket.shutdown().await.unwrap()
}

#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    let rt = runtime::Builder::new_multi_thread()
        .thread_name_fn(||{
            static ATOMIC_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("my-thread-{}", id)
        })
        .build()
        .unwrap();
    // runtime::Handle::
    loop {
        let (socket, s_addr) = listener.accept().await.unwrap();

        // let num_tasks = metrics.
        // println!("Runtime has {} idle blocking thread pool threads", n);

        rt.spawn(async move {
            // println!("connection came").
            println!("{} - connection made: {}:{}",thread::current().name().unwrap(), s_addr.ip(), s_addr.port() );
            process_socket(socket).await
        });

        // rt.spawn(async {).await.unwrap();
    }
}

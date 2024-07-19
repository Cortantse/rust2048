use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};


mod config;

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(config::SERVER_CAPACITY));

    let listener = TcpListener::bind("0.0.0.0:" + config::SERVER_PORT).await.unwrap();
    println!("Server is running on " + config::SERVER_IP + ":" + config::SERVER_PORT);

    // 使用mpsc通道來管理連接
    let (tx, mut rx) = mpsc::channel::<(tokio::net::TcpStream, tokio::net::TcpStream)>(100);

    // 任務處理客戶端連接
    tokio::spawn(async move {
        while let Some((mut socket1, mut socket2)) = rx.recv().await {
            tokio::spawn(async move {
                let mut buffer1 = [0; 1024];
                let mut buffer2 = [0; 1024];

                // 讀取並回應第一個客戶端
                match socket1.read(&mut buffer1).await {
                    Ok(0) => return, // 連接已經關閉
                    Ok(_) => {
                        // sleep(Duration::from_secs(1)).await;
                        socket1
                            .write_all(
                                b"HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\nHello, Client 1!",
                            )
                            .await
                            .unwrap();
                    }
                    Err(e) => {
                        println!("Failed to read from socket 1; err = {:?}", e);
                    }
                }

                // 讀取並回應第二個客戶端
                match socket2.read(&mut buffer2).await {
                    Ok(0) => return, // 連接已經關閉
                    Ok(_) => {
                        sleep(Duration::from_secs(1)).await;
                        socket2
                            .write_all(
                                b"HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\nHello, Client 2!",
                            )
                            .await
                            .unwrap();
                    }
                    Err(e) => {
                        println!("Failed to read from socket 2; err = {:?}", e);
                    }
                }
            });
        }
    });

    let mut pending_socket: Option<tokio::net::TcpStream> = None;

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        if let Some(pending) = pending_socket.take() {
            // 如果有待處理的socket，將它們配對並發送給處理線程
            tx.send((pending, socket)).await.unwrap();
        } else {
            // 如果沒有待處理的socket，則將當前socket設置為待處理
            pending_socket = Some(socket);
        }

        drop(permit); // 釋放線程池資源
    }
}

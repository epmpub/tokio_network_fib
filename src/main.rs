use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            biz_logic(&mut socket).await;
        });
    }
}

async fn biz_logic(socket: &mut tokio::net::TcpStream) {
    let mut buf = [0; 1024];

    // In a loop, read data from the socket and write the data back.
    loop {
        let n = match socket.read(&mut buf).await {
            // socket closed
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };

        //print buf variable to string

        // println!("{:?}", &buf[0..n]);

        //convert buf to string
        let s = std::str::from_utf8(&buf[0..n-1]).unwrap();
        println!("{}", s);

        //convert string to i32
        let mut i: i32 = s.parse().unwrap();
        //calculate fibonacci of i
        fun_name(&mut i);


        println!("{}", i);


        //convert i to u8 array
        let s = i.to_string();
        //s append \n
        let s = s + "\n";
        let s = s.as_bytes();

        // Write the data back
        socket.write_all(s).await.unwrap();
    }
}

fn fun_name(i: &mut i32) {
    *i = match *i {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..*i {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    };
}
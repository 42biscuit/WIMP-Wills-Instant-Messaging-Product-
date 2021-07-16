mod users;

use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, sync::{Arc, RwLock}};
use tokio::{ 
    io::{AsyncReadExt, AsyncWriteExt,},
    net::*,
};
use users::User;
#[allow(unreachable_code)]
#[allow(non_snake_case)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let users = vec![
        User(
            "William\n".to_string(),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)), 8080),
        ),
        User(
            "Sam\n".to_string(),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
        ),
    ];
    let  users: Arc<RwLock<Vec<User>>> = Arc::new(RwLock::new(users));
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        let Users_list = Arc::clone(&users);
        tokio::spawn(async move {
            let mut buf_read = [0 as u8; 20];

            let users_read = (*Users_list.read().unwrap()).clone();
            println!("{:?}",users_read.clone());
            for i in users_read {
                socket.write(String::from(i.0).as_bytes()).await.unwrap();
            }

            socket.readable().await.unwrap();
            socket.read(&mut buf_read).await.unwrap();
            if buf_read[0] - 48 == 0 {
                println!("dont want to connect do they?");
                Users_list.write().unwrap().push(User(
                    String::from_utf8(buf_read[1..buf_read.iter().position(|&x|x == 0).unwrap()].to_vec()).unwrap().trim().to_string() + "\n",
                    addr,
                ));
            } else {
                {
                    let open_reader = (*Users_list.read().unwrap()).clone();
                    socket.write(open_reader[buf_read[0]as usize-49].1.ip().to_string().as_bytes()).await.unwrap();
                }
                Users_list.write().unwrap().remove(buf_read[0]as usize-49);
            }
            socket.shutdown().await.unwrap();
        });
    }

    Ok(())
}

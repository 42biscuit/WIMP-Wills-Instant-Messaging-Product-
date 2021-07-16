
use tokio::{io::{AsyncReadExt,AsyncWriteExt}, net::{TcpStream,TcpListener}};
use std::{io::{ stdin}, thread::sleep,sync::mpsc};
#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {

    let mut name = String::new();
    println!("please enter the name you wish to be identified by");
    stdin().read_line(&mut name).unwrap();
    //name += "\n";
    let mut stream = TcpStream::connect("192.168.5.89:8080").await?;

    let mut buff = [0_u8;50];
    stream.read(&mut buff).await.unwrap();
    println!("---------------currently connected-----------------\n{}\n enter a number to connect to the corresponding user, or \"0\" to wait",String::from_utf8_lossy(&buff[..]).to_string());
    let mut other_half = String::new();
    stdin().read_line(&mut other_half).unwrap();
    stream.writable().await.unwrap();
    stream.write_all((other_half.clone()+&name).as_bytes()).await?;
    stream.shutdown().await?;
    sleep(std::time::Duration::from_secs(2));
    if other_half.trim() == "0"{
        detroit_become_server().await;
    }else{
        stream.readable().await.unwrap();
        let mut buf = [0_u8;15];

        stream.read(&mut buf).await.unwrap();
        println!("{:?}",buf);
        println!("{}",String::from_utf8_lossy(&mut buf).trim().to_string()+":8081");
        you_a_client(String::from_utf8_lossy(&mut buf).trim().to_string().replace("\0", "")+":8081".trim()  ).await; 
    }
    Ok(())
}

async fn you_a_client(address:String){
    let mut stream = TcpStream::connect(address).await.unwrap();
    println!("client connected");
    chat(stream).await;
}




async fn detroit_become_server(){

    println!("----------waiting for someone to connect-----------");
    let listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();
    let ( mut stream, _) = listener.accept().await.unwrap();
    println!("user connected");
    chat(stream).await;
}
    
    

async fn chat(socket:TcpStream){
    let (mut r,mut w) = tokio::io::split(socket);
    tokio::spawn(async move{
        let mut  message = String::new();
        loop{
            stdin().read_line(&mut message).unwrap();
            match w.write(message.trim().as_bytes()).await{
                Ok(len) if len == message.len() =>{

                }
                Ok(_)=>{}
                Err(e)=>{eprintln!("error printing what you wrote to the strean \n error code:\t{}",e);}
            }
        }
    });
    let a = tokio::spawn(async move{
        let mut buff = [0_u8;100];
        loop{
            match r.read(&mut buff).await{
                Ok(_) =>{println!("{}",String::from_utf8_lossy(&buff).trim().to_string())}
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    return;
                }
                Err(e) => {
                    println!("we got an error cheif!! \nthis may be nothing to worrya bout but it may be worth checking out/n {}",e);
                }
            }
        }
    });
    a.await.unwrap();
}
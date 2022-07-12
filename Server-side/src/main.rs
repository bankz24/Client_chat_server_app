use std::io::{ErrorKind,Read,Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// My server side codes for a Client to server Chat Application

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;


fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener Failed to bind");
    server.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let mut clients  =vec![]; //defining of a vector for clients to be connected

    let (tx,rx) =mpsc::channel::<String>(); //instantiating the channel and creating a tx and rx for our channel that send and receive strings

    loop {
        if let Ok((mut socket, addr)) = server.accept(){
           println!("This client : {} connected ", addr);
            let tx = tx.clone(); //server trying to listen for clients in line 23 and 24
            clients.push(socket.try_clone().expect("Faileed to clone client"));

            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE]; // in line 27 to 31, the server is trying yo connect to those clients
                match socket.read_exact(&mut buff){
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&X | X !=0).collect::<Vec<_>>();// in line 30  to 38, if it receives amsg from those clients,it will send it onward onto the channel via tx in line 35
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                        println!("{} : {:?}", addr,msg);

                        tx.send(msg).expect("Failed to send message to RX");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing the connection with {}",addr);
                        break;
                    }
                }
                sleep();
            });

        }
        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client |{
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE,0);
                client.write_all(&buff).map(|_| client).ok()
            })

                .collect::<Vec<_>>();
        }
        sleep();
    }
    fn sleep() {
        thread::sleep(Duration::from_millis(100));
    }

}

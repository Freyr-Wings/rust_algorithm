use std::{sync::mpsc, thread};

enum ClientMessage {
    Incr, // Client wants to increment the server's counter
    Get,  // Client wants the server's current counter value
    Quit, // Client wants the server to stop
}

enum ServerMessage {
    Get(usize), // Server responds with the current counter value
}

fn main() {
    // 1) Create two channels:
    //    - One for server -> client (server_tx, client_rx)
    //    - One for client -> server (client_tx, server_rx)
    let (server_tx, client_rx) = mpsc::channel();
    let (client_tx, server_rx) = mpsc::channel();

    // 2) Spawn the 'server' thread
    let server = thread::spawn(move || {
        let mut n = 0; // This is the server's "state" (a counter)

        // The server will keep running until it receives "Quit"
        loop {
            // 3) Wait for a message from the client
            match server_rx.recv().unwrap() {
                // Stop the server
                ClientMessage::Quit => break,

                // Increment the server's counter
                ClientMessage::Incr => n += 1,

                // Send the current counter value back to the client
                ClientMessage::Get => {
                    server_tx.send(ServerMessage::Get(n)).unwrap();
                }
            }
        }
    });

    // 4) The "client" code (in the main thread)
    //    Send a series of messages to the server
    for msg in [ClientMessage::Incr, ClientMessage::Get, ClientMessage::Quit] {
        client_tx.send(msg).unwrap();
    }

    // 5) Receive the response from the server (which came through server_tx)
    let ServerMessage::Get(n) = client_rx.recv().unwrap();
    println!("{}", n);
    

    // 6) Wait for the server thread to finish
    server.join().unwrap();
}

use websocket;
use std::net::SocketAddr;

fn main() {
    let (events, mut ws) = websocket::WebSocket::new("0.0.0.0:10000".parse::<SocketAddr>().unwrap());

    for event in events {
        match event {
            websocket::WebSocketEvent::Connect(tok) => {
                println!("connected peer: {:?}", tok);

                ws.send(websocket::WebSocketEvent::TextMessage(tok, "Hello!".to_string()));
            }

            websocket::WebSocketEvent::TextMessage(tok, msg) => {
                println!("msg from {:?}", tok);

                for peer in ws.get_peers() {
                    println!("-> relaying to peer {:?}", peer);

                    let response =
                        websocket::WebSocketEvent::TextMessage(peer, format!("{:?} says \"{}\"", tok, msg));
                    ws.send(response);
                }
            }

            _ => {}
        }
    }
}

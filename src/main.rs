extern crate discord_rpc_client;
use discord_rpc_client::Client;
use i3_ipc::{
    event::{Event, Subscribe},
    I3Stream,
};
use std::{env, io, thread, time};

fn main() -> io::Result<()> {
    let mut i3 = I3Stream::conn_sub(&[Subscribe::Window, Subscribe::Workspace])?;

    let mut drpc = Client::new(1109944637281550457);
    drpc.start();

    for e in i3.listen() {
        match e? {
            Event::Window(ev) => {
                let class: String = if let Some(wproperties) = (*ev).container.window_properties {
                    if let Some(class) = wproperties.class {
                        class
                    } else {
                        String::from("Couldn't get class name")
                    }
                } else {
                    String::from("Couldn't get window properties")
                };

                let name: String = (*ev)
                    .container
                    .name
                    .unwrap_or(String::from("Unable to get name"));

                let msg = format!("{} - {}", name, class);

                match drpc.set_activity(|act| {
                    act.state(msg)
                        .details(
                            "What you see below is what I am currently doing. You Weird stalker...",
                        )
                        .assets(|assets| assets.large_image("troll").large_text("vtest"))
                }) {
                    Ok(_) => {}
                    Err(e) => println!("{}", e),
                }
            }
            _ => continue,
        }
    }

    println!("bye");
    Ok(())
}

// fn rpc() {
//     // Get our main status message
//     let state_message = env::args().nth(1).expect("Requires at least one argument");

//     // Create the client
//     let mut drpc = Client::new(1109944637281550457);

//     // Start up the client connection, so that we can actually send and receive stuff
//     drpc.start();

//     // Set the activity
//     drpc.set_activity(|act| act.state(state_message))
//         .expect("Failed to set activity");

//     // Wait 10 seconds before exiting
//     thread::sleep(time::Duration::from_secs(10));
// }

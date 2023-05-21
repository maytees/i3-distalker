use activity::{Assets, Button};
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use i3_ipc::{
    event::{Event, Subscribe},
    I3Stream,
};
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut i3 = I3Stream::conn_sub(&[Subscribe::Window, Subscribe::Workspace])?;

    let mut client = DiscordIpcClient::new("1109944637281550457")?;
    client.connect()?;

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

                let payload = activity::Activity::new()
                    .details("What you see below is what window I am currently on.")
                    .state(msg.as_str())
                    .assets(
                        Assets::new()
                            .large_image("https://vikasbackend.matees.net/9836")
                            .large_text("Go away stalker"),
                    )
                    .buttons(vec![Button::new(
                        "View stalk(-app?) repository",
                        "https://github.com/maytees/i3-distalker",
                    )]);
                client.set_activity(payload)?;
            }
            _ => {}
        }
    }

    client.close()?;
    Ok(())
}

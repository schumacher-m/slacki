extern crate slack;
extern crate term_painter;
extern crate chrono;

use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;
use slack::*;
use slack::Message::*;
use chrono::Utc;
use chrono::TimeZone;
use std::borrow::Cow;

struct Handler {
	lookup: Slacki
}

struct Slacki {
	users: Vec<User>,
	channels: Vec<Channel>
}

#[allow(unused_variables)]
impl Slacki {
	fn handle_message(&self, cli: &RtmClient, msg: Box<Message>) {
        match *msg {
            Standard(message) => {
				let text = &message.text;
				let channel = &message.channel.clone().unwrap();
				let user_id = &message.user.clone().unwrap();
				let ts = &message.ts.unwrap();
				let derp:Vec<&str> = ts.split('.').collect();
				let tss:i64 = derp.get(0).unwrap().parse().unwrap();

				match *text {
					Some(ref text) => println!("{} [{}] {}: {}",
															Utc.timestamp(tss, 0).format("%H:%M").to_string(),
															Yellow.bold().paint(self.lookup_channel(channel)),
															BrightWhite.bold().paint(self.lookup_user(user_id)),
															text),
					None => (),
				}
			}
            // MessageReplied(msg_reply) => println!("MessageReply: {:?}", msg_reply),
            _ => return,
        }
    }

}

#[allow(unused_variables)]
impl Slacki {
	fn new() -> Slacki {
		Slacki { users: vec![], channels: vec![] }
	}

	fn refresh(&mut self, cli: &RtmClient) {
		self.users = cli.start_response().users.as_ref().unwrap().clone();
		self.channels = cli.start_response().channels.as_ref().unwrap().clone();
	}

	fn lookup_user(&self, user_id: &str) -> String {
		let user = self.users.iter().find(|user| user.id == Some(String::from(user_id)));
		match user {
			Some(User) => {
				let u = user.unwrap();
				let profile = u.profile.clone().unwrap();
				let first_name = profile.first_name.unwrap();
				format!("{}", first_name)
			},
			_ => String::from("Unknown")
		}
	}

	fn lookup_channel(&self, channel_id: &str) -> String {
		let channel = self.channels.iter().find(|channel| channel.id == Some(String::from(channel_id)));
		match channel {
			Some(Channel) => {
				let c = channel.clone().unwrap();
				let name = c.name.clone().unwrap();
				format!("{}", name)
			},
			_ => String::from("Unknown")
		}
	}
}

#[allow(unused_variables)]
impl slack::EventHandler for Handler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
		match event {
			Event::Message(message) => self.lookup.handle_message(cli, message),
			_ => ()
		}
    }

    fn on_close(&mut self, cli: &RtmClient) {
        // println!("on_close");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
		self.lookup.refresh(cli);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => panic!("No api-key in args! Usage: cargo run <api-key>"),
        x => args[x - 1].clone(),
    };
    let mut handler = Handler { lookup: Slacki::new() };
    let r = RtmClient::login_and_run(&api_key, &mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}

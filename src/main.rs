use serenity::{
	async_trait,
	model::{gateway::Ready, channel::Message},
	prelude::*
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		if msg.author.bot {
			return;
		}

		if msg.content.to_lowercase().contains("silly") {
			let emoji = "<a:sildance:1462056515056828499>";
			if let Err(why) = msg.reply(&ctx.http, emoji).await {
				println!("Error sending message: {:?}", why);
			}
		}
	}

	async fn ready(&self, _: Context, ready: Ready) {
		println!("{} is connected!", ready.user.name);
	}
}

#[tokio::main]
async fn main() {
	dotenvy::dotenv().expect("Missing .env");
	let token = std::env::var("TOKEN").expect("Expected a token in the environment");
	let intents = GatewayIntents::GUILD_MESSAGES 
		| GatewayIntents::DIRECT_MESSAGES 
		| GatewayIntents::MESSAGE_CONTENT;

	let mut client = Client::builder(&token, intents)
		.event_handler(Handler)
		.await
		.expect("Err creating client");

	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}
}

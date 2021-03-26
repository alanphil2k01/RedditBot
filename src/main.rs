mod scraper;
mod commands;

use crate::commands::Parsedmsg;

use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        let parsed_msg = match Parsedmsg::new(msg){
            Some(x) => x,
            None => {return;}
        };
        if parsed_msg.command.eq("reddit"){
            parsed_msg.reddit(context).await;
        } else if parsed_msg.command.eq("ping"){
            parsed_msg.ping(context).await;
        } else if parsed_msg.command.eq("help"){
            parsed_msg.help(context).await;
        }
       //  if msg.content.contains("!wallpaper") {
       //      println!("The message was {}", msg.content);
       //      let files = vec!["https://preview.redd.it/n7fobiqluln61.jpg?width=960&crop=smart&auto=webp&s=c3934bfbf948bbd908e1376821cf92bbd00cf4f0"];
       //     if let Err(why) = msg.channel_id.send_files(&context.http, files, |m| {
       //         m.content("new wall")
       //     }).await {
       //          println!("Error sending message: {:?}", why);
       //     };
       //  }
       // if msg.content == "!wallpaper" {
       //     let files = vec!["https://preview.redd.it/n7fobiqluln61.jpg?width=960&crop=smart&auto=webp&s=c3934bfbf948bbd908e1376821cf92bbd00cf4f0"];
       //    if let Err(why) = msg.channel_id.send_files(&context.http, files, |m| {
       //        m.content("new wall")
       //    }).await {
       //         println!("Error sending message: {:?}", why);
       //    };
       // }
       // if msg.content == "!ping" {
       //     println!("{} pinged the bot", msg.author.name);
       //      if let Err(why) = msg.channel_id.say(&context.http, "Pong!").await {
       //          println!("Error sending message: {:?}", why);
       //      }
       //  }
       // if msg.content == "!messageme" {
       //      // If the `utils`-feature is enabled, then model structs will
       //      // have a lot of useful methods implemented, to avoid using an
       //      // often otherwise bulky Context, or even much lower-level `rest`
       //      // method.
       //      //
       //      // In this case, you can direct message a User directly by simply
       //      // calling a method on its instance, with the content of the
       //      // message.
       //      let dm = msg.author.dm(&context, |m| {
       //          m.content("Hello!");
       //
       //          m
       //      }).await;
       //
       //      if let Err(why) = dm {
       //          println!("Error when direct messaging user: {:?}", why);
       //      }
       //  }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

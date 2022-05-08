use tokio;
use serenity;
use image;
use std::io::Write;
use image::GenericImageView;
use lazy_mut::lazy_mut;
use substring;
use substring::Substring;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    cache
};

macro_rules! Send{
    ($string:expr, $channel_id:expr, $ctx:expr)=>{
        {
            if let Err(why) = $channel_id.say(&$ctx.http, $string).await {
                 println!("ERROR SENDING MESSAGE: {:?}", why);
            }
        }
    }
}

macro_rules! GetSpaceUntilString{
    ($string:expr, $i:expr, $output:expr)=>{
        {
            while $i < $string.len() && $string.chars().nth($i).unwrap() != ' '{
                $output.push($string.chars().nth($i).unwrap());
                $i+=1
            }
        }
    }
}

macro_rules! SendImage{
    ($filepath:expr, $channel_id:expr, $ctx:expr)=>{
        {
            $channel_id.send_files(&$ctx.http, vec![$filepath], |m| m.content($filepath)).await.unwrap();
        }
    }
}

const DIMENSIONS: [u32; 2] = [100, 100];

lazy_mut!{
    static mut IMG: image::DynamicImage = image::open("place.png").expect("WASN'T ABLE TO RETRIEVE IMG");
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.chars().nth(0).unwrap() != '!' || 
           msg.author.id == ctx.cache.current_user_id().await
        { return; }
        let mut command: String = String::new();
        let mut i = 1;
        GetSpaceUntilString!(msg.content, i, command);
        match command.as_str(){
            "help" => {
                Send!(format!("Hello! This is placebot! It's basically r/Place, but for Discord!\nThe grid for this is {}x{} and there is no time delay\nTo start placing enter !place x y r g b, where x and y are the coordinates and rgb is for the color.\nIf you just want to see the image just enter !img", DIMENSIONS[0], DIMENSIONS[1]), msg.channel_id, ctx);
            }
            "place" => {
                // Get coordinates and color and place that pixel there
                let mut pixel_data: [u8; 5] = [0; 5];
                for j in 0..5{
                    i+=1;
                    let mut current_param: String = String::new();
                    GetSpaceUntilString!(msg.content, i, current_param);
                    pixel_data[j] = match current_param.parse::<u8>(){
                        Ok(u8) => u8,
                        Err(u8) => {
                            Send!("Invalid parameters for placement.", msg.channel_id, ctx);
                            return;
                        }
                    }
                }
                unsafe{
                    if pixel_data[0] > 99 || pixel_data[1] > 99 { return; }
                    *IMG.as_mut_rgb8().unwrap().get_pixel_mut(pixel_data[0].into(), pixel_data[1].into()) = image::Rgb([pixel_data[2], pixel_data[3], pixel_data[4]]);
                    IMG.save("place.png").unwrap();
                    SendImage!("place.png", msg.channel_id, ctx);
                }
            }
            "img" => {
                SendImage!("place.png", msg.channel_id, ctx);
            }
            _ => {
                Send!("Invalid Command", msg.channel_id, ctx);
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let mut client: Client = Client::builder(
            std::fs::read_to_string("token.txt")
                        .expect("ERROR RETRIEVING TOKEN")
            )
            .event_handler(Handler)
            .await
            .expect("ERROR STARTING CLIENT EVENT HANDLER");
    if let Err(why) = client.start().await {
        println!("CLIENT ERROR: {:?}", why);
    }
}

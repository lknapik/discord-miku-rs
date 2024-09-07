use chrono::{Datelike, Timelike};
use poise::serenity_prelude::{self as serenity, ChannelId, CreateMessage, CreateScheduledEvent};
use rspotify::{model::FullTrack, prelude::BaseClient};
use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

static IMG_RUNNING: AtomicBool = AtomicBool::new(false);

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Creates a new guild event
#[poise::command(slash_command, prefix_command)]
async fn farting(
    ctx: Context<'_>
) -> Result<(), Error> {
    let g = ctx.guild_id().unwrap();

    let time = serenity::Timestamp::from_unix_timestamp(serenity::Timestamp::now().unix_timestamp()+60).unwrap();
    let end_time = serenity::Timestamp::from_unix_timestamp(serenity::Timestamp::now().unix_timestamp()+3660).unwrap();
    let mut event = CreateScheduledEvent::new(serenity::ScheduledEventType::External, "Group Farting".to_string(), time);
    event = event.location("Brap House");
    event = event.end_time(end_time);

    let foo = ctx.http().create_scheduled_event(g, &event, None).await;

    match foo {
        Ok(_v) => (),
        Err(e) => println!("{:?}", e)
    };

    ctx.say("New farting event starting in 1 minute!").await?;

    Ok(())
}

fn song_webhook(
    song: FullTrack
) -> poise::CreateReply {

    let embed = serenity::CreateEmbed::default()
        .title(format!("{}", &song.name))
        .image(&song.album.images[0].url);

    poise::CreateReply::default().embed(embed)
}

/// Embed Spotify Link
#[poise::command(slash_command, prefix_command)]
async fn song_embed(
    ctx: Context<'_>,
    #[description = "link"] link: String
) -> Result<(), Error> {

    let re = regex::Regex::new(r"\w{22}").unwrap();
    let uri = re.find(&link).unwrap();

    let creds = rspotify::Credentials::from_env().unwrap();
    let spotify = rspotify::ClientCredsSpotify::new(creds);
    spotify.request_token().await.unwrap();

    let res_song = spotify.track(rspotify::model::TrackId::from_id(uri.as_str()).unwrap() , None).await;

    match res_song {
        Ok(song) => {
            let embed = song_webhook(song);
            ctx.send(embed).await
        },
        Err(_) => ctx.say("Error finding song from link...").await
    }.unwrap();

    Ok(())
}

/// Messages the requesting user after the time has passed
#[poise::command(slash_command, prefix_command)]
async fn remindme(
    ctx: Context<'_>,
    #[description = "minutes"] time: u64,
    #[rest] #[description = "message"] msg: Option<String>
) -> Result<(), Error> {
    
    let user = ctx.author().to_owned();
    
    let ctx1 = Arc::new(ctx.serenity_context().to_owned());
    tokio::spawn(async move {
        // sleep in thread
        tokio::time::sleep(std::time::Duration::from_secs(time*60)).await;

        let reply_msg = match msg {
            Some(msg) => msg,
            None => "reminder".to_string()
        };
        
        
        let _ = user.direct_message(Arc::clone(&ctx1.http), CreateMessage::new().content(reply_msg)).await;

    });

    ctx.say(":3").await?;

    Ok(())
}

async fn daily_img(
    ctx: Arc<serenity::Context>
) -> Result<(), Error> {

    let current_time = chrono::Utc::now();

    if IMG_RUNNING.load(Ordering::Relaxed) == true {
        println!("daily_img() called when thread already spawned");
        return Ok(())
    }

    println!("Notification loop started {}", current_time);
    tokio::spawn(async move {
        IMG_RUNNING.store(true, Ordering::SeqCst);

        loop {

            let mut all_msgs: Vec<CreateMessage> = Vec::new();
            
            let current_time = chrono::Utc::now();

            let mut message_builder = CreateMessage::new();

            //Sunday night in US
            if current_time.weekday() == chrono::Weekday::Mon && current_time.hour() == 0 && current_time.minute() == 0 {
                message_builder = message_builder.content("https://tenor.com/view/mondag-gif-24190364");
                all_msgs.push(message_builder.clone());
            }

            if current_time.weekday() == chrono::Weekday::Sun && current_time.hour() == 4 && current_time.minute() == 3 {
                message_builder = message_builder.content("https://tenor.com/view/neco-arc-dance-sleep-wojak-gif-23247049");
                all_msgs.push(message_builder.clone());

            }

            if current_time.weekday() == chrono::Weekday::Tue && current_time.hour() == 14 && current_time.minute() == 0 {
                message_builder = message_builder.content("https://media.discordapp.net/attachments/867525808092610600/867525973104787496/tueaday.png");
                all_msgs.push(message_builder.clone());

            }

            if current_time.weekday() == chrono::Weekday::Thu && current_time.hour() == 14 && current_time.minute() == 0 {
                message_builder = message_builder.content("https://tenor.com/view/yuyuko-touhou-fumo-fumo-plush-yuyuko-saigyouji-yuyu-bath-gif-24163441");
                all_msgs.push(message_builder.clone());
            }

            if current_time.weekday() == chrono::Weekday::Fri && current_time.hour() == 14 && current_time.minute() == 0 {
                message_builder = message_builder.content("https://vxtwitter.com/scarletfumo/status/1375400199930179589");
                all_msgs.push(message_builder.clone());
            }

            //post this cat on the 19th of every month
            if current_time.day() == 19 && current_time.hour() == 14 && current_time.minute() == 0 {
                message_builder = message_builder.content("https://tenor.com/view/cat-kitty-pussycat-feline-gif-26001328");
                all_msgs.push(message_builder.clone());
            }

            //wake up it's the first of the month
            if current_time.day() == 1 && current_time.hour() == 14 && current_time.minute() == 0 {
                message_builder = message_builder.content("https://youtu.be/HWTwt5zv04c");
                all_msgs.push(message_builder.clone());
            }
            
            for msg in all_msgs {
                let _ = ChannelId::new(181058166967107584).send_message(&ctx, msg).await;
                println!("Sending message at {}", current_time);
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }

            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        }
    });


    Ok(())
}

#[tokio::main]
async fn main() {
    println!("Starting Bot");
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), farting(), remindme(), song_embed()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx.clone(), event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}


async fn event_handler(
    ctx: serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
            daily_img(Arc::new(ctx)).await?
        }

        _ => {}
    }
    Ok(())
}
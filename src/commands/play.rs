use serenity::{
    model::prelude::{
        interaction::{
            application_command::{
                CommandDataOption,
                CommandDataOptionValue
            },
            Interaction
        },
        command::CommandOptionType
    },
    prelude::Context,
    builder::CreateApplicationCommand
};

pub async fn run(options: &[CommandDataOption], ctx: &Context, interaction: &Interaction) -> Option<String> {
    super::join::run(options, ctx, interaction).await.expect("couldn't join");

    let option = options
        .get(0)
        .expect("Expected URL option")
        .resolved
        .as_ref()
        .expect("Expected String");

    let url: String;

    if let CommandDataOptionValue::String(ur) = option {
        url = ur.clone()
    } else {
        return Some("Please provide a valid user".to_string());
    }

    if !url.starts_with("http") {
        return Some("Must provide a valid URL".to_string());
    }

    let guild = ctx.cache.guild(interaction.clone().application_command().unwrap().guild_id.unwrap()).unwrap();
    
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                return Some("Error sourcing ffmpeg".to_string());
            },
        };

        handler.play_source(source);

        return Some("Playing song".to_string());
    } else {
        return Some("Must be in a voice channel".to_string());
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("play").description("Play a Youtube video").create_option(|option| {
        option
            .name("url")
            .description("The Youtube URL to play")
            .kind(CommandOptionType::String)
            .required(true)
    })
}

use serenity::{
    model::prelude::interaction::{
        application_command::CommandDataOption,
        Interaction
    },
    prelude::Context,
    builder::CreateApplicationCommand
};

pub async fn run(_options: &[CommandDataOption], ctx: &Context, interaction: &Interaction) -> Option<String> {
    let guild = ctx.cache.guild(interaction.clone().application_command().unwrap().guild_id.unwrap()).unwrap();

    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            return Some(format!("Failed: {:?}", e));
        }

        return Some("Left the server.".to_string());

    } else {
        return Some("Not in a voice channel".to_string());
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("leave").description("Manually tell the bot to leave a voice call")
}

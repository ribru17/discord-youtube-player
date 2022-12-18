use serenity::{
    builder::CreateApplicationCommand,
    prelude::Context,
    model::prelude::interaction::{
        Interaction,
        application_command::CommandDataOption
    }
};

pub async fn run(_options: &[CommandDataOption], ctx: &Context, interaction: &Interaction) -> Option<String> {
    let intx = interaction.clone().application_command().unwrap();
    let user_id = intx.member.unwrap().user.id;
    let guild = ctx.cache.guild(intx.guild_id.unwrap()).unwrap();
    // let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&user_id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            return Some("Must be in a channel.".to_string());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    Some("Joined channel.".to_string())

}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("join").description("Manually tell the bot to join a voice call")
}

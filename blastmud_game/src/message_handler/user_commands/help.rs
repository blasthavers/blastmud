use super::{
    VerbContext, UserVerb, UserVerbRef, UResult,
    CommandHandlingError::UserError
};
use async_trait::async_trait;
use ansi_macro::ansi;
use phf::phf_map;

static HELP_PAGES: phf::Map<&'static str, &'static str> = phf_map! {
    "" =>
        ansi!("Type <bold>help <lt>topicname><reset> to learn about a topic. Most \
          commands can be used as a topicname.\r\n\
          Topics of interest to new users:\r\n\
          \t<bold>register<reset>\tLearn about the <bold>register<reset> command.\r\n\
          \t<bold>newbie<reset>\tLearn how to survive as a newb."),
    "<topicname>" =>
        ansi!("You are supposed to replace <lt>topicname> with the topic you want \
         to learn about. Example:\r\n\
         \t<bold>help register<reset> will tell you about the register command.")
};

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &VerbContext, _verb: &str, remaining: &str) -> UResult<()> {
        let help = HELP_PAGES.get(remaining).ok_or(
            UserError("No help available on that".to_string()))?;
        ctx.trans.queue_for_session(ctx.session, &(help.to_string() + "\r\n")).await?;
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;

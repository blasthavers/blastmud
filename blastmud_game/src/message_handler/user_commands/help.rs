use super::{
    VerbContext, UserVerb, UserVerbRef, UResult,
    CommandHandlingError::UserError
};
use async_trait::async_trait;
use ansi_macro::ansi;
use phf::phf_map;

static ALWAYS_HELP_PAGES: phf::Map<&'static str, &'static str> = phf_map! {
    "<topicname>" =>
        ansi!("You are supposed to replace <lt>topicname> with the topic you want \
         to learn about. Example:\r\n\
         \t<bold>help register<reset> will tell you about the register command.")
};

static UNREGISTERED_HELP_PAGES: phf::Map<&'static str, &'static str> = phf_map! {
    "" =>
        ansi!("Type <bold>help <lt>topicname><reset> to learn about a topic. Most \
          commands can be used as a topicname.\r\n\
          Topics of interest to unregistered users:\r\n\
          \t<bold>register<reset>\tLearn about the <bold>register<reset> command.\r\n\
          \t<bold>login<reset>\tLearn how to log in as an existing user.\r\n"),
};

static REGISTERED_HELP_PAGES: phf::Map<&'static str, &'static str> = phf_map! {
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

static EXPLICIT_HELP_PAGES: phf::Map<&'static str, &'static str> = phf_map! {
    "fuck" =>
        ansi!("Type <bold>fuck <lt>name><reset> to fuck someone. It only works if \
               they have consented.")
};

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &mut VerbContext, _verb: &str, remaining: &str) -> UResult<()> {
        let mut help = None;
        let is_unregistered = match ctx.user_dat {
            None => true,
            Some(user_dat) => !user_dat.terms.terms_complete
        };
        if is_unregistered {
            help = help.or_else(|| UNREGISTERED_HELP_PAGES.get(remaining));
        } else {
            help = help.or_else(|| REGISTERED_HELP_PAGES.get(remaining));
            if !ctx.session_dat.less_explicit_mode {
                help = help.or_else(|| EXPLICIT_HELP_PAGES.get(remaining))
            }
        }
        help = help.or_else(|| ALWAYS_HELP_PAGES.get(remaining));
        let help_final = help.ok_or(
            UserError("No help available on that".to_string()))?;
        ctx.trans.queue_for_session(ctx.session,
                                    Some(&(help_final.to_string() + "\r\n"))
        ).await?;
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;

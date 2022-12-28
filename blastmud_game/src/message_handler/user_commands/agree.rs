use super::{VerbContext, UserVerb, UserVerbRef, UResult, user_error};
use crate::models::user::{User, UserTermData};
use async_trait::async_trait;
use ansi::ansi;
use chrono::Utc;

pub struct Verb;

static REQUIRED_AGREEMENTS: [&str;4] = [
    "I acknowledge that BlastMud is for adults only, and certify that I am over 18 years of age \
     (or any higher relevant age of majority in my country) and want to view this content.",
    "THIS GAME IS PROVIDED BY THE CREATORS, STAFF, VOLUNTEERS AND CONTRIBUTORS \"AS IS\" AND ANY EXPRESS OR \
     IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND \
     FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE CREATORS, STAFF, VOLUNTEERS OR \
     CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL \
     DAMAGES HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR \
     TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS GAME, EVEN IF \
     ADVISED OF THE POSSIBILITY OF SUCH DAMAGE. FOR THE AVOIDANCE OF DOUBT, THIS DISCLAIMER EXTENDS TO ANY \
     USER-SUPPLIED CONTENT THAT THE GAME MAY EXPOSE.",
    "I acknowledge that this game allows user-supplied content, and that while staff endeavour to \
     moderate it, I may encounter content that is distressing and/or outside community standards. \
     I agree that I will not use the game or any services provided in connection with it to transmit content \
     which is illegal (including by virtue of infringing copyright), infringes on the rights of others, \
     is personally identifying information, or is objectionable or abhorrent (including, without \
     limitation, any content related to sexual violence, real or fictional children under 18, bestiality, \
     the promotion or glorification of proscribed drug use, or fetishes that involve degrading or \
     inflicting pain on someone for the enjoyment of others). I agree to defend, indemnify, and hold \
     harmless the creators, staff, volunteers and contributors in any matter relating to content sent \
     (or re-sent) by me, in any matter arising from the game sending content to me, and in any matter \
     consequential to sharing my password, using an insecure password, or otherwise allowing or taking \
     inadequate measures to prevent another player from logging in as one or more of my characters.",
    "I certify that I am not, to my knowledge, currently banned from the game. I agree not to sustain any \
     contact with another player that is unwelcome, or to take any action for the purpose of harassment or \
     limiting the game for other players without all affected players' consent. I agree not to allow any \
     other person to play as my character, not to have more than 5 characters active (available to log \
     in as) at any one time, not to be logged in as more than one character at any instant in time, and \
     not to use any of my characters to help another character of mine in the game.",
];

fn user_mut<'a>(ctx: &'a mut VerbContext) -> UResult<&'a mut User> {
    match ctx.user_dat.as_mut() {
        None => Err("Checked agreements before user logged in, which is a logic error")?,
        Some(user_dat) => Ok(user_dat)
    }
}

fn terms<'a>(ctx: &'a VerbContext<'a>) -> UResult<&'a UserTermData> {
    match ctx.user_dat.as_ref() {
        None => Err("Checked agreements before user logged in, which is a logic error")?,
        Some(user_dat) => Ok(&user_dat.terms)
    }
}


fn first_outstanding_agreement(ctx: &VerbContext) -> UResult<Option<(String, String)>> {
    let existing_terms = &terms(ctx)?.accepted_terms;
    for agreement in REQUIRED_AGREEMENTS {
        let shortcode =
            base64::encode(ring::digest::digest(&ring::digest::SHA256,
                                                agreement.as_bytes()))[0..20].to_owned();
        match existing_terms.get(&shortcode) {
            None => { return Ok(Some((agreement.to_owned(), shortcode))); }
            Some(_) => {}
        }
    }
    Ok(None)
}

pub async fn check_and_notify_accepts<'a, 'b>(ctx: &'a mut VerbContext<'b>) -> UResult<bool> where 'b: 'a {
    match first_outstanding_agreement(ctx)? {
        None => {
            let user = user_mut(ctx)?;
            user.terms.terms_complete = true;
            user.terms.last_presented_term = None;
            Ok(true)
        }
        Some((text, hash)) => {
            let user = user_mut(ctx)?;
            user.terms.terms_complete = false;
            user.terms.last_presented_term = Some(hash);
            ctx.trans.queue_for_session(ctx.session, Some(&format!(ansi!(
                "Please review the following:\r\n\
                \t{}\r\n\
                Type <green><bold>agree<reset> to accept. If you can't or don't agree, you \
                unfortunately can't play, so type <red><bold>quit<reset> to log off.\r\n"),
                text))).await?;
            Ok(false)
        }
    }
}

#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &mut VerbContext, _verb: &str, _remaining: &str) -> UResult<()> {
        let user = user_mut(ctx)?;
        match user.terms.last_presented_term.as_ref() {
            None => {
                drop(user);
                user_error("There was nothing pending your agreement.".to_owned())?;
            }
            Some(last_term) => {
                user.terms.accepted_terms.insert(last_term.to_owned(), Utc::now());
                drop(user);
                if check_and_notify_accepts(ctx).await? {
                    ctx.trans.queue_for_session(ctx.session, Some(
                        ansi!("That was the last of the terms to agree to - welcome onboard!\r\n\
                               Hint: Try <bold>l<reset> to look around.\r\n"))).await?;
                }
            }
        }
        ctx.trans.save_user_model(ctx.user_dat.as_ref().unwrap()).await?;
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;

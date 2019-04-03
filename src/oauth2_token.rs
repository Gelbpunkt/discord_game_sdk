use crate::prelude::*;

#[derive(Debug)]
pub struct OAuth2Token {
    pub access_token: String,
    pub scopes: Vec<String>,
    pub expires: chrono::NaiveDateTime,
}

impl FromSys for OAuth2Token {
    type Source = sys::DiscordOAuth2Token;

    fn from_sys(source: &Self::Source) -> Self {
        Self {
            access_token: unsafe { string_from_cstr(&source.access_token as *const _) },
            scopes: unsafe { string_from_cstr(&source.scopes as *const _) }
                .split(' ')
                .map(String::from)
                .collect(),
            expires: chrono::NaiveDateTime::from_timestamp(source.expires, 0),
        }
    }
}

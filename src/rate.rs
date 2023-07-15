use crate::prelude::*;

pub struct Rate {}

impl Rate {
    /// For details, see https://github.com/Nodonisko/capacitor-rate-app
    /// Note that both IOS and Android only let you request review occasionally for each user
    /// This may not show up at during local testing but you will be able to see it in debug logs
    #[cfg(all(feature = "rate_plugin", any(feature = "ios", feature = "android")))]
    pub async fn request_review() -> Result<(), Error> {
        run_unit_unit(crate::extern_functions::request_review).await
    }
}

use crate::extern_functions::*;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// For details, https://github.com/openforge/capacitor-game-connect
pub struct GameConnect {}

impl GameConnect {
    /// Method to sign-in a user
    #[cfg(all(feature = "game_plugin", any(feature = "ios", feature = "android")))]
    pub async fn sign_in() -> Result<(), Error> {
        run_unit_unit(game_connect_sign_in).await
    }

    /// Method to display the Achievements view
    #[cfg(all(feature = "game_plugin", any(feature = "ios", feature = "android")))]
    pub async fn show_achievements() -> Result<(), Error> {
        run_unit_unit(game_connect_show_achievements).await
    }

    /// Method to display the Leaderboards
    #[cfg(all(feature = "game_plugin", any(feature = "ios", feature = "android")))]
    pub async fn show_leaderboard(options: impl Into<ShowLeaderboardOptions>) -> Result<(), Error> {
        run_value_unit(options, game_connect_show_leaderboard).await
    }

    /// Method to submit a score to the Google Play Services SDK
    #[cfg(all(feature = "game_plugin", any(feature = "ios", feature = "android")))]
    pub async fn submit_score(options: impl Into<SubmitScoreOptions>) -> Result<(), Error> {
        run_value_unit(options, game_connect_submit_score).await
    }

    /// Method to unlock an achievement
    #[cfg(all(feature = "game_plugin", any(feature = "ios", feature = "android")))]
    pub async fn unlock_achievement(
        options: impl Into<UnlockAchievementOptions>,
    ) -> Result<(), Error> {
        run_value_unit(options, game_connect_unlock_achievement).await
    }

    /// Method to increment the progress of an achievement
    #[cfg(all(feature = "game_plugin", any(feature = "ios", feature = "android")))]
    pub async fn increment_achievement_progress(
        options: impl Into<IncrementAchievementOptions>,
    ) -> Result<(), Error> {
        run_value_unit(options, game_connect_increment_achievement_progress).await
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(default)]
pub struct ShowLeaderboardOptions {
    #[builder(setter(into))]
    #[serde(rename = "leaderboardID")]
    pub leaderboard_id: String,
}

impl From<&str> for ShowLeaderboardOptions {
    fn from(value: &str) -> Self {
        ShowLeaderboardOptions {
            leaderboard_id: value.to_string(),
        }
    }
}

impl From<String> for ShowLeaderboardOptions {
    fn from(leaderboard_id: String) -> Self {
        ShowLeaderboardOptions { leaderboard_id }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(default)]
pub struct SubmitScoreOptions {
    #[builder(setter(into))]
    #[serde(rename = "leaderboardID")]
    pub leaderboard_id: String,
    #[serde(rename = "totalScoreAmount")]
    pub total_score_amount: f32,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(default)]
pub struct UnlockAchievementOptions {
    #[builder(setter(into))]
    #[serde(rename = "achievementID")]
    pub achievement_id: String,
}

impl From<&str> for UnlockAchievementOptions {
    fn from(value: &str) -> Self {
        UnlockAchievementOptions {
            achievement_id: value.to_string(),
        }
    }
}

impl From<String> for UnlockAchievementOptions {
    fn from(achievement_id: String) -> Self {
        UnlockAchievementOptions { achievement_id }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(default)]
pub struct IncrementAchievementOptions {
    #[builder(setter(into))]
    #[serde(rename = "achievementID")]
    pub achievement_id: String,

    #[serde(rename = "pointsToIncrement")]
    pub points_to_increment: f32,
}

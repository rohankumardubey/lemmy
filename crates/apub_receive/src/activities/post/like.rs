use crate::{activities::post::like_or_dislike_post, inbox::new_inbox_routing::Activity};
use activitystreams::activity::kind::LikeType;
use lemmy_apub::check_is_apub_id_valid;
use lemmy_apub_lib::{verify_domains_match, PublicUrl, ReceiveActivity, VerifyActivity};
use lemmy_utils::LemmyError;
use lemmy_websocket::LemmyContext;
use url::Url;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LikePost {
  to: PublicUrl,
  pub(in crate::activities::post) object: Url,
  cc: [Url; 1],
  #[serde(rename = "type")]
  kind: LikeType,
}

#[async_trait::async_trait(?Send)]
impl VerifyActivity for Activity<LikePost> {
  async fn verify(&self, _context: &LemmyContext) -> Result<(), LemmyError> {
    verify_domains_match(&self.actor, self.id_unchecked())?;
    check_is_apub_id_valid(&self.actor, false)
  }
}

#[async_trait::async_trait(?Send)]
impl ReceiveActivity for Activity<LikePost> {
  async fn receive(
    &self,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError> {
    like_or_dislike_post(1, &self.actor, &self.inner.object, context, request_counter).await
  }
}
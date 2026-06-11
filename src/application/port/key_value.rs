use crate::application::error::AppResult;

#[async_trait::async_trait]
pub(crate) trait KeyValueStore: Send + Sync {
    async fn set_u32(key: String, value: u32) -> AppResult<()>;
    async fn get_u32(key: String) -> AppResult<Option<u32>>;
}

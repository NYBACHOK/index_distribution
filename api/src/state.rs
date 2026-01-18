use crate::StartError;

#[derive(Clone)]
pub struct AppState {}

impl AppState {
    pub async fn try_new() -> Result<Self, StartError> {
        Ok(Self {})
    }
}

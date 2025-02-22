use crate::data::profile::Profile;
use crate::data::profile_data_source::ProfileDataSource;
use std::error::Error;

pub struct ProfileRepository {
    data_source: ProfileDataSource,
}

impl ProfileRepository {
    pub fn new(data_source: ProfileDataSource) -> Self {
        Self { data_source }
    }

    pub async fn get(&self) -> Result<Profile, Box<dyn Error>> {
        self.data_source.get().await
    }
}

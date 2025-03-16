use std::error::Error;
use crate::data::location::Location;
use crate::data::location_data_source::LocationDataSource;

pub struct LocationRepository {
    data_source: LocationDataSource,
}

impl LocationRepository {
    pub fn new(data_source: LocationDataSource) -> Self {
        Self { data_source }
    }

    pub async fn get(&self, access_token: String) -> Result<Vec<Location>, Box<dyn Error>> {
        self.data_source.get(access_token).await
    }
}

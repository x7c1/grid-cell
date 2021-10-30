use crate::Spot;
use std::marker::PhantomData;

pub struct Filled;
pub struct Empty;

pub struct SpotBuilder<LAT, LON> {
    latitude: Option<f64>,
    longitude: Option<f64>,
    phantom: PhantomData<(LAT, LON)>,
}

impl Default for SpotBuilder<Empty, Empty> {
    fn default() -> Self {
        SpotBuilder {
            latitude: None,
            longitude: None,
            phantom: Default::default(),
        }
    }
}

impl SpotBuilder<Filled, Filled> {
    pub fn build(&self) -> Spot {
        Spot {
            latitude: self.latitude.unwrap(),
            longitude: self.longitude.unwrap(),
        }
    }
}

impl<A> SpotBuilder<Empty, A> {
    pub fn latitude(&mut self, latitude: f64) -> SpotBuilder<Filled, A> {
        SpotBuilder {
            latitude: Some(latitude),
            longitude: self.longitude,
            phantom: Default::default(),
        }
    }
}

impl<A> SpotBuilder<A, Empty> {
    pub fn longitude(&mut self, longitude: f64) -> SpotBuilder<A, Filled> {
        SpotBuilder {
            latitude: self.latitude,
            longitude: Some(longitude),
            phantom: Default::default(),
        }
    }
}

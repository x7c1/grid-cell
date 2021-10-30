mod level;
pub use level::{First, Fourth, Level, Second, Third};

mod spot_builder;
pub use spot_builder::SpotBuilder;

mod spot;
pub use spot::Spot;

use std::marker::PhantomData;

#[derive(Debug)]
pub struct GridCell<A: Level> {
    mesh_code: u64,
    west_longitude: f64,
    south_latitude: f64,
    phantom: PhantomData<A>,
}

#[cfg(test)]
mod tests {
    use crate::Spot;

    #[test]
    fn first_cell() {
        let mut spot = new_spot1();
        let actual = spot.first();
        let expected = 5339;
        assert_eq!(actual.mesh_code, expected);
    }

    #[test]
    fn second_cell() {
        let mut spot = new_spot1();
        let actual = spot.second();
        let expected = 533945;
        assert_eq!(actual.mesh_code, expected);
    }

    #[test]
    fn third_cell() {
        let mut spot = new_spot1();
        let actual = spot.third();
        let expected = 53394518;
        assert_eq!(actual.mesh_code, expected);
    }

    #[test]
    fn fourth_cell() {
        let mut spot = new_spot1();
        let actual = spot.fourth();
        let expected = 533945184;
        assert_eq!(actual.mesh_code, expected);
    }

    fn new_spot1() -> Spot {
        Spot::build()
            .longitude(139.733231)
            .latitude(35.680916)
            .build()
    }
}

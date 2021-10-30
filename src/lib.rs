mod level;
pub use level::{First, Fourth, Level, Second, Third};

mod spot_builder;
pub use spot_builder::SpotBuilder;

mod spot;
pub use spot::Spot;

use std::marker::PhantomData;

#[derive(Clone, Debug)]
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
        let actual = new_spot1().first_cell().mesh_code;
        let expected = 5339;
        assert_eq!(actual, expected);
    }

    #[test]
    fn second_cell() {
        let actual = new_spot1().second_cell().mesh_code;
        let expected = 533945;
        assert_eq!(actual, expected);
    }

    #[test]
    fn third_cell() {
        let actual = new_spot1().third_cell().mesh_code;
        let expected = 53394518;
        assert_eq!(actual, expected);
    }

    #[test]
    fn fourth_cell() {
        let actual = new_spot1().fourth_cell().mesh_code;
        let expected = 533945184;
        assert_eq!(actual, expected);
    }

    fn new_spot1() -> Spot {
        Spot::builder()
            .longitude(139.733231)
            .latitude(35.680916)
            .build()
    }
}

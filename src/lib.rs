mod spot_builder;
mod spot_cells;

pub use spot_cells::Spot;

#[derive(Debug)]
pub struct GridCell {
    mesh_code: u64,
    west_longitude: f64,
    south_latitude: f64,
}

#[cfg(test)]
mod tests {
    use crate::Spot;

    #[test]
    fn first_cell() {
        let mut spot = spot();
        let actual = spot.first();
        let expected = 5339;
        assert_eq!(actual.mesh_code, expected);
    }

    #[test]
    fn second_cell() {
        let mut spot = spot();
        let actual = spot.second();
        let expected = 533945;
        assert_eq!(actual.mesh_code, expected);
    }

    #[test]
    fn third_cell() {
        let mut spot = spot();
        let actual = spot.third();
        let expected = 53394518;
        assert_eq!(actual.mesh_code, expected);
    }

    #[test]
    fn fourth_cell() {
        let mut spot = spot();
        let actual = spot.fourth();
        let expected = 533945184;
        assert_eq!(actual.mesh_code, expected);
    }

    fn spot() -> Spot {
        Spot::build()
            .longitude(139.733231)
            .latitude(35.680916)
            .build()
    }
}

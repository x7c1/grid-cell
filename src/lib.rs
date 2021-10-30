mod spot_cells;

pub use spot_cells::SpotCells;

#[derive(Debug)]
pub struct GridCell {
    mesh_code: u64,
    west_longitude: f64,
    south_latitude: f64,
}

#[cfg(test)]
mod tests {
    use crate::SpotCells;

    #[test]
    fn first_cell() {
        let cells = SpotCells::new(139.733231, 35.680916);
        let actual = cells.first();
        let expected = 5339;
        assert_eq!(actual.mesh_code, expected);
    }

    #[test]
    fn second_cell() {
        let cells = SpotCells::new(139.733231, 35.680916);
        let actual = cells.second();
        let expected = 533945;
        assert_eq!(actual.mesh_code, expected);
    }

    #[test]
    fn third_cell() {
        let cells = SpotCells::new(139.733231, 35.680916);
        let actual = cells.third();
        let expected = 53394518;
        assert_eq!(actual.mesh_code, expected);
    }

    #[test]
    fn fourth_cell() {
        let cells = SpotCells::new(139.733231, 35.680916);
        let actual = cells.fourth();
        let expected = 533945184;
        assert_eq!(actual.mesh_code, expected);
    }
}

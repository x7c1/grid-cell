use crate::{First, Fourth, GridCell, Second, Third};

#[derive(Clone, Copy)]
pub struct Coordinate {
    pub longitude: f64,
    pub latitude: f64,
}

impl Coordinate {
    pub fn first_cell(&self) -> GridCell<First> {
        // lat difference of 1st cell is 40 minutes, base point is 0 degrees north-lat.
        let degree_y = 40.0 / 60.0;
        let y = (self.latitude / degree_y).floor();

        // lon difference of 1st cell is 1 degree, base point is 0 degrees east-lon.
        let east = self.longitude.floor();
        let x = east - 100.0;
        GridCell {
            code: (x + 100.0 * y) as u64,
            west_longitude: east,
            south_latitude: y * degree_y,
            phantom: Default::default(),
        }
    }

    pub fn second_cell(&self, first: &GridCell<First>) -> GridCell<Second> {
        // lat difference of 2nd cell is 5 minutes, base point is south-lat of 1st cell.
        let degree_y = 5.0 / 60.0;
        let y = ((self.latitude - first.south_latitude) / degree_y).floor();

        // lon difference of 2nd cell is 7 minutes 30 seconds, base point is west-lon of 1st cell.
        let degree_x = 7.5 / 60.0;
        let x = ((self.longitude - first.west_longitude) / degree_x).floor();

        GridCell {
            code: first.code * 100 + (y * 10.0 + x) as u64,
            west_longitude: first.west_longitude + x * degree_x,
            south_latitude: first.south_latitude + y * degree_y,
            phantom: Default::default(),
        }
    }

    pub fn third_cell(&self, second: &GridCell<Second>) -> GridCell<Third> {
        // lat difference of 3rd cell is 30 seconds, base point is south-lat of 2nd cell.
        let degree_y = 30.0 / 60.0 / 60.0;
        let y = ((self.latitude - second.south_latitude) / degree_y).floor();

        // lon difference of 3rd cell is 45 seconds, base point is west-lon of 2nd cell.
        let degree_x = 45.0 / 60.0 / 60.0;
        let x = ((self.longitude - second.west_longitude) / degree_x).floor();

        GridCell {
            code: second.code * 100 + (y * 10.0 + x) as u64,
            west_longitude: second.west_longitude + x * degree_x,
            south_latitude: second.south_latitude + y * degree_y,
            phantom: Default::default(),
        }
    }

    pub fn fourth_cell(&self, third: &GridCell<Third>) -> GridCell<Fourth> {
        // lat difference of 4th cell is 15 seconds, base point is south-lat of 3rd cell.
        let degree_y = 15.0 / 60.0 / 60.0;
        let y = ((self.latitude - third.south_latitude) / degree_y).floor();

        // lon difference of 4th cell is 22.5 seconds, base point is west-lon of 3rd cell.
        let degree_x = 22.5 / 60.0 / 60.0;
        let x = ((self.longitude - third.west_longitude) / degree_x).floor();

        // suffix of 4th cell is (SW:1, SE:2, NW:3, NE:4).
        let code = (x + 1.0) + y * 2.0;
        GridCell {
            code: third.code * 10 + (code as u64),
            west_longitude: third.west_longitude + x * degree_x,
            south_latitude: third.south_latitude + y * degree_y,
            phantom: Default::default(),
        }
    }
}

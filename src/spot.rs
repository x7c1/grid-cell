use crate::spot_builder::{Empty, SpotBuilder};
use crate::{First, Fourth, GridCell, Second, Third};

pub struct Spot {
    pub longitude: f64,
    pub latitude: f64,
    pub(crate) first: Option<GridCell<First>>,
    pub(crate) second: Option<GridCell<Second>>,
    pub(crate) third: Option<GridCell<Third>>,
    pub(crate) fourth: Option<GridCell<Fourth>>,
}

impl Spot {
    pub fn build() -> SpotBuilder<Empty, Empty> {
        SpotBuilder::default()
    }

    pub fn first(&mut self) -> &GridCell<First> {
        if self.first.is_none() {
            self.first = Some(self.create_first())
        }
        self.first.as_ref().unwrap()
    }

    pub fn second(&mut self) -> &GridCell<Second> {
        if self.second.is_none() {
            self.second = Some(self.create_second())
        }
        self.second.as_ref().unwrap()
    }

    pub fn third(&mut self) -> &GridCell<Third> {
        if self.third.is_none() {
            self.third = Some(self.create_third())
        }
        self.third.as_ref().unwrap()
    }

    pub fn fourth(&mut self) -> &GridCell<Fourth> {
        if self.fourth.is_none() {
            self.fourth = Some(self.create_fourth())
        }
        self.fourth.as_ref().unwrap()
    }
}

impl Spot {
    fn create_first(&mut self) -> GridCell<First> {
        // lat difference of 1st cell is 40 minutes, base point is 0 degrees north-lat.
        let degree_y = 40.0 / 60.0;
        let y = (self.latitude / degree_y).floor();

        // lon difference of 1st cell is 1 degree, base point is 0 degrees east-lon.
        let east = self.longitude.floor();
        let x = east - 100.0;
        GridCell {
            mesh_code: (x + 100.0 * y) as u64,
            west_longitude: east,
            south_latitude: y * degree_y,
            phantom: Default::default(),
        }
    }

    fn create_second(&mut self) -> GridCell<Second> {
        let (latitude, longitude) = (self.latitude, self.longitude);
        let first = self.first();

        // lat difference of 2nd cell is 5 minutes, base point is south-lat of 1st cell.
        let degree_y = 5.0 / 60.0;
        let y = ((latitude - first.south_latitude) / degree_y).floor();

        // lon difference of 2nd cell is 7 minutes 30 seconds, base point is west-lon of 1st cell.
        let degree_x = 7.5 / 60.0;
        let x = ((longitude - first.west_longitude) / degree_x).floor();

        GridCell {
            mesh_code: first.mesh_code * 100 + (y * 10.0 + x) as u64,
            west_longitude: first.west_longitude + x * degree_x,
            south_latitude: first.south_latitude + y * degree_y,
            phantom: Default::default(),
        }
    }

    fn create_third(&mut self) -> GridCell<Third> {
        let (latitude, longitude) = (self.latitude, self.longitude);
        let second = self.second();

        // lat difference of 3rd cell is 30 seconds, base point is south-lat of 2nd cell.
        let degree_y = 30.0 / 60.0 / 60.0;
        let y = ((latitude - second.south_latitude) / degree_y).floor();

        // lon difference of 3rd cell is 45 seconds, base point is west-lon of 2nd cell.
        let degree_x = 45.0 / 60.0 / 60.0;
        let x = ((longitude - second.west_longitude) / degree_x).floor();

        GridCell {
            mesh_code: second.mesh_code * 100 + (y * 10.0 + x) as u64,
            west_longitude: second.west_longitude + x * degree_x,
            south_latitude: second.south_latitude + y * degree_y,
            phantom: Default::default(),
        }
    }

    fn create_fourth(&mut self) -> GridCell<Fourth> {
        let (latitude, longitude) = (self.latitude, self.longitude);
        let third = self.third();

        // lat difference of 4th cell is 15 seconds, base point is south-lat of 3rd cell.
        let degree_y = 15.0 / 60.0 / 60.0;
        let y = ((latitude - third.south_latitude) / degree_y).floor();

        // lon difference of 4th cell is 22.5 seconds, base point is west-lon of 3rd cell.
        let degree_x = 22.5 / 60.0 / 60.0;
        let x = ((longitude - third.west_longitude) / degree_x).floor();

        // suffix of 4th cell is (SW:1, SE:2, NW:3, NE:4).
        let code = (x + 1.0) + y * 2.0;
        GridCell {
            mesh_code: third.mesh_code * 10 + (code as u64),
            west_longitude: third.west_longitude + x * degree_x,
            south_latitude: third.south_latitude + y * degree_y,
            phantom: Default::default(),
        }
    }
}

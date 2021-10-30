use crate::{Coordinate, Empty, First, Fourth, GridCell, Second, SpotBuilder, Third};

pub struct Spot {
    pub coordinate: Coordinate,
    pub(crate) first: Option<GridCell<First>>,
    pub(crate) second: Option<GridCell<Second>>,
    pub(crate) third: Option<GridCell<Third>>,
    pub(crate) fourth: Option<GridCell<Fourth>>,
}

impl Spot {
    pub fn builder() -> SpotBuilder<Empty, Empty> {
        SpotBuilder::default()
    }

    pub fn as_first_cell(&mut self) -> &GridCell<First> {
        if self.first.is_none() {
            self.first = Some(self.coordinate.first_cell());
        }
        self.first.as_ref().unwrap()
    }

    pub fn as_second_cell(&mut self) -> &GridCell<Second> {
        if self.second.is_none() {
            let coordinate = self.coordinate;
            self.second = Some(coordinate.second_cell(self.as_first_cell()));
        }
        self.second.as_ref().unwrap()
    }

    pub fn as_third_cell(&mut self) -> &GridCell<Third> {
        if self.third.is_none() {
            let coordinate = self.coordinate;
            self.third = Some(coordinate.third_cell(self.as_second_cell()))
        }
        self.third.as_ref().unwrap()
    }

    pub fn as_fourth_cell(&mut self) -> &GridCell<Fourth> {
        if self.fourth.is_none() {
            let coordinate = self.coordinate;
            self.fourth = Some(coordinate.fourth_cell(self.as_third_cell()))
        }
        self.fourth.as_ref().unwrap()
    }

    pub fn to_first_cell(mut self) -> GridCell<First> {
        self.as_first_cell();
        self.first.unwrap()
    }

    pub fn to_second_cell(mut self) -> GridCell<Second> {
        self.as_second_cell();
        self.second.unwrap()
    }

    pub fn to_third_cell(mut self) -> GridCell<Third> {
        self.as_third_cell();
        self.third.unwrap()
    }

    pub fn to_fourth_cell(mut self) -> GridCell<Fourth> {
        self.as_fourth_cell();
        self.fourth.unwrap()
    }
}

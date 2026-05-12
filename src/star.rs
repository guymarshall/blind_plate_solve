use crate::coordinates::Coordinates;

#[derive(Debug)]
pub(crate) struct Star {
    pub(crate) coordinates: Coordinates,
    pub(crate) size: i32,
}

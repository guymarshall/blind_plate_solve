use crate::{dimensions::Dimensions, star::Star};

#[derive(Debug)]
pub(crate) struct Image {
    pub(crate) dimensions: Dimensions,
    pub(crate) stars: Vec<Star>,
}

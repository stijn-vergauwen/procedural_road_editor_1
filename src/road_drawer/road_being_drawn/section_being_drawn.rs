use crate::{
    road::road_section::{RequestedRoadSection, RoadSectionVariant},
    utility::circular_arc::CircularArc,
};

use super::section_end_being_drawn::SectionEndBeingDrawn;

#[derive(Clone, Debug)]
pub struct SectionBeingDrawn {
    pub ends: [SectionEndBeingDrawn; 2],
    pub variant: SectionBeingDrawnVariant,
}

impl SectionBeingDrawn {
    pub fn to_requested_road_section(&self) -> RequestedRoadSection {
        RequestedRoadSection {
            ends: self.ends.map(|end| end.to_requested_road_section_end()),
            variant: self.variant.to_road_section_variant(),
        }
    }

    pub fn start(&self) -> SectionEndBeingDrawn {
        self.ends[0]
    }

    pub fn end(&self) -> SectionEndBeingDrawn {
        self.ends[1]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SectionBeingDrawnVariant {
    Straight,
    Curved(Option<CircularArc>),
}

impl SectionBeingDrawnVariant {
    fn to_road_section_variant(&self) -> RoadSectionVariant {
        match self {
            SectionBeingDrawnVariant::Straight => RoadSectionVariant::Straight,
            SectionBeingDrawnVariant::Curved(circular_arc) => RoadSectionVariant::Curved(
                circular_arc
                    .expect("CircularArc should be Some before converting to RoadSectionVariant."),
            ),
        }
    }
}

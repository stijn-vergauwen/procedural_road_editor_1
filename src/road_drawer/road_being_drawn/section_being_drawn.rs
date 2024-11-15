use bevy::prelude::*;

use crate::{
    road::road_section::{RequestedRoadSection, RequestedRoadSectionEnd, RoadSectionVariant},
    utility::circular_arc::CircularArc,
};

use super::{get_direction_from_to, section_end_being_drawn::SectionEndBeingDrawn};

#[derive(Clone, Copy, Debug)]
pub struct SectionBeingDrawn {
    pub ends: [SectionEndBeingDrawn; 2],
    pub variant: SectionBeingDrawnVariant,
}

impl SectionBeingDrawn {
    pub fn to_requested_road_section(
        &self,
    ) -> Result<RequestedRoadSection, SectionBeingDrawnError> {
        let road_section_variant = self.variant.to_road_section_variant()?;

        let requested_ends = match road_section_variant {
            RoadSectionVariant::Straight => calculate_straight_road_section_ends(self)?,
            RoadSectionVariant::Curved(circular_arc) => {
                calculate_curved_road_section_ends(self.ends, circular_arc)
            }
        };

        Ok(RequestedRoadSection {
            ends: requested_ends,
            variant: road_section_variant,
        })
    }

    pub fn start(&self) -> SectionEndBeingDrawn {
        self.ends[0]
    }

    pub fn end(&self) -> SectionEndBeingDrawn {
        self.ends[1]
    }

    /// Returns the straight direction from this sections start to end points, or Err if it's invalid.
    pub fn straight_section_direction(&self) -> Result<Dir3, SectionBeingDrawnError> {
        get_direction_from_to(
            self.start().snapped_position(),
            self.end().snapped_position(),
        )
        .map_err(|_| SectionBeingDrawnError::InvalidSectionLength)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SectionBeingDrawnVariant {
    Straight,
    Curved(CurvedSectionBeingDrawn),
}

impl SectionBeingDrawnVariant {
    fn to_road_section_variant(&self) -> Result<RoadSectionVariant, SectionBeingDrawnError> {
        Ok(match self {
            SectionBeingDrawnVariant::Straight => RoadSectionVariant::Straight,
            SectionBeingDrawnVariant::Curved(curved_section) => {
                RoadSectionVariant::Curved(curved_section.circular_arc?)
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CurvedSectionBeingDrawn {
    pub start_direction: Result<Dir3, SectionBeingDrawnError>,
    pub circular_arc: Result<CircularArc, SectionBeingDrawnError>,
}

impl CurvedSectionBeingDrawn {
    pub fn empty() -> Self {
        Self {
            start_direction: Err(SectionBeingDrawnError::InvalidSectionLength),
            circular_arc: Err(SectionBeingDrawnError::InvalidCurve),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SectionBeingDrawnError {
    InvalidSectionLength,
    InvalidCurve,
}

// Utility

fn calculate_straight_road_section_ends(
    section_being_drawn: &SectionBeingDrawn,
) -> Result<[RequestedRoadSectionEnd; 2], SectionBeingDrawnError> {
    let section_direction = section_being_drawn.straight_section_direction()?;

    Ok([
        section_being_drawn.ends[0].to_requested_road_section_end(-section_direction),
        section_being_drawn.ends[1].to_requested_road_section_end(section_direction),
    ])
}

fn calculate_curved_road_section_ends(
    ends_being_drawn: [SectionEndBeingDrawn; 2],
    circular_arc: CircularArc,
) -> [RequestedRoadSectionEnd; 2] {
    [
        ends_being_drawn[0]
            .to_requested_road_section_end(circular_arc.outwards_start_transform().forward()),
        ends_being_drawn[1]
            .to_requested_road_section_end(circular_arc.outwards_end_transform().forward()),
    ]
}

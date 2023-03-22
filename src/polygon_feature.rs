use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;

pub enum Rule {
    Boolean(bool),
    Values {
        included: Option<HashSet<&'static str>>,
        excluded: Option<HashSet<&'static str>>,
    },
}

/*
 * This list is a translation of Martin Raifer's rules list in osmtogeojson.
 * https://github.com/tyrasd/osmtogeojson/blob/2.0.0/polygon_features.json
 */

lazy_static! {
    pub static ref POLYGON_FEATURE_RULES: HashMap<&'static str, Rule> = HashMap::from([
        ("building", Rule::Boolean(true)),
        (
            "highway",
            Rule::Values {
                included: Some(HashSet::from(["services", "rest_area", "escape",])),
                excluded: None
            }
        ),
        (
            "natural",
            Rule::Values {
                included: None,
                excluded: Some(HashSet::from(["coastline", "ridge", "arete", "tree_row",])),
            }
        ),
        ("landuse", Rule::Boolean(true)),
        (
            "waterway",
            Rule::Values {
                included: Some(HashSet::from(["riverbank", "dock", "boatyard", "dam",])),
                excluded: None
            }
        ),
        ("amenity", Rule::Boolean(true)),
        ("leisure", Rule::Boolean(true)),
        (
            "barrier",
            Rule::Values {
                included: Some(HashSet::from([
                    "city_wall",
                    "ditch",
                    "hedge",
                    "retaining_wall",
                    "wall",
                    "spikes",
                ])),
                excluded: None
            }
        ),
        (
            "railway",
            Rule::Values {
                included: Some(HashSet::from([
                    "station",
                    "turntable",
                    "roundhouse",
                    "platform",
                ])),
                excluded: None
            }
        ),
        ("area", Rule::Boolean(true)),
        ("boundary", Rule::Boolean(true)),
        (
            "man_made",
            Rule::Values {
                included: None,
                excluded: Some(HashSet::from(["cutline", "embankment", "pipeline",])),
            }
        ),
        (
            "power",
            Rule::Values {
                included: Some(HashSet::from([
                    "generator",
                    "station",
                    "sub_station",
                    "transformer",
                ])),
                excluded: None
            }
        ),
        ("place", Rule::Boolean(true)),
        ("shop", Rule::Boolean(true)),
        (
            "aeroway",
            Rule::Values {
                included: None,
                excluded: Some(HashSet::from(["taxiway"])),
            }
        ),
        ("tourism", Rule::Boolean(true)),
        ("historic", Rule::Boolean(true)),
        ("public_transportation", Rule::Boolean(true)),
        ("office", Rule::Boolean(true)),
        ("building:part", Rule::Boolean(true)),
        ("military", Rule::Boolean(true)),
        ("ruins", Rule::Boolean(true)),
        ("area:highway", Rule::Boolean(true)),
        ("craft", Rule::Boolean(true)),
    ]);
}

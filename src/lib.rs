pub mod polygon_feature;

use std::{borrow::Cow, collections::HashMap};

use geo_types::{coord, Coord, Geometry, LineString, Point, Polygon};
use polygon_feature::{Rule, POLYGON_FEATURE_RULES};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct OSM3S {
    timestamp_osm_base: String,
    timestamp_areas_base: String,
    copyright: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct RelationMember {
    #[serde(rename = "type")]
    _type: String,
    #[serde(rename = "ref")]
    _ref: u64,
    role: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ElementType {
    #[serde(rename = "node")]
    Node { lat: f64, lon: f64 },
    #[serde(rename = "way")]
    Way { nodes: Vec<u64> },
    #[serde(rename = "relation")]
    Relation { members: Vec<RelationMember> },
}

#[derive(Clone, Serialize, Deserialize)]
struct Element {
    #[serde(rename = "type", flatten)]
    _type: ElementType,
    id: u64,
    // Also: timestamp, version, changeset, user, uid
    tags: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    version: f32,
    generator: String,
    osm3s: OSM3S,
    elements: Vec<Element>,
}

#[derive(Debug)]
pub enum Error {
    IncorrectType(&'static str),
    NodeNotFound,
}

impl Response {
    pub fn element_map(&self) -> HashMap<u64, Cow<Element>> {
        self.elements
            .iter()
            .map(|e| (e.id, Cow::Borrowed(e)))
            .collect()
    }
}

fn is_polygon_feature(tags: &HashMap<String, String>) -> bool {
    if tags.get("area").unwrap_or(&String::from("yes")) == "no" {
        return false;
    }
    for (tag, value) in tags.iter() {
        let rule = POLYGON_FEATURE_RULES.get(tag.as_str());
        if let Some(rule) = rule {
            if value == "no" {
                continue;
            }
            match rule {
                Rule::Boolean(true) => return true,
                Rule::Values {
                    included: Some(included),
                    ..
                } => {
                    if included.get(value.as_str()).is_some() {
                        return true;
                    }
                }
                Rule::Values {
                    excluded: Some(excluded),
                    ..
                } => {
                    if excluded.get(value.as_str()).is_none() {
                        return true;
                    }
                }
                _ => continue,
            }
        }
    }
    false
}

impl Element {
    fn to_geo(&self, element_map: HashMap<u64, Cow<Element>>) -> Result<Geometry, Error> {
        match &self._type {
            ElementType::Way { nodes } => {
                let points = nodes
                    .iter()
                    .map(
                        |node| match element_map.get(node).ok_or(Error::NodeNotFound)?._type {
                            ElementType::Node { lat, lon } => Ok(coord! { x: lon, y: lat }),
                            _ => Err(Error::IncorrectType(
                                "Way must be composed of node types only.",
                            )),
                        },
                    )
                    .collect::<Result<Vec<Coord>, Error>>()?;
                let line = LineString::new(points);
                if let Some(tags) = &self.tags {
                    if nodes.first().unwrap() == nodes.last().unwrap() && is_polygon_feature(tags) {
                        return Ok(Polygon::new(line, vec![]).into());
                    }
                }
                Ok(line.into())
            }
            ElementType::Node { lat, lon } => Ok(Point::new(*lon, *lat).into()),
            _ => {
                return Err(Error::IncorrectType(
                    "Can't generate geo item from anything except Way",
                ))
            }
        }
    }
}

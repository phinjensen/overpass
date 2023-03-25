pub mod polygon_feature;

use std::{borrow::Cow, collections::HashMap, convert::TryInto};

use geo_types::{coord, Coord, Geometry, LineString, Point, Polygon};
use polygon_feature::{Rule, POLYGON_FEATURE_RULES};
use serde::{Deserialize, Serialize};

type WayNodes = Vec<u64>;

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
    Way { nodes: WayNodes },
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
    InvalidData,
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

/// Returns disparate ways joined into LineStrings (including closed rings where possible)
fn join_ways(
    ways: Vec<Element>,
    node_map: &HashMap<u64, Cow<Element>>,
) -> Result<Vec<LineString>, Error> {
    let mut result: HashMap<u64, Vec<Coord>> = HashMap::new();
    for way in ways {
        if let ElementType::Way { nodes } = way._type {
            match (nodes.first(), nodes.last()) {
                (Some(first), Some(last)) => {
                    if first == last {
                        result.insert(*last, get_nodes(&nodes, node_map)?);
                    } else if let Some(coords) = result.get_mut(first) {
                        coords.extend_from_slice(&get_nodes(&nodes, node_map)?[1..]);
                    }
                }
                _ => (),
            }
        }
    }
    Ok(result
        .into_values()
        .map(|nodes| LineString::new(nodes))
        .collect())
}

fn get_nodes(nodes: &WayNodes, node_map: &HashMap<u64, Cow<Element>>) -> Result<Vec<Coord>, Error> {
    nodes
        .iter()
        .map(
            |node| match node_map.get(node).ok_or(Error::NodeNotFound)?._type {
                ElementType::Node { lat, lon } => Ok(coord! { x: lon, y: lat }),
                _ => Err(Error::IncorrectType(
                    "Way must be composed of node types only.",
                )),
            },
        )
        .collect()
}

impl Element {
    /// Converts a given Overpass API element into the appropriate geo_type.
    ///
    /// - A Node is converted into a Point.
    /// - A Way is converted
    ///     - into a Polygon if it fits the [polygon feature](https://wiki.openstreetmap.org/wiki/Overpass_turbo/Polygon_Features) rules
    ///     - into a LineString otherwise
    /// - A Relation is converted
    ///
    fn to_geo(&self, element_map: &HashMap<u64, Cow<Element>>) -> Result<Geometry, Error> {
        match &self._type {
            ElementType::Node { lat, lon } => Ok(Point::new(*lon, *lat).into()),
            ElementType::Way { nodes } => {
                // A way should be composed of nodes that we can convert to geo_types::Coord
                let points = get_nodes(nodes, element_map)?;

                // A way should also be longer than one node; otherwise it's not a string or
                // polygon
                if points.len() <= 1 {
                    return Err(Error::InvalidData);
                }

                // Default to a LineString, but wrap it in a Polygon if it fits the requirements
                let line = LineString::new(points);
                if let Some(tags) = &self.tags {
                    if nodes.first().unwrap() == nodes.last().unwrap() && is_polygon_feature(tags) {
                        return Ok(Polygon::new(line, vec![]).into());
                    }
                }
                Ok(line.into())
            }
            ElementType::Relation { members } => {
                // ignore if no members
                //
                // if type=route or type=waterway
                //  Make it a GeometryCollection
                // if type=multipolygon or type=boundary
                //  make it a multipolygon:
                //      count number of outer items (also throw/show an error if there is something with
                //      a role other than "inner" or "outer")
                //      ignore if no outer items
                //      create the multipolygon, conducting validity checks:
                //          looks for undefined/missing ways
                //              looks for undefined/missing nodes
                //          get outer and inner rings
                //          join all ways to rings
                //              if any aren't rings, ignore them and give a warning
                //          look for inner rings with no outer ring (ignore them with a warning)
                //          remove rings with < 4 nodes (3+connecting)
                //          ignore polygons without coordinates (possible? Or just by filtering
                //              other stuff out?)
                //
                //
                panic!("Relations not yet supported!");
            }
        }
    }
}

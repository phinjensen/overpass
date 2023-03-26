#[cfg(test)]
use crate::*;
use geo_types::LineString;

// Test the coord_in_polygon_ls function in src/lib.rs
#[test]
fn test_coord_in_polygon_ls() {
    let polygon = LineString(vec![
        coord! { x: 0.0, y: 0.0 },
        coord! { x: 1.0, y: 0.0 },
        coord! { x: 1.0, y: 1.0 },
        coord! { x: 0.0, y: 1.0 },
        coord! { x: 0.0, y: 0.0 },
    ]);
    let p = coord! { x: 0.5, y: 0.5 };
    assert!(coord_in_polygon_ls(&p, &polygon));
}

// Test the coord_in_polygon_ls function in src/lib.rs
#[test]
fn test_coord_in_polygon_ls_not() {
    let polygon = LineString(vec![
        coord! { x: 0.0, y: 0.0 },
        coord! { x: 1.0, y: 0.0 },
        coord! { x: 1.0, y: 1.0 },
        coord! { x: 0.0, y: 1.0 },
        coord! { x: 0.0, y: 0.0 },
    ]);
    let p = coord! { x: 2.0, y: 2.0 };
    assert!(!coord_in_polygon_ls(&p, &polygon));
}

// Test the coord_in_polygon_ls function in src/lib.rs
#[test]
fn test_coord_in_polygon_ls_on_edge() {
    let polygon = LineString(vec![
        coord! { x: 0.0, y: 0.0 },
        coord! { x: 1.0, y: 0.0 },
        coord! { x: 1.0, y: 1.0 },
        coord! { x: 0.0, y: 1.0 },
        coord! { x: 0.0, y: 0.0 },
    ]);
    let p = coord! { x: 0.5, y: 0.0 };
    assert!(coord_in_polygon_ls(&p, &polygon));
    let p = coord! { x: 0.0, y: 0.5 };
    assert!(coord_in_polygon_ls(&p, &polygon));
}

// Test the coord_in_polygon_ls function in src/lib.rs
#[test]
fn test_coord_in_polygon_ls_on_vertex() {
    let polygon = LineString(vec![
        coord! { x: 0.0, y: 0.0 },
        coord! { x: 1.0, y: 0.0 },
        coord! { x: 1.0, y: 1.0 },
        coord! { x: 0.0, y: 1.0 },
        coord! { x: 0.0, y: 0.0 },
    ]);
    let p = coord! { x: 0.0, y: 0.0 };
    assert!(coord_in_polygon_ls(&p, &polygon));
}

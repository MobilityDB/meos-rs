use chrono::{NaiveDateTime, TimeZone, Utc};
use geos::Geometry;
use meos::{meos_initialize, TGeomPointSequence, TGeoTrait};
use polars::prelude::*;
use std::{collections::HashMap, process};

fn main() {
    meos_initialize();

    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("data/ais_instants.csv".into()))
        .unwrap_or_else(|_| {
            println!("Error opening input file");
            process::exit(1);
        })
        .finish()
        .unwrap();

    let t_col = df.column("t").unwrap().str().unwrap();
    let mmsi_col = df.column("mmsi").unwrap().i64().unwrap();
    let lat_col = df.column("latitude").unwrap().f64().unwrap();
    let lon_col = df.column("longitude").unwrap().f64().unwrap();

    // Group instants by MMSI
    let mut ship_points: HashMap<i64, Vec<(Geometry, chrono::DateTime<Utc>)>> = HashMap::new();
    for i in 0..df.height() {
        let Some(mmsi) = mmsi_col.get(i) else {
            continue;
        };
        let Some(t_str) = t_col.get(i) else { continue };
        let Some(lat) = lat_col.get(i) else { continue };
        let Some(lon) = lon_col.get(i) else { continue };

        let t = Utc
            .from_utc_datetime(&NaiveDateTime::parse_from_str(t_str, "%Y-%m-%d %H:%M:%S").unwrap());
        let geom = Geometry::new_from_wkt(&format!("POINT({lon} {lat})")).unwrap();
        ship_points.entry(mmsi).or_default().push((geom, t));
    }

    // Build a trajectory per ship (need at least 2 points)
    let trajectories: HashMap<i64, TGeomPointSequence> = ship_points
        .into_iter()
        .filter(|(_, pts)| pts.len() >= 2)
        .map(|(mmsi, pts)| (mmsi, pts.into_iter().collect()))
        .collect();

    println!("Built {} trajectories", trajectories.len());

    // Find the pair of ships with the closest nearest approach distance
    let ships: Vec<_> = trajectories.iter().collect();
    let (mut min_dist, mut min_pair) = (f64::MAX, (0i64, 0i64));

    for i in 0..ships.len() {
        for j in (i + 1)..ships.len() {
            let (mmsi_a, traj_a) = ships[i];
            let (mmsi_b, traj_b) = ships[j];
            let dist = traj_a.nearest_approach_distance(&traj_b.clone().into());
            if dist < min_dist {
                min_dist = dist;
                min_pair = (*mmsi_a, *mmsi_b);
            }
        }
    }

    println!(
        "Closest ships: MMSI {} and {} with nearest approach distance: {:.4}°",
        min_pair.0, min_pair.1, min_dist
    );
}

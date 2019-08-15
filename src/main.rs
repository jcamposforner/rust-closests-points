use geo::prelude::*;
use geo::Point;

fn main() {
    let p = |x, y| Point::<f64>::from((x, y));

    let place = p(-74.006, 40.7128);
    let players = vec![
        p(-0.1278, 51.5074),
        p(-73.1278, 40.5074),
        p(-65.1278, 51.5074),
    ];

    let mut distances = Vec::new();

    for player in players.into_iter() {
        distances.push(player.haversine_distance(&place));
    }

    println!("{}", distances.iter().cloned().fold(0. / 0., f64::min));
}

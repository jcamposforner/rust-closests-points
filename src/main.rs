use geo::prelude::*;
use geo::Point;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;

#[derive(Debug)]
struct Player {
    point: Point::<f64>,
    name: String,
}

impl Player {
    fn new(point: Point::<f64>, name: &'static str) -> Self {
        Player {
            point,
            name: name.to_string(),
        }
    }
}

fn main() {
    let p = |x, y| Point::<f64>::from((x, y));

    let places = vec![
        p(-74.006, 40.7128),
        p(-68.1278, 53.5074),
        p(-61.1278, 58.5074),
    ];
    let players = vec![
        Player::new(p(-0.1278, 51.5074), "Jesus"),
        Player::new(p(61.1278, 58.5074), "a"),
        Player::new(p(-61.1278, 58.5074), "d"),
    ];

    let mut distances: HashMap<String, Vec<f64>> = HashMap::new();

    for player in players.into_iter() {
        let mut vec_of_points: Vec<f64> = Vec::new();
        let places = places.clone();
        
        for place in places.into_iter() {
            match distances.get(&player.name) {
                Some(_) => {
                    vec_of_points.push(player.point.haversine_distance(&place));
                    distances.insert(
                        player.name.clone(),
                        vec_of_points.clone(),
                    );
                }
                None => {
                    vec_of_points.push(player.point.haversine_distance(&place));
                    distances.insert(
                        player.name.clone(),
                        vec_of_points.clone(),
                    );
                }
            }
        }
    }

    for (player, mut distance) in distances {
        println!("{} closed distance {}", player, distance.iter().cloned().fold(0. / 0., f64::min));
        distance.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
        println!("{:?}", distance);
    }
}

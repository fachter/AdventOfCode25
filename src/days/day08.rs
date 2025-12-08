use std::collections::HashMap;

pub fn run_playground() {
    println!("--- Day 8: Playground ---");

    day08("08-test");
    println!("--------------------\n\n");
}

fn day08(file_name: &str) {
    let lines = crate::utils::file::read_lines(file_name);

    let mut points: Vec<Point3D> = Vec::new();
    let mut neighbors: Vec<Vec<Neighbor>> = Vec::new();
    let mut flattened_neighbors: Vec<Neighbor> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let coords: Vec<f64> = line
            .split(',')
            .map(|s| s.trim().parse().expect("Could not parse to f64"))
            .collect();
        if coords.len() != 3 {
            panic!("Each line must contain exactly three coordinates");
        }
        let point = Point3D {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        };

        println!("Point: ({}, {}, {})", point.x, point.y, point.z);

        let mut this_neighbors: Vec<Neighbor> = Vec::new();
        for (j, existing) in points.iter().enumerate() {
            let distance = point.distance_to(existing);
            this_neighbors.push(Neighbor {
                this: i,
                index: j,
                distance,
            });
        }
        this_neighbors.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        flattened_neighbors.extend(this_neighbors.iter().cloned());
        neighbors.push(this_neighbors);
        points.push(point);
        // Process the point as needed
    }

    flattened_neighbors.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    println!("Total points processed: {}", points.len());
    println!("Total neighbor entries: {}", flattened_neighbors.len());
    println!(
        "First 10 flattened neighbors: {:?}",
        &flattened_neighbors[..10.min(flattened_neighbors.len())]
    );

    let mut clusters: HashMap<usize, u32> = HashMap::new();
    let mut cluster_id = 0;

    for neighbor in flattened_neighbors.iter().take(10) {
        println!(
            "Point {} is close to Point {} with distance {:.4}",
            neighbor.this, neighbor.index, neighbor.distance
        );

        let ca = *clusters.get(&neighbor.this).unwrap_or(&cluster_id);
        let cb = *clusters.get(&neighbor.index).unwrap_or(&cluster_id);

        let m = ca.min(cb);
        clusters.insert(neighbor.this, m);
        clusters.insert(neighbor.index, m);
    }
}

struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, Debug)]
struct Neighbor {
    this: usize,
    index: usize,
    distance: f64,
}

impl Point3D {
    fn distance_to(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

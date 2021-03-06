#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn haversine_distance(a: &Point, b: &Point) -> f64 {
    let r: f64 = 6371008.7714; // IUGG  mean earth radius
    let d_lat: f64 = (b.y - a.y).to_radians();
    let d_lon: f64 = (b.x - a.x).to_radians();
    let lat1: f64 = a.y.to_radians();
    let lat2: f64 = b.y.to_radians();
    let a: f64 = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
        + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
    let c: f64 = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));
    return r * c;
}

pub fn midpoint(a: &Point, b: &Point) -> Point {
    return Point {
        x: (a.x + b.x) / 2.0,
        y: (a.y + b.y) / 2.0,
    };
}

pub fn get_point_from_line(a: &Point, b: &Point, part: f64) -> Point {
    return Point {
        x: a.x + (part * (b.x - a.x)),
        y: a.y + (part * (b.y - a.y)),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_midpoint() {
        let a = Point {
            x: 2.3484976,
            y: 48.8275185,
        };
        let b = Point {
            x: 2.3486683,
            y: 48.8275416,
        };
        assert_eq!(
            midpoint(&a, &b),
            Point {
                x: 2.34858295,
                y: 48.82753005
            }
        );
        assert_eq!(midpoint(&a, &b), get_point_from_line(&a, &b, 1.0 / 2.0));
    }

    #[test]
    fn test_a_get_point_from_line() {
        let a = Point { x: -4.0, y: 1.0 };
        let b = Point { x: 8.0, y: 7.0 };
        assert_eq!(
            get_point_from_line(&a, &b, 1.0 / 3.0),
            Point { x: 0.0, y: 3.0 }
        );
        assert_eq!(
            get_point_from_line(&a, &b, 2.0 / 3.0),
            Point { x: 4.0, y: 5.0 }
        );
    }
    #[test]
    fn test_b_get_point_from_line() {
        let a = Point {
            x: 2.3484976,
            y: 48.8275185,
        };
        let b = Point {
            x: 2.3486683,
            y: 48.8275416,
        };
        assert_eq!(
            get_point_from_line(&a, &b, 1.0 / 3.0),
            Point {
                x: 2.3485545,
                y: 48.8275262
            }
        );
        assert_eq!(
            get_point_from_line(&a, &b, 2.0 / 3.0),
            Point {
                x: 2.3486114,
                y: 48.8275339
            }
        );
    }
}

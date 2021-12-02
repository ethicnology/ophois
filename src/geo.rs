#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub fn haversine_distance(a: &Point, b: &Point) -> f32 {
    let r: f32 = 6356752.0; // earth radius in meters
    let d_lat: f32 = (b.y - a.y).to_radians();
    let d_lon: f32 = (b.x - a.x).to_radians();
    let lat1: f32 = a.y.to_radians();
    let lat2: f32 = b.y.to_radians();
    let a: f32 = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
        + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
    let c: f32 = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));
    return r * c;
}

pub fn midpoint(a: &Point, b: &Point) -> Point {
    return Point {
        x: (a.x + b.x) / 2.0,
        y: (a.y + b.y) / 2.0,
    };
}

pub fn get_point_from_line(a: &Point, b: &Point, part: f32) -> Point {
    return Point {
        x: a.x + (part * (b.x - a.x)),
        y: a.y + (part * (b.y - a.y)),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_x() {
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
                x: 2.348583,
                y: 48.82753
            }
        );
        assert_eq!(midpoint(&a, &b), get_point_from_line(&a, &b, 1.0 / 2.0));
    }

    #[test]
    fn test_y() {
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
    fn test_z() {
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
                x: 2.3485546,
                y: 48.827526
            }
        );
        //3758221295-3761637488-1/3␟lat␟48.827526␟lon␟2.3485546
        assert_eq!(
            get_point_from_line(&a, &b, 2.0 / 3.0),
            Point {
                x: 2.3486114,
                y: 48.827534
            }
        );
        //3758221295-3761637488-2/3␟lat␟48.827534␟lon␟2.3486114
    }
}

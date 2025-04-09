use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Vector2D { x: 0.0, y: 0.0 } // Return zero vector if magnitude is zero
        } else {
            Vector2D {
                x: self.x / mag,
                y: self.y / mag,
            }
        }
    }
}

impl std::ops::Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vector2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul for Vector2D {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vector2D {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl std::ops::Div for Vector2D {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Vector2D {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

// Trajectory Calculation Functions
fn calculate_trajectory(
    initial_velocity: Vector2D,
    angle: f64,
    time_step: f64,
    gravity: f64,
) -> Vec<Vector2D> {
    let mut trajectory = Vec::new();
    let mut current_position = Vector2D::new(0.0, 0.0);
    let mut velocity = initial_velocity;

    let num_steps = 100; //Adjust as needed for accuracy

    for _ in 0..num_steps {
        trajectory.push(current_position);

        // Update velocity due to gravity
        velocity.y -= gravity * time_step;

        // Update position
        current_position.x += velocity.x * time_step;
        current_position.y += velocity.y * time_step;
    }

    trajectory
}

pub fn test() {

    // Trajectory Calculation Example
    let initial_velocity = Vector2D::new(10.0, 45.0); // Initial speed and angle
    let angle: f64 = 0.0;
    let time_step = 0.1;
    let gravity = 9.81;

    let trajectory = calculate_trajectory(initial_velocity, angle, time_step, gravity);

    println!("Trajectory Points:");
    for point in &trajectory {
        println!("({}, {})", point.x, point.y);
    }
}
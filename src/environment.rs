use glam::Vec3;


#[derive(Debug)]
pub struct SkierParams {
    pub mass: f32,
    pub cd_area: f32,
    pub rho: f32,
    pub mu_roll: f32,   // Coulomb friction coefficient
    pub c_visc: f32,    // viscous ground damping
    pub v_min: f32,
    pub g: Vec3,
}

// Increase power_w for faster acceleration.
// Increase mu_roll for stickier snow or if skier should slow faster on flats.
// Increase cd_area to reduce top speed (windier/bulkier skier).
// Lower c_visc for snappier responses; raise to damp oscillations.
impl Default for SkierParams {
    fn default() -> Self {
        Self {
            mass: 75.0,
            cd_area: 0.35,
            rho: 1.225,
            mu_roll: 0.03,
            c_visc: 5.0,
            v_min: 0.5,
            g: Vec3 { x: 0.0, y: -9.81, z: 0.0 },
        }
    }
}

#[derive(Debug)]
pub struct SkierState {
    pub t: f32, // global curve parameter (same t used by CubicCurve)
    pub speed: f32 // scalar speed along curve (m/s)
}

impl SkierState {
    pub fn new(t0: f32, speed0: f32) -> Self {
        Self { t: t0, speed: speed0.max(0.0) }
    }

    pub fn update(
        &mut self,
        power_w: f32,
        params: &SkierParams,
        curve_velocity: Vec3,
        curve_acceleration: Vec3,
        dt: f32,
    ) {
        // Sample derivatives at current curve parameter t
        let r1 = curve_velocity;      // dr/dt
        let r2 = curve_acceleration;  // d2r/dt2

        // Tangent, normal, curvature, and local |dr/dt| to convert between dt and arc-length
        let (tangent, normal, kappa, r1_norm) = compute_tangent_curvature(r1, r2);

        let v = self.speed.max(0.0);

        // Motor thrust F = P / max(v, v_min)
        let v_for_force = v.max(params.v_min);
        let motor_f = power_w / v_for_force;

        // Gravity component along tangent: F_g = m * g · tangent
        let fg_t = params.mass * params.g.dot(tangent);

        // Centripetal contribution to normal force: m * v^2 * κ
        let centripetal = params.mass * v * v * kappa;

        // Surface normal contribution from gravity: m * |g · normal|
        let g_normal_comp = params.mass * params.g.dot(normal); // signed
        let normal_force = (g_normal_comp.abs()) + centripetal; // magnitude

        // Rolling friction magnitude = mu_roll * normal_force
        let f_roll_mag = params.mu_roll * normal_force;
        // Direction: oppose motion along tangent
        let f_roll = if v > 0.0 { -f_roll_mag } else { 0.0 };

        // Viscous ground damping
        let f_visc = -params.c_visc * v;

        // Air drag (quadratic)
        let f_drag = -0.5 * params.rho * params.cd_area * v * v;

        // Net force along tangent (positive forward)
        let f_net = motor_f + fg_t + f_roll + f_visc + f_drag;

        // Acceleration along tangent
        let a_t = f_net / params.mass;

        // Semi-implicit Euler
        self.speed = (v + a_t * dt).max(0.0);

        // Advance curve parameter t by converting arc-length delta to parameter delta:
        // arc-length_delta = speed * dt, and ds/dt = |r'(t)|, so dt_param = arc-length_delta / |r'|
        // If r1_norm is very small, avoid division by zero.
        let arc = self.speed * dt;
        let dt_param = if r1_norm > 1e-8 { arc / r1_norm } else { 0.0 };
        self.t += dt_param;
    }
}

/// Compute curvature κ and unit tangent & normal from derivatives r' and r''.
/// r' and r'' are derivatives with respect to curve parameter t (not arc-length).
fn compute_tangent_curvature(
    r1: Vec3,
    r2: Vec3,
) -> (Vec3, Vec3, f32, f32) {
    let r1_norm = r1.length();
    let tangent = if r1_norm > 1e-8 { r1 / r1_norm } else { Vec3::X }; // fallback
    // curvature κ = |r' x r''| / |r'|^3
    let cross = r1.cross(r2);
    let kappa = cross.length() / (r1_norm * r1_norm * r1_norm + 1e-12);
    // normal direction (unit) approximately r'' projected perpendicular to tangent
    let mut normal = r2 - tangent * (tangent.dot(r2));
    let normal_norm = normal.length();
    if normal_norm > 1e-8 {
        normal /= normal_norm;
    } else {
        // fallback normal (some arbitrary perpendicular)
        normal = tangent.any_orthogonal_vector();
    }
    (tangent, normal, kappa, r1_norm)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skier() {
        let test_cases: Vec<(f32, _)> = vec![
            (0.0, "flat"),
            (0.04, "4% up"),
            (-0.04, "4% down"),
            (-0.15, "15% down"),
        ];
        let power = 120.0;

        for (slope, description) in test_cases {
            let slope_angle = slope.atan();
            let tangent = Vec3::new(slope_angle.cos(), slope_angle.sin(), 0.0).normalize();
            let curve_velocity = tangent;
            let curve_acceleration = Vec3::ZERO;

            let mut skier = SkierState::new(0.0, 0.0);
            let params = SkierParams::default();
            let dt = 0.016;

            for _ in 0..2000 {
                skier.update(power, &params, curve_velocity, curve_acceleration, dt);
            }

            println!("{}: speed = {:.2} km/h", description, skier.speed * 3.6);
        }
    }
}
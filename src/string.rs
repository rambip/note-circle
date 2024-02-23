use bevy::render::color::Color;
use bevy::ecs::component::Component;
use bevy::gizmos::gizmos::Gizmos;
use bevy::math::f32::Vec2;
use std::f32::consts::PI;

#[derive(Clone, Debug, Component)]
pub struct StringState {
    last: Vec<f32>,
    current: Vec<f32>,
}

impl StringState {
    pub fn new_sinusoid(n: usize) -> Self {
        let t = (0..n)
            .map(|x| x as f32 / n as f32);

        let signal = t
            .map(|s| if s < 3./4. {f32::sin(s*PI) + f32::sin(s*7.*PI)} else {0.});

        // let signal = t
        //     .map(|s| if s > 0.2 {f32::sin(s*PI)} else {f32::sin(s*PI)+3.*f32::sin(s*10.*PI)});

        Self {
            last: signal.clone().collect(),
            current: signal.collect(),
        }
    }

    pub fn new_flat(n: usize) -> Self {
        Self {
            last: vec![0.; n],
            current: vec![0.; n],
        }
    }

    pub fn n_samples(&self) -> usize {
        self.current.len()
    }

    pub fn step(&mut self, p: &StringParams) {
        let acceleration = compute_acceleration(p, &self);


        for i in 0..p.n_samples {
            // attention: on inverse last et current dans
            self.last[i] = 2.*self.current[i]-self.last[i] + p.dt*p.dt*acceleration[i]
        }
        
        std::mem::swap(&mut self.last, &mut self.current);
    }

    pub fn draw(&self, p: &StringParams, mut gizmos: Gizmos) {
        let n = self.current.len();
        for i in 0..n {
            let p = Vec2::new(
                -250. + i as f32 / n as f32 * p.length,
                -250. + self.current[i]*250.
            );
            gizmos.circle_2d(p, 1., Color::BLUE);
        }
        for &i in p.junctions.iter().chain(Some(&0)) {
            let p = Vec2::new(
                -250. + i as f32 / n as f32 * p.length,
                -250. 
            );
            gizmos.circle_2d(p, 1., Color::RED);
        }
    }
}

#[derive(Clone, Debug, Component)]
pub struct StringParams {
    pub length: f32,
    pub n_samples: usize,
    pub dt: f32,
    pub c: f32,
    pub junctions: Vec<usize>,
    pub spring_coeff: f32,
    pub solid_friction_coeff: f32,
    pub liquid_friction_coeff: f32,
    pub steps_per_render: usize,
}

fn compute_acceleration(p: &StringParams, state: &StringState) -> Vec<f32> {
    let dx = p.length /  p.n_samples as f32;

    let time_derivative = |i| (state.current[i] - state.last[i])/p.dt;
    
    let mut acc = vec![0.; p.n_samples];

    let safe_index = |i: i32| if i >= 0 && i < p.n_samples as i32 
        {state.current[i as usize]} 
        else {0.};

    let laplacian = |i| 
        (safe_index(i as i32+1)+safe_index(i as i32-1)-2.*safe_index(i as i32))/(dx*dx)
    ;

    for i in 0..p.n_samples {
        acc[i] = p.c*p.c*laplacian(i) - 
            p.liquid_friction_coeff*time_derivative(i)*time_derivative(i).abs();

        if p.junctions.contains(&i) || i==0 || i==p.n_samples-1{
            acc[i] += - p.spring_coeff * state.current[i]
                      - p.solid_friction_coeff * time_derivative(i)
            ;
        }
    }

    acc
}

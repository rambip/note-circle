use super::Note;
use bevy::render::color::Color;
use bevy::ecs::component::Component;
use bevy::gizmos::gizmos::Gizmos;
use bevy::math::f32::Vec2;
use std::f32::consts::PI;


const OFFSET: Vec2 = Vec2::new(300., 0.);

#[derive(Clone, Debug, Component)]
pub struct StringState {
    last: Vec<f32>,
    current: Vec<f32>,
}

impl StringState {
    pub fn add_sinusoid(&mut self, n: usize) {
        let len = self.current.len();
        assert!(len >= n);
        for i in 0..n {
            let x = i as f32 / n as f32;
            let v = f32::sin(x*PI) 
                + if x<0.3 {f32::sin(x*3.*PI)} else {0.}
                + if x<0.1 {f32::sin(x*10.*PI)} else {0.}
                ;
            self.current[i] += v;
            self.last[i] += v;
        }

    }

    pub fn new_flat(n: usize) -> Self {
        Self {
            last: vec![0.; n],
            current: vec![0.; n],
        }
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
            let p = OFFSET + Vec2::new(
                -250. + i as f32 / n as f32 * p.length,
                self.current[i]*150.
            );
            gizmos.circle_2d(p, 1., Color::BLUE);
        }
        for note in p.chord.iter() {
            let p = OFFSET + Vec2::new(
                -250. + note.relative_length() * 500.,
                0.
            );
            gizmos.circle_2d(p, 5., note.color());
        }
    }
}

#[derive(Clone, Debug, Component)]
pub struct StringParams {
    pub length: f32,
    pub n_samples: usize,
    pub dt: f32,
    pub c: f32,
    pub chord: Vec<Note>,
    pub spring_coeff: f32,
    pub solid_friction_coeff: f32,
    pub liquid_friction_coeff: f32,
    pub steps_per_render: usize,
}

impl StringParams {
    fn is_junction(&self, i: usize) -> bool {
        self.chord.iter()
            .any(|note| (note.relative_length() * 150.) as usize == i)
    }
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

        if p.is_junction(i) || i==0 || i==p.n_samples-1{
            acc[i] += - p.spring_coeff * state.current[i]
                      - p.solid_friction_coeff * time_derivative(i)
            ;
        }
    }

    acc
}

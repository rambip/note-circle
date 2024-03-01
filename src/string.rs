use super::{Note, STRING_LENGTH};
use bevy::render::color::Color;
use bevy::ecs::component::Component;
use bevy::gizmos::gizmos::Gizmos;
use bevy::math::f32::Vec2;
use std::f32::consts::PI;


const OFFSET: Vec2 = Vec2::new(300., 0.);

#[derive(Clone, Debug, Component)]
pub struct StringState {
    time: f32,
    last: Vec<f32>,
    current: Vec<f32>,
}

impl StringState {
    pub fn new_flat(n: usize) -> Self {
        Self {
            time: 0.,
            last: vec![0.; n],
            current: vec![0.; n],
        }
    }

    pub fn step(&mut self, p: &StringParams) {
        let acceleration = compute_acceleration(p, &self);

        if self.current.len() == 0 {
            return 
        }

        self.time += p.dt;

        for i in 0..p.n_samples {
            // attention: on inverse last et current dans
            self.last[i] = 2.*self.current[i]-self.last[i] + p.dt*p.dt*acceleration[i]
        }

        self.last[0] = compute_excitation(p, &self);
        
        std::mem::swap(&mut self.last, &mut self.current);
    }

    pub fn draw(&self, p: &StringParams, mut gizmos: Gizmos) {
        let n = self.current.len();
        for i in 0..n {
            let p = OFFSET + Vec2::new(
                -STRING_LENGTH/2. + i as f32 / n as f32 * p.length,
                self.current[i]*STRING_LENGTH/3.
            );
            gizmos.circle_2d(p, 1., Color::BLUE);
        }
        for note in p.chord.iter() {
            let p = OFFSET + Vec2::new(
                -STRING_LENGTH/2. + note.relative_length() * STRING_LENGTH,
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
    pub excitation_coeff: f32,
}

impl StringParams {
    fn is_junction(&self, i: usize) -> bool {
        self.chord.iter()
            .any(|note| (note.relative_length() * 150.) as usize == i)
    }
}

fn _triangle(t: f32) -> f32 {
    let u = t % 1.;
    if u < 0.5 {u-0.25} else {0.75-u}
}

fn _rectangle(t: f32) -> f32 {
    if t % 1. < 0.5 {-0.5} else {0.5}
}

fn _sawtooth(t: f32) -> f32 {
    t % 1. - 0.5
}

fn spikes(t: f32) -> f32 {
    1./(1.4 - f32::cos(2.*PI*t)) - 1.
}

fn compute_excitation(p: &StringParams, state: &StringState) -> f32 {
    let mut r = 0.;
    let f = 0.5*p.c/STRING_LENGTH;

    for note in &p.chord {
        r += p.excitation_coeff * spikes(f * state.time / note.relative_length())
    }

    r
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

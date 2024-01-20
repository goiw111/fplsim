use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use core::f32::consts::FRAC_PI_2;


#[derive(Debug, Default, Component)]
pub struct Qstate {
    m1:     f32,
    m1p:    Vect,
    m2:     f32,
    m2p:    Vect,
    m3:     f32,
    m3p:    Vect,
    m4:     f32,
    m4p:    Vect,
}

#[derive(Debug)]
pub enum BM {
    M1,
    M2,
    M3,
    M4
}

impl Qstate {
    pub fn new_with_offset(r: f32, offset: f32) -> Self {
        let initp = r*Vect::X;
        let list: Vec<Vect> = (0..=3)
            .map(|x| offset + (x as f32)*FRAC_PI_2 )
            .map(|x| Mat3::from_rotation_z(x)*initp )
            .collect();

        Self {
            m1p:    list[0],
            m2p:    list[1],
            m3p:    list[2],
            m4p:    list[3],
            ..Default::default()
        }
    }
    pub fn get(&self,m: BM) -> f32 {
        match m {
            BM::M1 => self.m1,
            BM::M2 => self.m2,
            BM::M3 => self.m3,
            BM::M4 => self.m4,
        }
    }
    pub fn set(&mut self ,m: BM, v: f32) {
        match m {
            BM::M1 => self.m1 = v,
            BM::M2 => self.m2 = v,
            BM::M3 => self.m3 = v,
            BM::M4 => self.m4 = v,
        }
    }
    pub fn get_force(&self ,m: BM) -> ExternalForce {
        match m {
            BM::M1 => 
                ExternalForce::at_point(Mat3::from_axis_angle(self.m1p, 0.1) * (self.m1 * Vect::Z), self.m1p, Vect::ZERO),
            BM::M2 => 
                ExternalForce::at_point(Mat3::from_axis_angle(self.m2p, -0.1) * (self.m2 * Vect::Z), self.m2p, Vect::ZERO),
            BM::M3 => 
                ExternalForce::at_point(Mat3::from_axis_angle(self.m3p, 0.1) * (self.m3 * Vect::Z), self.m3p, Vect::ZERO),
            BM::M4 => 
                ExternalForce::at_point(Mat3::from_axis_angle(self.m4p, -0.1) * (self.m4 * Vect::Z), self.m4p, Vect::ZERO),
        }
    }
    pub fn get_total_forces(&self) -> ExternalForce {
        [BM::M1,BM::M2,BM::M3,BM::M4]
            .into_iter()
            .map(|x| self.get_force(x))
            .fold(ExternalForce::default(),|p,x| p + x)
    }
}

#[derive(Bundle)]
pub struct PlatformBundle {
    state:          Qstate,
    body:           RigidBody,
    collider:       Collider,
    restitution:    Restitution,
    transform:      TransformBundle,
    force:          ExternalForce
}

impl PlatformBundle {
    pub fn new(r: f32, c: f32, z: f32) -> Self {
        Self {
            state:          Qstate::new_with_offset(3.0, 0.0),
            body:           RigidBody::Dynamic,
            collider:       Collider::ball(r),
            restitution:    Restitution::coefficient(c),
            transform:      TransformBundle::from(Transform::from_xyz(0.0, 0.0, z)),
            force:          ExternalForce::default()
        }
    }
}


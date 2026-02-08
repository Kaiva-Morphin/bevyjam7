use bevy::{math::{Vec2, Vec3}, time::Time};




pub trait WrappedDelta {
    fn dt(&self) -> f32;
}

impl WrappedDelta for Time {
    fn dt(&self) -> f32 {
        self.delta_secs().min(0.5)
    }
}

#[macro_export]
macro_rules! wrap {
    ($name:ident($ty:ty)) => {
        struct $name($ty);
        $crate::wrap!(@$name($ty));
    };
    ($name:ident(pub $ty:ty)) => {
        struct $name(pub $ty);
        $crate::wrap!(@$name($ty));
    };
    (pub $name:ident($ty:ty)) => {
        pub struct $name($ty);
        $crate::wrap!(@$name($ty));
    };
    (pub $name:ident(pub $ty:ty)) => {
        pub struct $name(pub $ty);
        $crate::wrap!(@$name($ty));
    };
    (@$name:ident($ty:ty)) => {
        impl std::ops::Deref for $name {
            type Target = $ty;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $ty {
                &mut self.0
            }
        }
    };
}

pub trait PingPongRem{
    fn pingpong_rem(&self, max: Self) -> Self;
}


impl PingPongRem for u32 {
    fn pingpong_rem(&self, max: Self) -> Self {
        let r = self.rem_euclid((max * 2).max(1));
        if r / max > 0 {
            max - r / max * r % max
        } else {
            r
        }
    }
}

impl PingPongRem for usize {
    fn pingpong_rem(&self, max: Self) -> Self {
        let r = self.rem_euclid((max * 2).max(1));
        if r / max > 0 {
            max - r / max * r % max
        } else {
            r
        }
    }
}

pub trait ExpDecay<T> {
    fn exp_decay(&self, b: T, decay: f32, dt: f32) -> T;
}

impl ExpDecay<f32> for f32 {
    fn exp_decay(&self, b: f32, decay: f32, dt: f32) -> f32 {
        b + (self - b) * (-decay*dt).exp()
    }
}

impl ExpDecay<Vec3> for Vec3 {
    fn exp_decay(&self, b: Vec3, decay: f32, dt: f32) -> Vec3 {
        b + (*self - b) * (-decay*dt).exp()
    }
}

impl ExpDecay<Vec2> for Vec2 {
    fn exp_decay(&self, b: Vec2, decay: f32, dt: f32) -> Vec2 {
        b + (*self - b) * (-decay*dt).exp()
    }
}

pub trait MoveTowards {
    type Output;
    fn move_towards(self, target: Self::Output, max_delta: Self::Output) -> Self::Output;
}

impl MoveTowards for f32 {
    type Output = f32;
    fn move_towards(self, target: f32, max_delta: f32) -> f32 {
        let delta = target - self;
        if delta.abs() <= max_delta {
            target
        } else {
            self + delta.signum() * max_delta
        }
    }
}

pub trait Easings {
    fn ease_out_quad(self) -> f32;
}

impl Easings for f32 {
    fn ease_out_quad(self) -> f32 {
        self * (1.0 - self) * (1.0 - self)
    }
}
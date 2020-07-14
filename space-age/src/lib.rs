#[derive(Debug)]
pub struct Duration {
    years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            years: s as f64 / 31557600f64,
        }
    }
}

pub trait Planet {
    const ORBITAL: f64;
    fn years_during(d: &Duration) -> f64 {
        d.years / Self::ORBITAL
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const ORBITAL: f64 = 0.2408467;
}
impl Planet for Venus {
    const ORBITAL: f64 = 0.61519726;
}
impl Planet for Earth {
    const ORBITAL: f64 = 1.;
}
impl Planet for Mars {
    const ORBITAL: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const ORBITAL: f64 = 11.862615;
}
impl Planet for Saturn {
    const ORBITAL: f64 = 29.447498;
}
impl Planet for Uranus {
    const ORBITAL: f64 = 84.016846;
}
impl Planet for Neptune {
    const ORBITAL: f64 = 164.79132;
}
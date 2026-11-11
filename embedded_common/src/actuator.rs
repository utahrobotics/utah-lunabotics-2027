//! Robot-agnostic actuator command shapes. Each robot crate supplies its own
//! actuator enum implementing [`ActuatorSet`].

use core::marker::PhantomData;

/// The set of actuators on one robot.
pub trait ActuatorSet: Copy + Eq + core::fmt::Display + 'static {
    const ALL: &'static [Self];
    /// Stable index of this actuator. fits in a [`FaultMask`].
    fn index(self) -> u8;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "std", derive(bincode::Encode, bincode::Decode))]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "std", derive(bincode::Encode, bincode::Decode))]
pub enum ActuatorCommand<A> {
    SetSpeed {
        actuator: A,
        speed: u16,
        direction: Direction,
    },
    SetAngle {
        actuator: A,
        angle_rad: f32,
    },
    StopAll,
}
impl<A> Default for ActuatorCommand<A> {
    fn default() -> Self {
        Self::StopAll
    }
}

/// One bit per actuator 
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "std", derive(bincode::Encode, bincode::Decode))]
pub struct FaultMask<A>(u8, PhantomData<A>);

impl<A> Default for FaultMask<A> {
    fn default() -> Self {
        Self(0, PhantomData)
    }
}

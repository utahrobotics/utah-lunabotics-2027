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

impl<A> ActuatorCommand<A> {
    /// speed is clamped to 0.0-1.0 and scaled to the full u16 range.
    pub fn set_speed(speed: f64, actuator: A, direction: Direction) -> Self {
        let speed = (speed.clamp(0.0, 1.0) * u16::MAX as f64) as u16;
        Self::SetSpeed {
            actuator,
            speed,
            direction,
        }
    }

    pub fn set_angle(actuator: A, angle_rad: f32) -> Self {
        Self::SetAngle {
            actuator,
            angle_rad,
        }
    }

    /// Scales a SetSpeed command.
    pub fn apply_speed_factor(&mut self, factor: f32) {
        if let Self::SetSpeed { speed, .. } = self {
            *speed = (factor * *speed as f32) as u16;
        }
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

impl<A: ActuatorSet> FaultMask<A> {
    pub fn from_actuators(actuators: impl IntoIterator<Item = A>) -> Self {
        let bits = actuators
            .into_iter()
            .fold(0u8, |bits, a| bits | 1 << a.index());
        Self(bits, PhantomData)
    }

    pub fn contains(self, actuator: A) -> bool {
        self.0 & 1 << actuator.index() != 0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Faulted actuators, in [`ActuatorSet::ALL`] order.
    pub fn iter(self) -> impl Iterator<Item = A> {
        A::ALL.iter().copied().filter(move |&a| self.contains(a))
    }
}

/// "none", or the faulted actuator names joined by `", "`.
impl<A: ActuatorSet> core::fmt::Display for FaultMask<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_empty() {
            return f.write_str("none");
        }
        for (i, a) in self.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{a}")?;
        }
        Ok(())
    }
}

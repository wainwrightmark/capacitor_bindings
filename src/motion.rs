use serde::{Deserialize, Serialize};

use crate::extern_functions::*;

use crate::helpers::*;

pub struct Motion;

impl Motion {
    pub async fn add_acceleration_listener<F: Fn(AccelListenerEvent) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "accel", motion_add_listener).await
    }

    pub async fn add_orientation_listener<F: Fn(RotationRate) + 'static>(
        func: F,
    ) -> Result<PluginListenerHandle, Error> {
        listen_async(func, "orientation", motion_add_listener).await
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccelListenerEvent {
    /// An object giving the acceleration of the device on the three axis X, Y and Z. Acceleration is expressed in m/s
    pub acceleration: Acceleration,

    /// An object giving the acceleration of the device on the three axis X, Y and Z with the effect of gravity. Acceleration is expressed in m/s
    pub acceleration_including_gravity: Acceleration,

    /// An object giving the rate of change of the device's orientation on the three orientation axis alpha, beta and gamma. Rotation rate is expressed in degrees per seconds.
    pub rotation_rate: RotationRate,

    /// A number representing the interval of time, in milliseconds, at which data is obtained from the device.
    interval: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RotationRate {
    /// The amount of rotation around the Z axis, in degrees per second.
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Acceleration {
    /// The amount of acceleration along the X axis in m/s.
    pub x: f64,

    /// The amount of acceleration along the Y axis in m/s.
    pub y: f64,

    /// The amount of acceleration along the Z axis in m/s.
    pub z: f64,
}

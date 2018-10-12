/* PORTIONS OF THIS FILE WERE ORIGINALLY DISTRIBUTED WITH THE FOLLOWING LICENSE

"""
MIT License
Copyright (c) 2017 Rust for Robotics Developers
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""

This file is part of "first-rust-competition", which is free software: you can
redistribute it and/or modify it under the terms of the GNU General Public
License version 3 as published by the Free Software Foundation. See
<https://www.gnu.org/licenses/> for a copy.
*/

use self::hid::*;
use super::ds::*;

pub mod hid;

/// Struct for almost any FRC legal joystick
pub struct Joystick {
    hid: GenericHIDData,
    axes: [usize; 5],
}

/// Handle input from Xbox 360 or Xbox One controllers connected to the Driver Station.
pub struct XboxController {
    hid: GenericHIDData,
}

macro_rules! define_channels {
    ($func_get:ident,$func_set:ident,$index:expr) =>
    {pub fn $func_get(&self)->usize{self.axes[$index]}
    pub fn $func_set(&mut self,new_value:usize){self.axes[$index] = new_value;}};
}

macro_rules! define_getter {
    ($fncm:ident ($rtn_type:ident) $func:ident: $button:expr) => {
    pub fn $func(&self)->Result<$rtn_type,JoystickError>{self.$fncm($button)}};
    ($fncm:ident ($rtn_type:ident) $func1:ident: $button1:expr, $($func:ident: $button:expr),+) =>
    { define_getter!($fncm ($rtn_type) $func1: $button1); define_getter!($fncm ($rtn_type) $($func: $button),+);};
}

impl GetHID for Joystick {
    fn get_hid_data(&self) -> &GenericHIDData {
        &self.hid
    }

    fn get_hid_data_mut(&mut self) -> &mut GenericHIDData {
        &mut self.hid
    }
}

impl GenericHID for Joystick {}

impl Joystick {
    pub const DEFAULT_X_AXIS: usize = 0;
    pub const DEFAULT_Y_AXIS: usize = 1;
    pub const DEFAULT_Z_AXIS: usize = 2;
    pub const DEFAULT_TWIST_AXIS: usize = 2;
    pub const DEFAULT_THROTTLE_AXIS: usize = 3;

    /// user creates a Joystick object here
    pub fn new(port: usize) -> Joystick {
        Joystick {
            hid: GenericHIDData::new(port),
            axes: [
                Joystick::DEFAULT_X_AXIS,
                Joystick::DEFAULT_Y_AXIS,
                Joystick::DEFAULT_Z_AXIS,
                Joystick::DEFAULT_TWIST_AXIS,
                Joystick::DEFAULT_THROTTLE_AXIS,
            ],
        }
    }


    //Basic getters and setters which consumed a lot of space
    define_channels! {get_x_channel,set_x_channel,0}
    define_channels! {get_y_channel,set_y_channel,1}
    define_channels! {get_z_channel,set_z_channel,2}
    define_channels! {get_twist_channel,set_twist_channel,3}
    define_channels! {get_throttle_channel,set_throttle_channel,4}
    define_getter! {get_raw_button (bool) get_trigger: 1, get_top: 2}

    /// Get the X value of the joystick. This depends on the mapping of the joystick connected to the current port.
    pub fn get_x(&self) -> Result<f32, JoystickError> {
        self.get_raw_axis(self.axes[0])
    }

    /// Get the Y value of the joystick. This depends on the mapping of the joystick connected to the current port.
    pub fn get_y(&self) -> Result<f32, JoystickError> {
        self.get_raw_axis(self.axes[1])
    }

    /// Get the z position of the HID.
    pub fn get_z(&self) -> Result<f32, JoystickError> {
        self.get_raw_axis(self.axes[2])
    }

    /// Get the twist value of the current joystick. This depends on the mapping of the joystick connected to the current port.
    pub fn get_twist(&self) -> Result<f32, JoystickError> {
        self.get_raw_axis(self.axes[3])
    }

    /// Get the throttle value of the current joystick. This depends on the mapping of the joystick connected to the current port.
    pub fn get_throttle(&self) -> Result<f32, JoystickError> {
        self.get_raw_axis(self.axes[4])
    }

    /// Get the magnitude of the direction vector formed by the joystick's current position relative to its origin.
    pub fn get_magnitude(&self) -> Result<f32, JoystickError> {
        let x: f32 = match self.get_x() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let y: f32 = match self.get_y() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok((x.powi(2) + y.powi(2)).sqrt())
    }

    /// Get the direction of the vector formed by the joystick and its origin in radians.
    pub fn get_direction(&self) -> Result<f32, JoystickError> {
        let x: f32 = match self.get_x() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let y: f32 = match self.get_y() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(x.atan2(-y))
    }
}

impl GetHID for XboxController {
    fn get_hid_data(&self) -> &GenericHIDData {
        &self.hid
    }

    fn get_hid_data_mut(&mut self) -> &mut GenericHIDData {
        &mut self.hid
    }
}

impl GenericHID for XboxController {}

impl XboxController {
    pub fn new(port: usize) -> Self {
        Self { hid: GenericHIDData::new(port) }
    }

    define_getter! {get_raw_button (bool) get_a: 1, get_b: 2, get_x: 3, get_y: 4, get_left_bumper: 5, get_right_bumper: 6, get_back: 7, get_start: 8, get_left_stick_pressed: 9, get_right_stick_pressed: 10}
    define_getter! {get_raw_axis (f32) get_left_trigger: 2, get_right_trigger: 3, get_left_x: 0, get_left_y: 10, get_right_x: 4, get_right_y: 5}
    define_getter! {get_pov (i16) get_arrow_pad: 0}
}
use crate::device::*;

#[derive(Clone, Copy, Debug)]
pub struct DiskValue<T> {
    pub top: T,
    pub middle: T,
    pub bottom: T,
}

#[derive(Clone, Copy, Debug)]
pub struct DiskPositionSpeedLoad {
    pub position: DiskValue<f32>,
    pub speed: DiskValue<f32>,
    pub load: DiskValue<f32>,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pid {
    pub p: f32,
    pub i: f32,
    pub d: f32,
}

const MISSING: u8 = 255;

reg_read_only!(model_number, 0, u16);
reg_read_only!(firmware_version, 6, u8);
reg_read_write!(id, 7, u8);

reg_write_only!(system_check, MISSING, u8);
reg_read_only!(motors_drivers_states, MISSING, u8);

reg_read_write!(voltage_limit, MISSING, f32);
reg_read_write!(intensity_limit, MISSING, f32);

reg_read_write!(velocity_pid, MISSING, DiskValue::<f32>);
reg_read_write!(velocity_p_gain, MISSING, f32);
reg_read_write!(velocity_i_gain, MISSING, f32);
reg_read_write!(velocity_d_gain, MISSING, f32);
reg_read_write!(velocity_out_ramp, MISSING, f32);
reg_read_write!(velocity_low_pass_filter, MISSING, f32);

reg_read_write!(angle_pid, MISSING, Pid);
reg_read_write!(angle_p_gain, MISSING, f32);
reg_read_write!(angle_i_gain, MISSING, f32);
reg_read_write!(angle_d_gain, MISSING, f32);
reg_read_write!(angle_velocity_limit, MISSING, f32);

reg_read_write!(temperature_limit, MISSING, f32);

reg_read_write!(torque_enable, 20, u8);

reg_read_write!(goal_position, 50, DiskValue::<f32>);
reg_read_write!(top_goal_position, 50, f32);
reg_read_write!(middle_goal_position, 54, f32);
reg_read_write!(bottom_goal_position, 58, f32);

reg_read_only!(present_position_speed_load, 90, DiskPositionSpeedLoad);
reg_read_only!(present_position, 90, DiskValue::<f32>);
reg_read_only!(top_present_position, 90, f32);
reg_read_only!(middle_present_position, 94, f32);
reg_read_only!(bottom_present_position, 98, f32);
reg_read_only!(present_speed, 102, DiskValue::<f32>);
reg_read_only!(top_present_speed, 102, f32);
reg_read_only!(middle_present_speed, 106, f32);
reg_read_only!(bottom_present_speed, 110, f32);
reg_read_only!(present_load, 114, DiskValue::<f32>);
reg_read_only!(top_present_load, 114, f32);
reg_read_only!(middle_present_load, 118, f32);
reg_read_only!(bottom_present_load, 122, f32);

reg_read_only!(present_temperature, 130, DiskValue::<f32>);
reg_read_only!(top_present_temperature, 130, f32);
reg_read_only!(middle_present_temperature, 130, f32);
reg_read_only!(bottom_present_temperature, 130, f32);

impl<T: PartialEq> PartialEq for DiskValue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.top == other.top && self.middle == other.middle && self.bottom == other.bottom
    }
}

impl DiskValue<f32> {
    pub fn from_le_bytes(bytes: [u8; 12]) -> Self {
        DiskValue {
            top: f32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            middle: f32::from_le_bytes(bytes[4..8].try_into().unwrap()),
            bottom: f32::from_le_bytes(bytes[8..12].try_into().unwrap()),
        }
    }
    pub fn to_le_bytes(&self) -> [u8; 12] {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.top.to_le_bytes());
        bytes.extend_from_slice(&self.middle.to_le_bytes());
        bytes.extend_from_slice(&self.bottom.to_le_bytes());

        bytes.try_into().unwrap()
    }
}

impl Pid {
    pub fn from_le_bytes(bytes: [u8; 12]) -> Self {
        Pid {
            p: f32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            i: f32::from_le_bytes(bytes[4..8].try_into().unwrap()),
            d: f32::from_le_bytes(bytes[8..12].try_into().unwrap()),
        }
    }
    pub fn to_le_bytes(&self) -> [u8; 12] {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.p.to_le_bytes());
        bytes.extend_from_slice(&self.i.to_le_bytes());
        bytes.extend_from_slice(&self.d.to_le_bytes());

        bytes.try_into().unwrap()
    }
}

impl DiskPositionSpeedLoad {
    pub fn from_le_bytes(bytes: [u8; 36]) -> Self {
        DiskPositionSpeedLoad {
            position: DiskValue::from_le_bytes(bytes[0..12].try_into().unwrap()),
            speed: DiskValue::from_le_bytes(bytes[12..24].try_into().unwrap()),
            load: DiskValue::from_le_bytes(bytes[24..36].try_into().unwrap()),
        }
    }
    pub fn to_le_bytes(&self) -> [u8; 36] {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.position.to_le_bytes());
        bytes.extend_from_slice(&self.speed.to_le_bytes());
        bytes.extend_from_slice(&self.load.to_le_bytes());

        bytes.try_into().unwrap()
    }
}

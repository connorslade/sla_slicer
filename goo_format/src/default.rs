use common::serde::SizedString;

use crate::{HeaderInfo, LayerContent, PreviewImage};

impl Default for HeaderInfo {
    fn default() -> Self {
        Self {
            version: SizedString::new(b"V3.0"),
            software_info: SizedString::new(b"sla_slicer by Connor Slade"),
            software_version: SizedString::new(b"0.1.0"),
            file_time: SizedString::new(b"2024-06-14 08:10:14"),
            printer_name: SizedString::new(b"standard"),
            printer_type: SizedString::new(b"Default"),
            profile_name: SizedString::new(b"New Script"),
            anti_aliasing_level: 8,
            grey_level: 0,
            blur_level: 0,
            small_preview: PreviewImage::empty(),
            big_preview: PreviewImage::empty(),
            layer_count: 171,
            x_resolution: 11520,
            y_resolution: 5102,
            x_mirror: false,
            y_mirror: false,
            x_size: 218.88,
            y_size: 122.88,
            z_size: 260.0,
            layer_thickness: 0.05,
            exposure_time: 3.0,
            exposure_delay_mode: true,
            turn_off_time: 0.0,
            bottom_before_lift_time: 0.0,
            bottom_after_lift_time: 0.0,
            bottom_after_retract_time: 0.0,
            before_lift_time: 0.0,
            after_lift_time: 0.0,
            after_retract_time: 0.0,
            bottom_exposure_time: 50.0,
            bottom_layers: 8,
            bottom_lift_distance: 5.0,
            bottom_lift_speed: 65.0,
            lift_distance: 5.0,
            lift_speed: 65.0,
            bottom_retract_distance: 5.0,
            bottom_retract_speed: 150.0,
            retract_distance: 5.0,
            retract_speed: 0.0,
            bottom_second_lift_distance: 0.0,
            bottom_second_lift_speed: 0.0,
            second_lift_distance: 0.0,
            second_lift_speed: 0.0,
            bottom_second_retract_distance: 0.0,
            bottom_second_retract_speed: 0.0,
            second_retract_distance: 0.0,
            second_retract_speed: 0.0,
            bottom_light_pwm: 255,
            light_pwm: 255,
            advance_mode: false,
            printing_time: 2659,
            total_volume: 526.507,
            total_weight: 0.684,
            total_price: 0.0,
            price_unit: SizedString::new(b"$"),
            grey_scale_level: true,
            transition_layers: 10,
        }
    }
}

impl Default for LayerContent {
    fn default() -> Self {
        Self {
            pause_flag: 0,
            pause_position_z: 200.0,
            layer_position_z: 0.05,
            layer_exposure_time: 50.0,
            layer_off_time: 0.0,
            before_lift_time: 0.0,
            after_lift_time: 0.0,
            after_retract_time: 0.0,
            lift_distance: 5.0,
            lift_speed: 65.0,
            second_lift_distance: 0.0,
            second_lift_speed: 0.0,
            retract_distance: 5.0,
            retract_speed: 150.0,
            second_retract_distance: 0.0,
            second_retract_speed: 0.0,
            light_pwm: 255,
            data: Vec::new(),
            checksum: 0,
        }
    }
}
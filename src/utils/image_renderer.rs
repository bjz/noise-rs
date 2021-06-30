use crate::math::interpolate;
use std::{self, f64::consts::SQRT_2};

use super::{color_gradient::*, noise_image::*, noise_map::*};

pub struct ImageRenderer {
    // The color gradient used to specify the image colors.
    gradient: ColorGradient,

    // The light source, if there is one being used.
    light_source: LightSource,

    light_enabled: bool,

    // Flag specifying whether wrapping is enabled.
    wrap_enabled: bool,
}

impl ImageRenderer {
    pub fn new() -> Self {
        Self {
            gradient: ColorGradient::new(),
            light_source: LightSource::new(),
            light_enabled: false,
            wrap_enabled: false,
        }
    }

    pub fn set_gradient(self, gradient: ColorGradient) -> Self {
        Self { gradient, ..self }
    }

    pub fn gradient(&self) -> &ColorGradient {
        &self.gradient
    }

    pub fn enable_light(&mut self) {
        self.light_enabled = true;
    }

    pub fn disable_light(&mut self) {
        self.light_enabled = false;
    }

    pub fn light_enabled(&self) -> bool {
        self.light_enabled
    }

    pub fn set_light_azimuth(mut self, azimuth: f64) -> Self {
        self.light_source.set_azimuth(azimuth);

        self
    }

    pub fn light_azimuth(&self) -> f64 {
        self.light_source.azimuth
    }

    pub fn set_light_brightness(mut self, brightness: f64) -> Self {
        self.light_source.set_brightness(brightness);

        self
    }

    pub fn light_brightness(&self) -> f64 {
        self.light_source.brightness
    }

    pub fn set_light_color(mut self, color: Color) -> Self {
        self.light_source.set_color(color);

        self
    }

    pub fn light_color(&self) -> Color {
        self.light_source.color
    }

    pub fn set_light_contrast(mut self, contrast: f64) -> Self {
        self.light_source.set_contrast(contrast);

        self
    }

    pub fn light_contrast(&self) -> f64 {
        self.light_source.contrast
    }

    pub fn set_light_elevation(mut self, elevation: f64) -> Self {
        self.light_source.set_elevation(elevation);

        self
    }

    pub fn light_elevation(&self) -> f64 {
        self.light_source.elevation
    }

    pub fn set_light_intensity(mut self, intensity: f64) -> Self {
        self.light_source.set_intensity(intensity);

        self
    }

    pub fn light_intensity(&self) -> f64 {
        self.light_source.intensity
    }

    pub fn enable_wrap(self) -> Self {
        Self {
            wrap_enabled: true,
            ..self
        }
    }

    pub fn wrap_enabled(&self) -> bool {
        self.wrap_enabled
    }

    pub fn render(&mut self, noise_map: &NoiseMap) -> NoiseImage {
        // noise_map.width
        let (width, height) = noise_map.size();

        let mut destination_image = NoiseImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let point = noise_map.get_value(x, y);

                let source_color = self.gradient.get_color(point);

                let mut light_intensity;

                if self.light_enabled {
                    let mut x_left_offset: isize = -1;
                    let mut x_right_offset: isize = 1;
                    let mut y_down_offset: isize = -1;
                    let mut y_up_offset: isize = 1;

                    if self.wrap_enabled {
                        if x == 0 {
                            x_left_offset = width as isize - 1;
                            x_right_offset = 1;
                        } else if x == (width as isize - 1) as usize {
                            x_left_offset = -1;
                            x_right_offset = width as isize - 1;
                        }

                        if y == 0 {
                            y_down_offset = height as isize - 1;
                            y_up_offset = 1;
                        } else if y == (height as isize - 1) as usize {
                            y_down_offset = -1;
                            y_up_offset = height as isize - 1;
                        }
                    } else {
                        if x == 0 {
                            x_left_offset = 0;
                            x_right_offset = 1;
                        } else if x == (width as isize - 1) as usize {
                            x_left_offset = -1;
                            x_right_offset = 0;
                        }

                        if y == 0 {
                            y_down_offset = 0;
                            y_up_offset = 1;
                        } else if y == (height as isize - 1) as usize {
                            y_down_offset = -1;
                            y_up_offset = 0;
                        }
                    }

                    let pc = point;
                    let pl = noise_map.get_value((x as isize + x_left_offset) as usize, y);
                    let pr = noise_map.get_value((x as isize + x_right_offset) as usize, y);
                    let pd = noise_map.get_value(x, (y as isize + y_down_offset) as usize);
                    let pu = noise_map.get_value(x, (y as isize + y_up_offset) as usize);

                    light_intensity = self.light_source.calc_light_intensity(pc, pl, pr, pd, pu);
                    light_intensity *= self.light_source.brightness;
                } else {
                    light_intensity = 1.0;
                }

                let destination_color = self.calc_destination_color(source_color, light_intensity);

                destination_image.set_value(x, y, destination_color);
            }
        }

        destination_image
    }

    fn calc_destination_color(&self, source_color: Color, light_value: f64) -> Color {
        let source = u8_array_to_f64_array(source_color);

        let mut red = source[0];
        let mut green = source[1];
        let mut blue = source[2];

        if self.light_enabled {
            // Calculate light color
            let light_red = light_value * f64::from(self.light_source.color[0]) / 255.0;
            let light_green = light_value * f64::from(self.light_source.color[1]) / 255.0;
            let light_blue = light_value * f64::from(self.light_source.color[2]) / 255.0;

            // Apply the light color
            red *= light_red;
            green *= light_green;
            blue *= light_blue;
        }

        // Clamp color channels to [0..1]
        red = red.max(0.0).min(1.0);
        green = green.max(0.0).min(1.0);
        blue = blue.max(0.0).min(1.0);

        // Rescale color channels to u8 [0..255] and return the final color
        [
            (red * 255.0) as u8,
            (green * 255.0) as u8,
            (blue * 255.0) as u8,
            source_color[3],
        ]
    }

    pub fn render_with_background(
        &mut self,
        noise_map: &NoiseMap,
        background: &NoiseImage,
    ) -> NoiseImage {
        // noise_map.width
        let (width, height) = noise_map.size();

        let mut destination_image = NoiseImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let point = noise_map.get_value(x, y);
                let source_color = self.gradient.get_color(point);

                let mut light_intensity;

                if self.light_enabled {
                    let mut x_left_offset: isize = -1;
                    let mut x_right_offset: isize = 1;
                    let mut y_down_offset: isize = -1;
                    let mut y_up_offset: isize = 1;

                    if self.wrap_enabled {
                        if x == 0 {
                            x_left_offset = width as isize - 1;
                            x_right_offset = 1;
                        } else if x == (width as isize - 1) as usize {
                            x_left_offset = -1;
                            x_right_offset = width as isize - 1;
                        }

                        if y == 0 {
                            y_down_offset = height as isize - 1;
                            y_up_offset = 1;
                        } else if y == (height as isize - 1) as usize {
                            y_down_offset = -1;
                            y_up_offset = height as isize - 1;
                        }
                    } else {
                        if x == 0 {
                            x_left_offset = 0;
                            x_right_offset = 1;
                        } else if x == (width as isize - 1) as usize {
                            x_left_offset = -1;
                            x_right_offset = 0;
                        }

                        if y == 0 {
                            y_down_offset = 0;
                            y_up_offset = 1;
                        } else if y == (height as isize - 1) as usize {
                            y_down_offset = -1;
                            y_up_offset = 0;
                        }
                    }

                    let pc = point;
                    let pl = noise_map.get_value((x as isize + x_left_offset) as usize, y);
                    let pr = noise_map.get_value((x as isize + x_right_offset) as usize, y);
                    let pd = noise_map.get_value(x, (y as isize + y_down_offset) as usize);
                    let pu = noise_map.get_value(x, (y as isize + y_up_offset) as usize);

                    light_intensity = self.light_source.calc_light_intensity(pc, pl, pr, pd, pu);
                    light_intensity *= self.light_source.brightness;
                } else {
                    light_intensity = 1.0;
                }

                let background_color = background.get_value(x, y);

                let destination_color = self.calc_destination_color_with_background(
                    source_color,
                    background_color,
                    light_intensity,
                );

                destination_image.set_value(x, y, destination_color);
            }
        }

        destination_image
    }

    fn calc_destination_color_with_background(
        &self,
        source_color: Color,
        background_color: Color,
        light_value: f64,
    ) -> Color {
        let source = u8_array_to_f64_array(source_color);
        let background = u8_array_to_f64_array(background_color);

        // Blend source color and background color together using source's alpha.
        let mut red = interpolate::linear(source[0], background[0], source[3]);
        let mut green = interpolate::linear(source[1], background[1], source[3]);
        let mut blue = interpolate::linear(source[2], background[2], source[3]);

        if self.light_enabled {
            // Calculate light color
            let light_red = light_value * f64::from(self.light_source.color[0]) / 255.0;
            let light_green = light_value * f64::from(self.light_source.color[1]) / 255.0;
            let light_blue = light_value * f64::from(self.light_source.color[2]) / 255.0;

            // Apply the light color
            red *= light_red;
            green *= light_green;
            blue *= light_blue;
        }

        // Clamp color channels to [0..1]
        red = red.max(0.0).min(1.0);
        green = green.max(0.0).min(1.0);
        blue = blue.max(0.0).min(1.0);

        // Rescale color channels to u8 [0..255] and return the final color
        [
            (red * 255.0) as u8,
            (green * 255.0) as u8,
            (blue * 255.0) as u8,
            std::cmp::max(source_color[3], background_color[3]),
        ]
    }
}

impl Default for ImageRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone, Default)]
pub struct LightSource {
    // Azimuth of the light source, in degrees.
    azimuth: f64,

    // Brightness of the light source.
    brightness: f64,

    // The color of the light source.
    color: Color,

    // The contrast between areas in light and areas in shadow.
    contrast: f64,

    // Elevation of the light source, in degrees.
    elevation: f64,

    // The intensity of the light source.
    intensity: f64,

    // The cosine of the azimuth of the light source.
    azimuth_cosine: f64,

    // The sine of the azimuth of the light source.
    azimuth_sine: f64,

    // The cosine of the elevation of the light source.
    elevation_cosine: f64,

    // The sine of the elevation of the light source.
    elevation_sine: f64,

    // Used by the calc_light_intensity method to recalculate the light values
    // only if the light parameters change.
    //
    // When the light parameters change, this value is set to True. When the
    // calc_light_intensity method is called, this value is set to false.
    recalculate_light_values: bool,
}

impl LightSource {
    pub fn new() -> Self {
        Self {
            azimuth: 45.0,
            brightness: 1.0,
            color: [255; 4],
            contrast: 1.0,
            elevation: 45.0,
            intensity: 1.0,
            azimuth_cosine: 45.0_f64.to_radians().cos(),
            azimuth_sine: 45.0_f64.to_radians().sin(),
            elevation_cosine: 45.0_f64.to_radians().cos(),
            elevation_sine: 45.0_f64.to_radians().sin(),
            recalculate_light_values: false,
        }
    }

    pub fn set_azimuth(&mut self, azimuth: f64) {
        self.azimuth = azimuth;
        self.recalculate_light_values = true;
    }

    pub fn set_brightness(&mut self, brightness: f64) {
        self.brightness = brightness;
        self.recalculate_light_values = true;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_contrast(&mut self, contrast: f64) {
        if contrast >= 0.0 {
            self.contrast = contrast;
            self.recalculate_light_values = true;
        } else {
            eprintln!("contrast value out of bounds: {}", contrast);
        }
    }

    pub fn set_elevation(&mut self, elevation: f64) {
        self.elevation = elevation;
        self.recalculate_light_values = true;
    }

    pub fn set_intensity(&mut self, intensity: f64) {
        self.intensity = intensity;
        self.recalculate_light_values = true;
    }

    fn calc_light_intensity(
        &mut self,
        _center: f64,
        left: f64,
        right: f64,
        down: f64,
        up: f64,
    ) -> f64 {
        // Recalculate the sine and cosine of the various light values if necessary so it does not
        // have to be calculated each time this method is called.
        if self.recalculate_light_values {
            self.azimuth_cosine = self.azimuth.to_radians().cos();
            self.azimuth_sine = self.azimuth.to_radians().sin();
            self.elevation_cosine = self.elevation.to_radians().cos();
            self.elevation_sine = self.elevation.to_radians().sin();

            self.recalculate_light_values = false;
        }

        let i_max = 1.0;

        let io = i_max * SQRT_2 * self.elevation_sine / 2.0;
        let ix =
            (i_max - io) * self.contrast * SQRT_2 * self.elevation_cosine * self.azimuth_cosine;
        let iy = (i_max - io) * self.contrast * SQRT_2 * self.elevation_cosine * self.azimuth_sine;

        let intensity = ix * (left - right) + iy * (down - up) + io;

        if intensity < 0.0 {
            return 0.0;
        }

        intensity
    }
}

#[inline]
fn u8_array_to_f64_array(input: [u8; 4]) -> [f64; 4] {
    let mut result = [0.0; 4];

    for x in 0..4 {
        result[x] = f64::from(input[x]) / 255.0;
    }

    result
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn array_conversion() {
        assert_eq!([0.0; 4], u8_array_to_f64_array([0; 4]));
        assert_eq!([1.0; 4], u8_array_to_f64_array([255; 4]));
    }
}

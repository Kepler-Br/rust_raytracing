use std::mem::swap;

use nalgebra_glm::{lerp, Vec3, Vec4};

pub fn blackbody_blender(t: f32) -> Vec3 {
    const TABLE_G: [Vec3; 6] = [
        Vec3::new(-7.50343014e+02, 3.15679613e-04, 4.73464526e-01),
        Vec3::new(-1.00402363e+03, 1.29189794e-04, 9.08181524e-01),
        Vec3::new(-1.22075471e+03, 2.56245413e-05, 1.20753416e+00),
        Vec3::new(-1.42546105e+03, -4.01730887e-05, 1.44002695e+00),
        Vec3::new(-1.18134453e+03, -2.18913373e-05, 1.30656109e+00),
        Vec3::new(-5.00279505e+02, -4.59745390e-06, 1.09090465e+00),
    ];

    const TABLE_B: [Vec4; 6] = [
        Vec4::new(0.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 0.0),
        Vec4::new(
            -2.02524603e-11,
            1.79435860e-07,
            -2.60561875e-04,
            -1.41761141e-02,
        ),
        Vec4::new(
            -2.22463426e-13,
            -1.55078698e-08,
            3.81675160e-04,
            -7.30646033e-01,
        ),
        Vec4::new(
            6.72595954e-13,
            -2.73059993e-08,
            4.24068546e-04,
            -7.52204323e-01,
        ),
    ];

    const TABLE_R: [Vec3; 6] = [
        Vec3::new(2.52432244e+03, -1.06185848e-03, 3.11067539e+00),
        Vec3::new(3.37763626e+03, -4.34581697e-04, 1.64843306e+00),
        Vec3::new(4.10671449e+03, -8.61949938e-05, 6.41423749e-01),
        Vec3::new(4.66849800e+03, 2.85655028e-05, 1.29075375e-01),
        Vec3::new(4.60124770e+03, 2.89727618e-05, 1.48001316e-01),
        Vec3::new(3.78765709e+03, 9.36026367e-06, 3.98995841e-01),
    ];

    let rgb;

    if t >= 12000.0 {
        rgb = Vec3::new(0.826270103, 0.994478524, 1.56626022);
    } else if t < 965.0 {
        rgb = Vec3::new(4.70366907, 0.0, 0.0);
    } else {
        let i = if t >= 6365.0 {
            5
        } else if t >= 3315.0 {
            4
        } else if t >= 1902.0 {
            3
        } else if t >= 1449.0 {
            2
        } else if t >= 1167.0 {
            1
        } else {
            0
        };

        let r = TABLE_R[i];
        let g = TABLE_G[i];
        let b = TABLE_B[i];

        let t_inv = 1.0 / t;

        rgb = Vec3::new(
            r.x * t_inv + r.y * t + r.z,
            g.x * t_inv + g.y * t + g.z,
            ((b.x * t + b.y) * t + b.z) * t + b.w,
        );
    }

    return rgb;
}

/// Approximation of blackbody color using linear interpolation between three colors.
/// Working range is from 1000 K to 40000 K.
/// 1000 K is `Vec3(1.0, 0.22, 0.0)`
/// 6600 K is `Vec3(1.0, 0.976, 0.992)`
/// 40000 K is `Vec3(0.608, 0.737, 1.0)`
///
/// # Arguments
///
/// * `temperature`: temperature in kelvins
///
/// returns: RGB color
pub fn blackbody(temperature: f32) -> Vec3 {
    const RANGE_FIRST: f32 = 1000.0;
    const RANGE_SECOND: f32 = 6600.0;
    const RANGE_THIRD: f32 = 29800.0;

    // 1000 K
    const FIRST: Vec3 = Vec3::new(1.0, 0.22, 0.0);
    // 6600 K
    const SECOND: Vec3 = Vec3::new(1.0, 0.976, 0.992);
    // 40000 K
    const THIRD: Vec3 = Vec3::new(0.624, 0.749, 1.0);

    if temperature <= RANGE_FIRST {
        return FIRST;
    }

    if temperature == RANGE_SECOND {
        return SECOND;
    }

    if temperature >= RANGE_THIRD {
        return THIRD;
    }

    return if temperature < RANGE_SECOND {
        const DIV_RANGE: f32 = 1.0 / (RANGE_SECOND - RANGE_FIRST);

        let t = (temperature - RANGE_FIRST) * DIV_RANGE;

        lerp(&FIRST, &SECOND, t)
    } else {
        const DIV_RANGE: f32 = 1.0 / (RANGE_THIRD - RANGE_SECOND);

        let t = (temperature - RANGE_SECOND) * DIV_RANGE;

        lerp(&SECOND, &THIRD, t)
    };
}

/// Converts float color value from range [0.0, 1.0] f32 to [0, 255] u8
///
/// # Arguments
///
/// * `value`: Color. Will be clamped from 0.0 to 1.0
///
/// returns: Color converted to u8
pub fn f32_to_u8(value: f32) -> u8 {
    return (value.clamp(0.0, 1.0) * 255.0) as u8;
}

pub fn hsl_to_rgb(hsl: &Vec3) -> Vec3 {
    let nr = f32::clamp(f32::abs(hsl.x * 6.0 - 3.0) - 1.0, 0.0, 1.0);
    let ng = f32::clamp(2.0 - f32::abs(hsl.x * 6.0 - 2.0), 0.0, 1.0);
    let nb = f32::clamp(2.0 - f32::abs(hsl.x * 6.0 - 4.0), 0.0, 1.0);

    let chroma = (1.0 - f32::abs(2.0 * hsl.z - 1.0)) * hsl.y;

    return Vec3::new(
        (nr - 0.5) * chroma + hsl.z,
        (ng - 0.5) * chroma + hsl.z,
        (nb - 0.5) * chroma + hsl.z,
    );
}

pub fn hsv_to_rgb(hsv: &Vec3) -> Vec3 {
    let nr = f32::clamp(f32::abs(hsv.x * 6.0 - 3.0) - 1.0, 0.0, 1.0);
    let ng = f32::clamp(2.0 - f32::abs(hsv.x * 6.0 - 2.0), 0.0, 1.0);
    let nb = f32::clamp(2.0 - f32::abs(hsv.x * 6.0 - 4.0), 0.0, 1.0);

    return Vec3::new(
        ((nr - 1.0) * hsv.y + 1.0) * hsv.z,
        ((ng - 1.0) * hsv.y + 1.0) * hsv.z,
        ((nb - 1.0) * hsv.y + 1.0) * hsv.z,
    );
}

pub fn rgb_to_hsl(rgb: &Vec3) -> Vec3 {
    let cmax = f32::max(f32::max(rgb.x, rgb.y), rgb.z);
    let cmin = f32::min(f32::min(rgb.x, rgb.y), rgb.z);

    let mut h;
    let s;
    let l = f32::min(1.0, (cmax + cmin) / 2.0);

    if cmax == cmin {
        h = 0.0;
        s = 0.0;
    } else {
        let d = cmax - cmin;

        s = if l > 0.5 {
            d / (2.0 - cmax - cmin)
        } else {
            d / (cmax + cmin)
        };

        if cmax == rgb.x {
            let coefficient = if rgb.y < rgb.z { 6.0 } else { 0.0 };

            h = (rgb.y - rgb.z) / d + coefficient;
        } else if cmax == rgb.y {
            h = (rgb.z - rgb.x) / d + 2.0;
        } else {
            h = (rgb.x - rgb.y) / d + 4.0;
        }
    }

    h /= 6.0;

    return Vec3::new(h, s, l);
}

pub fn rgb_to_hsv(rgb: &Vec3) -> Vec3 {
    let mut r = rgb.x;
    let mut g = rgb.y;
    let mut b = rgb.z;

    let mut k = 0.0;
    let mut min_gb;

    if g < b {
        swap(&mut g, &mut b);
        k = -1.0;
    }

    min_gb = b;

    if r < g {
        swap(&mut r, &mut g);
        k = -2.0 / 6.0 - k;
        min_gb = f32::min(g, b);
    }

    let chroma = r - min_gb;

    return Vec3::new(
        f32::abs(k + (g - b) / 6.0 * chroma + 1e-20),
        chroma / (r + 1e-20),
        r,
    );
}

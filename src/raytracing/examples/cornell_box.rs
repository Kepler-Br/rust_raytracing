use nalgebra_glm::{IVec2, Vec3};

use crate::misc::color::blackbody_blender;
use crate::misc::default_rand_gen::DefaultRandGen;
use crate::scene::scene_info::SceneInfo;

pub fn cornell_box(resolution: IVec2) -> SceneInfo {
    // let rand = DefaultRandGen::new_rc();
    // // Materials
    // let emission = Emission::new_rc(blackbody_blender(3000.0), 10.0);
    // let green = Lambertian::new_rc(Vec3::new(0.0, 1.0, 0.0), rand.clone());
    // let red = Lambertian::new_rc(Vec3::new(1.0, 0.0, 0.0), rand.clone());
    // let white = Lambertian::new_rc(Vec3::new(1.0, 1.0, 1.0), rand.clone());
    //
    // // Hittables
    // let lantern =
    //     XyRect::square_boxed(Vec3::new(0.0, 0.0, 1.0 - 0.01), 2.0 - 0.4, emission.clone());
    // let back = XzRect::square_boxed(Vec3::new(0.0, -1.0, 0.0), 2.0, white.clone());
    // let floor = XyRect::square_boxed(Vec3::new(0.0, 0.0, -1.0), 2.0, white.clone());
    // let ceiling = XyRect::square_boxed(Vec3::new(0.0, 0.0, 1.0), 2.0, white.clone());
    // let left = YzRect::square_boxed(Vec3::new(1.0, 0.0, 0.0), 2.0, green.clone());
    // let right = YzRect::square_boxed(Vec3::new(-1.0, 0.0, 0.0), 2.0, red.clone());
    // let sphere = Sphere::boxed(Vec3::new(0.0, 0.0, 0.0), 0.5, white.clone());

    return SceneInfo::new(DefaultRandGen::new_rc)
        .camera(
            Vec3::new(0.0, 2.4, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            70.0,
            resolution.x as f32 / resolution.y as f32,
        )
        .emission("lantern", blackbody_blender(5000.0), 1.0)
        .emission("lantern-red", blackbody_blender(2000.0), 1.0)
        .lambertian("green", Vec3::new(0.0, 1.0, 0.0))
        .lambertian("red", Vec3::new(1.0, 0.0, 0.0))
        .lambertian("white", Vec3::new(1.0, 1.0, 1.0))
        .reflective("reflective", Vec3::new(1.0, 1.0, 1.0), 1.0/0.001)
        .refractive("refractive", Vec3::new(1.0, 1.0, 1.0), 1.5)
        .lambertian("purple", Vec3::new(1.0, 0.0, 1.0) * 0.6)
        // .xy_square(
        //     "lantern",
        //     "lantern",
        //     Vec3::new(0.0, 0.0, 1.0 - 0.01),
        //     2.0 - 0.4,
        // )
        .yz_square(
            "lantern",
            "lantern",
            Vec3::new(1.0 - 0.01, 0.0, 0.0),
            2.0 - 0.4,
        )
        .yz_square(
            "lantern",
            "lantern-red",
            Vec3::new(-1.0 + 0.01, 0.0, 0.0),
            2.0 - 0.4,
        )
        .xz_square("back", "white", Vec3::new(0.0, -1.0, 0.0), 2.0)
        .xy_square("floor", "white", Vec3::new(0.0, 0.0, -1.0), 2.0)
        .xy_square("ceiling", "white", Vec3::new(0.0, 0.0, 1.0), 2.0)
        .yz_square("left", "green", Vec3::new(1.0, 0.0, 0.0), 2.0)
        .yz_square("right", "red", Vec3::new(-1.0, 0.0, 0.0), 2.0)
        .sphere(
            "sphere",
            "reflective",
            Vec3::new(0.0, 0.0, 0.0 - (1.0 - 0.5)),
            0.5,
        );

    // return SceneBuilder::new(rand.clone())
    //     .material(green.clone())
    //     .material(red.clone())
    //     .material(white.clone())
    //     .hittable(floor)
    //     .hittable(ceiling)
    //     .hittable(left)
    //     .hittable(right)
    //     .hittable(back)
    //     .hittable(lantern)
    //     .camera(Camera::new(
    //         Vec3::new(0.0, 1.0, 0.0),
    //         Vec3::new(0.0, 0.0, 0.0),
    //         Vec3::new(-1.0, 0.0, 0.0),
    //         80.0,
    //         resolution.x as f32 / resolution.y as f32,
    //         rand.clone(),
    //     ))
    //     .build();
}
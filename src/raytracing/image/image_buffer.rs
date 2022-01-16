use nalgebra_glm::{IVec2, Vec2, Vec3};

pub trait ImageBuffer {
    /// Add color to existing pixel
    /// There is a default implementation for this, but you should consider overriding it
    ///
    /// # Arguments
    ///
    /// * `coord`: X, Y coordinates to put pixel into
    /// * `color`: RGB color
    ///
    /// returns: String with text what went wrong or nothing otherwise
    fn add(&mut self, coord: IVec2, color: Vec3) -> Result<(), String> {
        let get_color = self.get(coord)?;

        self.put(coord, get_color + color)?;

        return Result::Ok(());
    }

    fn put(&mut self, coord: IVec2, color: Vec3) -> Result<(), String>;
    fn get(&self, coord: IVec2) -> Result<Vec3, String>;
    fn get_resolution(&self) -> IVec2;
    fn get_buffer(&self) -> &[Vec3];
    fn get_buffer_mut(&mut self) -> &mut [Vec3];

    fn add_to(&self, other: &mut impl ImageBuffer) -> Result<(), String> {
        let res = self.get_resolution();
        let other_res = self.get_resolution();
        let min_x = i32::min(res.x, other_res.x);
        let min_y = i32::min(res.y, other_res.y);

        for y in 0..min_y {
            for x in 0..min_x {
                let coord = IVec2::new(x, y);
                let color = self.get(coord)?;
                let other_color = other.get(coord)?;

                other.put(coord, color + other_color)?;
            }
        }

        return Result::Ok(());
    }

    fn copy_to(&self, other: &mut impl ImageBuffer) -> Result<(), String> {
        let res = self.get_resolution();
        let other_res = self.get_resolution();
        let min_x = i32::min(res.x, other_res.x);
        let min_y = i32::min(res.y, other_res.y);

        for y in 0..min_y {
            for x in 0..min_x {
                let coord = IVec2::new(x, y);
                let color = self.get(coord)?;

                other.put(coord, color)?;
            }
        }

        return Result::Ok(());
    }

    fn copy_to_with<F>(&self, other: &mut impl ImageBuffer, fun: F) -> Result<(), String>
        where
            F: Fn(&Vec3, &Vec3, &IVec2) -> Vec3,
    {
        let res = self.get_resolution();
        let other_res = self.get_resolution();
        let min_x = i32::min(res.x, other_res.x);
        let min_y = i32::min(res.y, other_res.y);

        for y in 0..min_y {
            for x in 0..min_x {
                let coord = IVec2::new(x, y);
                let color = self.get(coord)?;
                let other_color = other.get(coord)?;

                other.put(coord, fun(&color, &other_color, &coord))?;
            }
        }

        return Result::Ok(());
    }

    fn scale_copy_to_with<F>(&self, other: &mut impl ImageBuffer, fun: F) -> Result<(), String>
        where
            F: Fn(&Vec3, &Vec3, &IVec2) -> Vec3,
    {
        let self_res = self.get_resolution();
        let self_res_f32 = Vec2::new(self_res.x as f32, self_res.y as f32);
        let other_res = other.get_resolution();
        let other_res_f32 = Vec2::new(other_res.x as f32, other_res.y as f32);

        for y in 0..other_res.y {
            for x in 0..other_res.x {
                let other_coord = IVec2::new(x, y);
                let other_norm = Vec2::new(x as f32 / other_res_f32.x, y as f32 / other_res_f32.y);
                let self_coord = IVec2::new(
                    (self_res_f32.x * other_norm.x) as i32,
                    (self_res_f32.y * other_norm.y) as i32,
                );

                let color = self.get(self_coord)?;
                let other_color = other.get(other_coord)?;

                other.put(other_coord, fun(&color, &other_color, &other_coord))?;
            }
        }

        return Result::Ok(());
    }
}

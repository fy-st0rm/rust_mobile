
#[derive(Clone, Debug)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32,
}

impl Vec2 {
	pub fn new(x: f32, y: f32) -> Vec2{
		Vec2 {
			x: x,
			y: y,
		}
	}
}

#[derive(Clone, Debug)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32) -> Vec3{
		Vec3 {
			x: x,
			y: y,
			z: z
		}
	}
}

#[derive(Clone, Debug)]
pub struct Vec4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

impl Vec4 {
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4{
		Vec4 {
			x: x,
			y: y,
			z: z,
			w: w,
		}
	}
}

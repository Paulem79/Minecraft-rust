extern crate kiss3d;

use kiss3d::camera::FirstPerson;
use kiss3d::light::Light;
use kiss3d::nalgebra::Point3;
use kiss3d::nalgebra::Vector3;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

struct Cube {
    size: f32,
    color: (f32, f32, f32),
    position: Vector3<f32>,
}

impl Cube {
    fn new(size: f32, color: (f32, f32, f32), position: Vector3<f32>) -> Self {
        Cube { size, color, position }
    }

    fn add_to_window(&self, window: &mut Window) -> SceneNode {
        let mut node = window.add_cube(self.size, self.size, self.size);
        node.set_color(self.color.0, self.color.1, self.color.2);
        node.set_local_translation(self.position.into());
        node
    }
}

fn main() {
    env_logger::init();
    let mut window = Window::new("Kiss3d: cube");

    let mut cubes = Vec::new();
    for x in 0..10 {
        let size = 1.0;
        let color = (1.0, 0.0, 0.0);
        let dx = (x as f32);

        for z in 0..10 {
            let dz = (z as f32);
            let position = Vector3::new(dx, 0.0, dz);
            let cube = Cube::new(size, color, position);
            cubes.push(cube);
        }
    }

    let mut nodes = Vec::new();
    for cube in &cubes {
        let node = cube.add_to_window(&mut window);
        nodes.push(node);
    }

    window.set_light(Light::StickToCamera);

    // CAMERA
    let eye = Point3::new(10.0f32, 10.0, 10.0);
    let at = Point3::origin();
    let mut first_person = FirstPerson::new(eye, at);

    while !window.should_close() {
        window.render_with_camera(&mut first_person);
    }
}
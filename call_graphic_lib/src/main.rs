use cgmath::prelude::*;

const NUM_INSTANCES_PER_ROW: u32 = 10;

struct Instance {
    position: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
}

impl Instance {
    fn to_raw(&self) -> lens::InstanceRaw {
        let model =
            cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation);
        lens::InstanceRaw {
            model: model.into(),
            normal: cgmath::Matrix3::from(self.rotation).into(),
        }
    }
}

fn main() {
    let mut lens_scene = lens::Lens::new();

    let res_dir = std::path::Path::new(env!("OUT_DIR")).join("res");
    let cube_object = lens::Object::load_from(res_dir.join("cube").join("cube.obj"));

    let mut light_object = lens::Object::load_from(res_dir.join("cube").join("cube.obj"));
    light_object.textures = None;

    const SPACE_BETWEEN: f32 = 3.0;
    let instances = (0..NUM_INSTANCES_PER_ROW)
        .flat_map(|z| {
            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                let position = cgmath::Vector3 { x, y: 0.0, z };

                let rotation = if position.is_zero() {
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                Instance { position, rotation }
            })
        })
        .collect::<Vec<_>>();

    let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
    let instance_len = instance_data.len();

    lens_scene.add_object(lens::LensObject {
        object: light_object,
        position: cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        transform: None,
        shader_file: include_str!("../shader/light.wgsl").into(),
        instances: None,
    });

    lens_scene.add_object(lens::LensObject {
        object: cube_object,
        position: cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        transform: None,
        shader_file: include_str!("../shader/shader.wgsl").into(),
        instances: Some((instance_data, instance_len)),
    });

    lens_scene.run();
}

// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Animation!".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(-200.0, 50.0, 50.0),
        vec3(0.0, 50.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 1000.0);

    let mut loaded = if let Ok(loaded) = three_d_asset::io::load_async(&[
        "../assets/Fox.glb", // Source: https://github.com/KhronosGroup/glTF-Sample-Models/tree/master/2.0
    ])
    .await
    {
        loaded
    } else {
        three_d_asset::io::load_async(&[
            "https://asny.github.io/three-d/assets/Fox.glb",
        ])
        .await
        .expect("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
    };

    let mut cpu_model: CpuModel = loaded.deserialize("glb").unwrap();
    cpu_model
        .geometries
        .iter_mut()
        .for_each(|g| g.compute_normals());
    let model = Model::<PhysicalMaterial>::new(&context, &cpu_model).unwrap();

    let light = AmbientLight::new(&context, 1.0, Color::WHITE);

    // main loop
    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
            .render(&camera, &model, &[&light]);

        FrameOutput::default()
    });
}

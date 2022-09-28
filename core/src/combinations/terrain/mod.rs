use specs::{Builder, DispatcherBuilder, WorldExt};

use crate::{combinations::Combination, components, systems};

use wasm_bindgen::JsValue;
use web_sys::console::log_1;

use rust_3d::*;

pub struct TerrainCombination;
impl Combination for TerrainCombination {
    fn init<'world, 'a, 'b>(world: &'world mut specs::World) -> specs::Dispatcher<'a, 'b> {
        // components (register)
        world.register::<components::terrain::Terrain>();

        let mesh = terrain::gen_mesh().unwrap();
        let num_faces = mesh.num_faces();
        // MEMO: Terrain에서 entity는 쓰이지 않으나, 예시로 남겨둡니다.
        // // entities (build)
        let terrain = world
            .create_entity()
            .with(components::terrain::Terrain {
                bitmap: terrain::test_runner1().unwrap(),
                mesh,
            })
            .build();

        let debug_message = JsValue::from(format!("num_faces: {}", num_faces));
        log_1(&debug_message);

        DispatcherBuilder::new()
            .with(
                systems::renders::terrain::RenderTerrainSystem {},
                "render_terrain_system",
                &[],
            )
            .build()
    }
}

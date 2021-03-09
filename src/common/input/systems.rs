use super::marker_components::MainCamera;
use super::resources::MouseWorldPosition;
use bevy::prelude::*;

//credit to jamadazi for mouse world position
pub fn get_mouse_world_position(
    // events to get cursor position
    mut evr_cursor: EventReader<CursorMoved>,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<&Transform, With<MainCamera>>,
    // query for pickable rects
    mut mouse_world_position: ResMut<MouseWorldPosition>,
) {
    // assuming there is exactly one main camera entity, so this is OK
    let camera_transform = q_camera.iter().next().unwrap();

    for ev in evr_cursor.iter() {
        // get the size of the window that the event is for
        let wnd = wnds.get(ev.id).unwrap();
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = ev.position - size / 2.0;

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        mouse_world_position.position = Vec2::new(pos_wld.x, pos_wld.y);
    }
}

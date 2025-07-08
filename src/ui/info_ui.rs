use bevy::prelude::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {

        //add ui elements / plugins here
        app
            .add_plugins(crate::ui::fps_ui::FPSUIPlugin)
            .add_plugins(crate::ui::cam_mode_ui::CamUIPlugin)
            .add_plugins(crate::ui::coords_ui::CoordsUIPlugin)
        ;
    }
}
use crate::{
    component::{self, PieceColor},
    game::Data,
    player::{Player, PlayerFocus},
    public,
};
use bevy::prelude::*;

#[derive(Event)]
pub struct SwithPlayerEvent;

pub fn swith_player_event(
    mut commands: Commands,
    mut data: ResMut<Data>,
    mut events: EventReader<SwithPlayerEvent>,
    mut query: Query<&mut Style, With<PlayerFocus>>,
) {
    // todo
    for event in events.iter() {
        let mut focus_style = query.single_mut();

        match data.current_color {
            PieceColor::White => {
                data.current_color = PieceColor::Black;
                data.white_player.stop_timer();
                data.black_player.start_timer();
                focus_style.left = Val::Percent(5.5_f32);
                trace!("swith to black player");
            }
            PieceColor::Black => {
                data.current_color = PieceColor::White;
                data.black_player.stop_timer();
                data.white_player.start_timer();
                focus_style.right = Val::Percent(5.5_f32);
                trace!("swith to white player");
            }
        }
    }
}

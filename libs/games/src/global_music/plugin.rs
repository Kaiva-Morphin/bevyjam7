use bevy::audio::{PlaybackMode, Volume};

use crate::prelude::*;


#[derive(Component)]
pub struct NewBgMusic {
    pub handle: Option<Handle<AudioSource>>,
    pub instant_translation: bool
}

#[derive(Component)]
pub struct CurrentGlobalMusic {
    handle: Handle<AudioSource>
}

#[derive(Component)]
struct PrevGlobalMusic;


pub struct GlobalMusicPlugin;

impl Plugin for GlobalMusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, music_watcher);
    }
}

fn music_watcher(
    mut music_q: Query<&mut AudioSink>,
    new_q: Query<(Entity, &NewBgMusic)>,
    prev_q: Query<Entity, With<PrevGlobalMusic>>,
    current_q: Query<(Entity, &CurrentGlobalMusic)>,
    time: Res<Time>,
    mut cmd: Commands
) {
    let dt = time.dt();
    for prev in prev_q.iter() {
        let Ok(mut s) = music_q.get_mut(prev) else {continue;};
        let volume: Volume = s.volume();
        if volume.to_linear() > 0.0 {
            s.set_volume(Volume::Linear(volume.to_linear() - (MUSIC_INTERPOLATION * dt)));
        }
        if volume.to_linear() <= 0.0 {
            cmd.entity(prev).despawn();
        }
    }
    for current in current_q.iter() {
        let Ok(mut s) = music_q.get_mut(current.0) else {continue;};
        let volume: Volume = s.volume();
        if volume.to_linear() < 1.0 {
            s.set_volume(Volume::Linear(volume.to_linear() + (MUSIC_INTERPOLATION * dt)));
        }
    }
    // trying 
    let mut new_iter = new_q.into_iter();
    let mut current_iter = current_q.into_iter();


    let Some((new_e, new_music)) = new_iter.next() else {return;};

    for (new_e, _) in new_iter {
        cmd.entity(new_e).despawn();
    }

    let Some((current_e, current)) = current_iter.next() else {
        let Some(handle) = &new_music.handle else {return;};
        let mut settings = PlaybackSettings{
            mode: PlaybackMode::Loop,
            volume: Volume::SILENT,
            ..default()
        };
        if new_music.instant_translation {
            settings.volume = Volume::Linear(1.0);
        }
        cmd.entity(new_e).remove::<NewBgMusic>().insert((
            CurrentGlobalMusic{handle: handle.clone()},
            AudioPlayer::new(handle.clone()),
            settings
        ));
        return;
    };
    // despawn other
    for (current_e, _) in current_iter {
        cmd.entity(current_e).despawn();
    }
    let Some(handle) = &new_music.handle else {
        // if none - current -> prev
        if new_music.instant_translation {
            cmd.entity(current_e).despawn();
        } else {
            cmd.entity(current_e).remove::<CurrentGlobalMusic>().insert(PrevGlobalMusic);
        }
        cmd.entity(new_e).despawn();
        return;
    };
    // compare with new
    if handle == &current.handle {
        cmd.entity(new_e).despawn();
        info!("Same music");
        return;
    }
    info!("New music");
    // replace to new
    let mut settings = PlaybackSettings{
        mode: PlaybackMode::Loop,
        volume: Volume::SILENT,
        ..default()
    };
    if new_music.instant_translation {
        settings.volume = Volume::Linear(1.0);
        cmd.entity(current_e).despawn();
    } else {
        cmd.entity(current_e).remove::<CurrentGlobalMusic>().insert((
            PrevGlobalMusic,
            AudioPlayer::new(handle.clone()),
        ));
    }
    cmd.entity(new_e).remove::<NewBgMusic>().insert((
        CurrentGlobalMusic{handle: handle.clone()},
        AudioPlayer::new(handle.clone()),
        settings
    ));
}
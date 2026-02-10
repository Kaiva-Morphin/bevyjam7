use std::collections::HashSet;

use bevy::prelude::*;

struct _StatesPlugin;

impl Plugin for _StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GlobalAppState>();
    }
}

#[allow(non_upper_case_globals)]
pub const OnGame: OnEnter<crate::core::states::GlobalAppState> =
    OnEnter(crate::core::states::GlobalAppState::InGame);

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GlobalAppState {
    #[default]
    AssetLoading,
    InGame,
    Defeat,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GlobalAppState = GlobalAppState::AssetLoading)]
pub enum AppLoadingAssetsSubState {
    #[default]
    Loading,
    Done,
}

#[derive(Resource, Debug)]
pub struct PreGameTasks {
    tasks: HashSet<String>,
}

impl PreGameTasks {
    pub fn add(&mut self, task: String) {
        self.tasks.insert(task);
    }
    pub fn done(&mut self, task: String) {
        self.tasks.remove(&task);
    }
    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

pub fn try_translate(
    mut next_state: ResMut<NextState<GlobalAppState>>,
    loading_state: Res<State<AppLoadingAssetsSubState>>,
    tasks: Res<PreGameTasks>,
    // s: Option<Single<Entity, With<LoadingScreenText>>>,
    // ls: Option<Single<Entity, With<LoadingScreen>>>,
) {
    if *loading_state == AppLoadingAssetsSubState::Done && tasks.is_empty() {
        next_state.set(GlobalAppState::InGame);
        // if let Some(ls) = ls {
        //     cmd.entity(*ls).despawn();
        // }
    } else {
        // if let Some(s) = s {
        //     let t = tasks.tasks.iter().join("\n");
        //     cmd.entity(*s).insert(
        //         Text::new(format!("Loading:\nAssets: {:?}\nTasks:\n{}", loading_state, t)),
        //     );
        // }
    }
}

use std::{thread::sleep, time::Duration};

use bevy::{
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        RenderApp, RenderStage,
    },
};

pub struct StutterPlugin;

#[derive(Clone)]
pub struct Stutter {
    pub probability: f32,
    pub millis: u64,
}

#[derive(Component, Clone)]
pub struct RenderStutter(pub Stutter);

#[derive(Component, Clone)]
pub struct UpdateStutter(pub Stutter);

impl ExtractComponent for RenderStutter {
    type Query = &'static Self;
    type Filter = ();

    fn extract_component(item: QueryItem<Self::Query>) -> Self {
        item.clone()
    }
}

impl Plugin for StutterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_stutter);
        app.add_plugin(ExtractComponentPlugin::<RenderStutter>::default());
        let render_app = app.get_sub_app_mut(RenderApp);
        if let Ok(render_app) = render_app {
            render_app.add_system_to_stage(RenderStage::Render, render_stutter);
        }
    }
}

fn render_stutter(stutters: Query<&RenderStutter>) {
    for stutter in stutters.iter() {
        if fastrand::f32() < stutter.0.probability {
            info!("sleeping for {} in render", stutter.0.millis);
            sleep(Duration::from_millis(stutter.0.millis));
        }
    }
}

fn update_stutter(stutters: Query<&UpdateStutter>) {
    for stutter in stutters.iter() {
        if fastrand::f32() < stutter.0.probability {
            info!("sleeping for {} in update", stutter.0.millis);
            sleep(Duration::from_millis(stutter.0.millis));
        }
    }
}

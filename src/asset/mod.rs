//! This module defines all assets for 2D Animations.
//!

use std::cmp::Ordering;

use bevy::{
    prelude::{App, Asset, AssetApp, Handle, Plugin},
    reflect::TypePath,
    utils::HashMap,
};
use thiserror::Error;

use self::asset_loader::Animation2DLoader;

pub mod asset_loader;

/// Adds support for spritesheet animation manifest files loading to the app.
pub struct Animation2DLoaderPlugin;

impl Plugin for Animation2DLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimationClip2D>()
            .init_asset::<Trickfilm>()
            .init_asset_loader::<Animation2DLoader>();
    }
}

/// AnimationClip for a 2D animation.
#[derive(Asset, Debug, TypePath)]
pub struct AnimationClip2D {
    /// Timestamps for each keyframe in seconds.
    keyframe_timestamps: Vec<f32>,
    /// An ordered list of incides of the TextureAtlas or Images that represent the frames of this animation.
    keyframes: Vec<usize>,
    /// Total duration of this animation clip in seconds.
    duration: f32,
}

/// Possible errors that can be produced by [`AnimationClip2D`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AnimationClip2DError {
    #[error("Size of keyframes and keyframe_timestamps does not match: {0} and {1}")]
    SizeMismatch(usize, usize),
    #[error("Animation clip is empty, because the size of keyframes is 0")]
    Empty(),
    #[error("Duration of {0} is insufficient to display last keyframe at {1}")]
    InsufficientDuration(f32, f32),
}

impl AnimationClip2D {
    pub fn new(
        keyframe_timestamps: Vec<f32>,
        keyframes: Vec<usize>,
        duration: f32,
    ) -> Result<Self, AnimationClip2DError> {
        let keyframe_timestamps_len = keyframe_timestamps.len();
        let keyframes_len = keyframes.len();
        if keyframe_timestamps_len != keyframes_len {
            return Err(AnimationClip2DError::SizeMismatch(
                keyframe_timestamps_len,
                keyframes_len,
            ));
        }

        if keyframe_timestamps_len == 0 {
            return Err(AnimationClip2DError::Empty());
        }

        let keyframe_timestamps_max = keyframe_timestamps
            .iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        if let Some(Ordering::Greater) = keyframe_timestamps_max.partial_cmp(&duration) {
            return Err(AnimationClip2DError::InsufficientDuration(
                *keyframe_timestamps_max,
                duration,
            ));
        }

        Ok(Self {
            keyframe_timestamps,
            keyframes,
            duration,
        })
    }

    /// Timestamps for each keyframe in seconds.
    #[inline]
    pub fn keyframe_timestamps(&self) -> &[f32] {
        &self.keyframe_timestamps
    }

    /// Ordered list of [`Keyframes2D`] elements for this animation.
    #[inline]
    pub fn keyframes(&self) -> &[usize] {
        &self.keyframes
    }

    /// Total duration of this animation clip in seconds.
    #[inline]
    pub fn duration(&self) -> f32 {
        self.duration
    }
}

/// Representation of a loaded trickfilm file.
#[derive(Asset, Debug, TypePath)]
pub struct Trickfilm {
    /// Named animations loaded from the trickfilm file.
    pub animations: HashMap<String, Handle<AnimationClip2D>>,
}

// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/archetypes/depth_image.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: A depth image.
///
/// The shape of the `TensorData` must be mappable to an `HxW` tensor.
/// Each pixel corresponds to a depth value in units specified by `meter`.
///
/// ## Example
///
/// ### Depth to 3D example
/// ```ignore
/// use ndarray::{s, Array, ShapeBuilder};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_depth_image_3d").spawn()?;
///
///     let mut image = Array::<u16, _>::from_elem((200, 300).f(), 65535);
///     image.slice_mut(s![50..150, 50..150]).fill(20000);
///     image.slice_mut(s![130..180, 100..280]).fill(45000);
///
///     let depth_image = rerun::DepthImage::try_from(image.clone())?
///         .with_meter(10000.0)
///         .with_colormap(rerun::components::Colormap::Viridis);
///
///     // If we log a pinhole camera model, the depth gets automatically back-projected to 3D
///     rec.log(
///         "world/camera",
///         &rerun::Pinhole::from_focal_length_and_resolution(
///             [200.0, 200.0],
///             [image.shape()[1] as f32, image.shape()[0] as f32],
///         ),
///     )?;
///
///     rec.log("world/camera/depth", &depth_image)?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/1200w.png">
///   <img src="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct DepthImage {
    /// The depth-image data. Should always be a rank-2 tensor.
    pub data: crate::components::TensorData,

    /// An optional floating point value that specifies how long a meter is in the native depth units.
    ///
    /// For instance: with uint16, perhaps meter=1000 which would mean you have millimeter precision
    /// and a range of up to ~65 meters (2^16 / 1000).
    ///
    /// Note that the only effect on 2D views is the physical depth values shown when hovering the image.
    /// In 3D views on the other hand, this affects where the points of the point cloud are placed.
    pub meter: Option<crate::components::DepthMeter>,

    /// Colormap to use for rendering the depth image.
    ///
    /// If not set, the depth image will be rendered using the Turbo colormap.
    pub colormap: Option<crate::components::Colormap>,

    /// Scale the radii of the points in the point cloud generated from this image.
    ///
    /// A fill ratio of 1.0 (the default) means that each point is as big as to touch the center of its neighbor
    /// if it is at the same depth, leaving no gaps.
    /// A fill ratio of 0.5 means that each point touches the edge of its neighbor if it has the same depth.
    ///
    /// TODO(#6744): This applies only to 3D views!
    pub point_fill_ratio: Option<crate::components::FillRatio>,

    /// An optional floating point value that specifies the 2D drawing order, used only if the depth image is shown as a 2D image.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    pub draw_order: Option<crate::components::DrawOrder>,
}

impl ::re_types_core::SizeBytes for DepthImage {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.data.heap_size_bytes()
            + self.meter.heap_size_bytes()
            + self.colormap.heap_size_bytes()
            + self.point_fill_ratio.heap_size_bytes()
            + self.draw_order.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::components::TensorData>::is_pod()
            && <Option<crate::components::DepthMeter>>::is_pod()
            && <Option<crate::components::Colormap>>::is_pod()
            && <Option<crate::components::FillRatio>>::is_pod()
            && <Option<crate::components::DrawOrder>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.TensorData".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.DepthImageIndicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 4usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.DepthMeter".into(),
            "rerun.components.Colormap".into(),
            "rerun.components.FillRatio".into(),
            "rerun.components.DrawOrder".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 6usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.TensorData".into(),
            "rerun.components.DepthImageIndicator".into(),
            "rerun.components.DepthMeter".into(),
            "rerun.components.Colormap".into(),
            "rerun.components.FillRatio".into(),
            "rerun.components.DrawOrder".into(),
        ]
    });

impl DepthImage {
    /// The total number of components in the archetype: 1 required, 1 recommended, 4 optional
    pub const NUM_COMPONENTS: usize = 6usize;
}

/// Indicator component for the [`DepthImage`] [`::re_types_core::Archetype`]
pub type DepthImageIndicator = ::re_types_core::GenericIndicatorComponent<DepthImage>;

impl ::re_types_core::Archetype for DepthImage {
    type Indicator = DepthImageIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.DepthImage".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Depth image"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: DepthImageIndicator = DepthImageIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let data = {
            let array = arrays_by_name
                .get("rerun.components.TensorData")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.DepthImage#data")?;
            <crate::components::TensorData>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.DepthImage#data")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.DepthImage#data")?
        };
        let meter = if let Some(array) = arrays_by_name.get("rerun.components.DepthMeter") {
            <crate::components::DepthMeter>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.DepthImage#meter")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let colormap = if let Some(array) = arrays_by_name.get("rerun.components.Colormap") {
            <crate::components::Colormap>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.DepthImage#colormap")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let point_fill_ratio = if let Some(array) = arrays_by_name.get("rerun.components.FillRatio")
        {
            <crate::components::FillRatio>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.DepthImage#point_fill_ratio")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let draw_order = if let Some(array) = arrays_by_name.get("rerun.components.DrawOrder") {
            <crate::components::DrawOrder>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.DepthImage#draw_order")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self {
            data,
            meter,
            colormap,
            point_fill_ratio,
            draw_order,
        })
    }
}

impl ::re_types_core::AsComponents for DepthImage {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.data as &dyn ComponentBatch).into()),
            self.meter
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.colormap
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.point_fill_ratio
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.draw_order
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl DepthImage {
    /// Create a new `DepthImage`.
    #[inline]
    pub fn new(data: impl Into<crate::components::TensorData>) -> Self {
        Self {
            data: data.into(),
            meter: None,
            colormap: None,
            point_fill_ratio: None,
            draw_order: None,
        }
    }

    /// An optional floating point value that specifies how long a meter is in the native depth units.
    ///
    /// For instance: with uint16, perhaps meter=1000 which would mean you have millimeter precision
    /// and a range of up to ~65 meters (2^16 / 1000).
    ///
    /// Note that the only effect on 2D views is the physical depth values shown when hovering the image.
    /// In 3D views on the other hand, this affects where the points of the point cloud are placed.
    #[inline]
    pub fn with_meter(mut self, meter: impl Into<crate::components::DepthMeter>) -> Self {
        self.meter = Some(meter.into());
        self
    }

    /// Colormap to use for rendering the depth image.
    ///
    /// If not set, the depth image will be rendered using the Turbo colormap.
    #[inline]
    pub fn with_colormap(mut self, colormap: impl Into<crate::components::Colormap>) -> Self {
        self.colormap = Some(colormap.into());
        self
    }

    /// Scale the radii of the points in the point cloud generated from this image.
    ///
    /// A fill ratio of 1.0 (the default) means that each point is as big as to touch the center of its neighbor
    /// if it is at the same depth, leaving no gaps.
    /// A fill ratio of 0.5 means that each point touches the edge of its neighbor if it has the same depth.
    ///
    /// TODO(#6744): This applies only to 3D views!
    #[inline]
    pub fn with_point_fill_ratio(
        mut self,
        point_fill_ratio: impl Into<crate::components::FillRatio>,
    ) -> Self {
        self.point_fill_ratio = Some(point_fill_ratio.into());
        self
    }

    /// An optional floating point value that specifies the 2D drawing order, used only if the depth image is shown as a 2D image.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    #[inline]
    pub fn with_draw_order(mut self, draw_order: impl Into<crate::components::DrawOrder>) -> Self {
        self.draw_order = Some(draw_order.into());
        self
    }
}

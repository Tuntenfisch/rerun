use re_log_types::Instance;
use re_query::range_zip_1x7;
use re_renderer::{LineDrawableBuilder, PickingLayerInstanceId};
use re_types::{
    archetypes::Boxes3D,
    components::{ClassId, Color, HalfSize3D, KeypointId, Position3D, Radius, Rotation3D, Text},
};
use re_viewer_context::{
    auto_color_for_entity_path, ApplicableEntities, IdentifiedViewSystem, QueryContext,
    SpaceViewSystemExecutionError, TypedComponentFallbackProvider, ViewContext,
    ViewContextCollection, ViewQuery, VisualizableEntities, VisualizableFilterContext,
    VisualizerQueryInfo, VisualizerSystem,
};

use crate::{contexts::SpatialSceneEntityContext, view_kind::SpatialSpaceViewKind};

use super::{
    entity_iterator::clamped, filter_visualizable_3d_entities,
    process_annotation_and_keypoint_slices, process_color_slice, process_labels_3d,
    process_radius_slice, SpatialViewVisualizerData, SIZE_BOOST_IN_POINTS_FOR_LINE_OUTLINES,
};

// ---

pub struct Boxes3DVisualizer(SpatialViewVisualizerData);

impl Default for Boxes3DVisualizer {
    fn default() -> Self {
        Self(SpatialViewVisualizerData::new(Some(
            SpatialSpaceViewKind::ThreeD,
        )))
    }
}

// NOTE: Do not put profile scopes in these methods. They are called for all entities and all
// timestamps within a time range -- it's _a lot_.
impl Boxes3DVisualizer {
    fn process_data<'a>(
        &mut self,
        ctx: &QueryContext<'_>,
        line_builder: &mut LineDrawableBuilder<'_>,
        query: &ViewQuery<'_>,
        ent_context: &SpatialSceneEntityContext<'_>,
        data: impl Iterator<Item = Boxes3DComponentData<'a>>,
    ) {
        let entity_path = ctx.target_entity_path;

        for data in data {
            let num_instances = data.half_sizes.len();
            if num_instances == 0 {
                continue;
            }

            let (annotation_infos, _) = process_annotation_and_keypoint_slices(
                query.latest_at,
                num_instances,
                data.half_sizes.iter().map(|_| glam::Vec3::ZERO),
                data.keypoint_ids,
                data.class_ids,
                &ent_context.annotations,
            );

            // Has not custom fallback for radius, so we use the default.
            // TODO(andreas): It would be nice to have this handle this fallback as part of the query.
            let radii =
                process_radius_slice(entity_path, num_instances, data.radii, Radius::default());
            let colors =
                process_color_slice(ctx, self, num_instances, &annotation_infos, data.colors);

            let mut line_batch = line_builder
                .batch("boxes3d")
                .depth_offset(ent_context.depth_offset)
                .world_from_obj(ent_context.world_from_entity)
                .outline_mask_ids(ent_context.highlight.overall)
                .picking_object_id(re_renderer::PickingLayerObjectId(entity_path.hash64()));

            let mut obj_space_bounding_box = macaw::BoundingBox::nothing();

            let centers =
                clamped(data.centers, num_instances).chain(std::iter::repeat(&Position3D::ZERO));
            let rotations = clamped(data.rotations, num_instances)
                .chain(std::iter::repeat(&Rotation3D::IDENTITY));
            for (i, (half_size, &center, rotation, radius, &color)) in
                itertools::izip!(data.half_sizes, centers, rotations, radii, &colors).enumerate()
            {
                obj_space_bounding_box.extend(half_size.box_min(center));
                obj_space_bounding_box.extend(half_size.box_max(center));

                let center = center.into();

                let box3d = line_batch
                    .add_box_outline_from_transform(
                        glam::Affine3A::from_scale_rotation_translation(
                            glam::Vec3::from(*half_size) * 2.0,
                            rotation.0.into(),
                            center,
                        ),
                    )
                    .color(color)
                    .radius(radius)
                    .picking_instance_id(PickingLayerInstanceId(i as _));

                if let Some(outline_mask_ids) = ent_context
                    .highlight
                    .instances
                    .get(&Instance::from(i as u64))
                {
                    box3d.outline_mask_ids(*outline_mask_ids);
                }
            }

            self.0.add_bounding_box(
                entity_path.hash(),
                obj_space_bounding_box,
                ent_context.world_from_entity,
            );

            if data.labels.len() == 1 || num_instances <= super::MAX_NUM_LABELS_PER_ENTITY {
                // If there's many boxes but only a single label, place the single label at the middle of the visualization.
                let label_positions = if data.labels.len() == 1 && num_instances > 1 {
                    // TODO(andreas): A smoothed over time (+ discontinuity detection) bounding box would be great.
                    itertools::Either::Left(std::iter::once(obj_space_bounding_box.center()))
                } else {
                    // Take center point of every box.
                    itertools::Either::Right(
                        clamped(data.centers, num_instances)
                            .chain(std::iter::repeat(&Position3D::ZERO))
                            .map(|&c| c.into()),
                    )
                };

                self.0.ui_labels.extend(process_labels_3d(
                    entity_path,
                    label_positions,
                    data.labels,
                    &colors,
                    &annotation_infos,
                    ent_context.world_from_entity,
                ));
            }
        }
    }
}

// ---

struct Boxes3DComponentData<'a> {
    // Point of views
    half_sizes: &'a [HalfSize3D],

    // Clamped to edge
    centers: &'a [Position3D],
    rotations: &'a [Rotation3D],
    colors: &'a [Color],
    radii: &'a [Radius],
    labels: &'a [Text],
    keypoint_ids: &'a [KeypointId],
    class_ids: &'a [ClassId],
}

impl IdentifiedViewSystem for Boxes3DVisualizer {
    fn identifier() -> re_viewer_context::ViewSystemIdentifier {
        "Boxes3D".into()
    }
}

impl VisualizerSystem for Boxes3DVisualizer {
    fn visualizer_query_info(&self) -> VisualizerQueryInfo {
        VisualizerQueryInfo::from_archetype::<Boxes3D>()
    }

    fn filter_visualizable_entities(
        &self,
        entities: ApplicableEntities,
        context: &dyn VisualizableFilterContext,
    ) -> VisualizableEntities {
        re_tracing::profile_function!();
        filter_visualizable_3d_entities(entities, context)
    }

    fn execute(
        &mut self,
        ctx: &ViewContext<'_>,
        view_query: &ViewQuery<'_>,
        context_systems: &ViewContextCollection,
    ) -> Result<Vec<re_renderer::QueueableDrawData>, SpaceViewSystemExecutionError> {
        let Some(render_ctx) = ctx.viewer_ctx.render_ctx else {
            return Err(SpaceViewSystemExecutionError::NoRenderContextError);
        };

        let mut line_builder = LineDrawableBuilder::new(render_ctx);
        line_builder.radius_boost_in_ui_points_for_outlines(SIZE_BOOST_IN_POINTS_FOR_LINE_OUTLINES);

        super::entity_iterator::process_archetype::<Self, Boxes3D, _>(
            ctx,
            view_query,
            context_systems,
            |ctx, spatial_ctx, results| {
                use re_space_view::RangeResultsExt as _;

                let resolver = ctx.recording().resolver();

                let half_sizes = match results.get_required_component_dense::<HalfSize3D>(resolver)
                {
                    Some(vectors) => vectors?,
                    _ => return Ok(()),
                };

                let num_boxes = half_sizes
                    .range_indexed()
                    .map(|(_, vectors)| vectors.len())
                    .sum::<usize>();
                if num_boxes == 0 {
                    return Ok(());
                }

                // Each box consists of 12 independent lines with 2 vertices each.
                line_builder.reserve_strips(num_boxes * 12)?;
                line_builder.reserve_vertices(num_boxes * 12 * 2)?;

                let centers = results.get_or_empty_dense(resolver)?;
                let rotations = results.get_or_empty_dense(resolver)?;
                let colors = results.get_or_empty_dense(resolver)?;
                let radii = results.get_or_empty_dense(resolver)?;
                let labels = results.get_or_empty_dense(resolver)?;
                let class_ids = results.get_or_empty_dense(resolver)?;
                let keypoint_ids = results.get_or_empty_dense(resolver)?;

                let data = range_zip_1x7(
                    half_sizes.range_indexed(),
                    centers.range_indexed(),
                    rotations.range_indexed(),
                    colors.range_indexed(),
                    radii.range_indexed(),
                    labels.range_indexed(),
                    class_ids.range_indexed(),
                    keypoint_ids.range_indexed(),
                )
                .map(
                    |(
                        _index,
                        half_sizes,
                        centers,
                        rotations,
                        colors,
                        radii,
                        labels,
                        class_ids,
                        keypoint_ids,
                    )| {
                        Boxes3DComponentData {
                            half_sizes,
                            centers: centers.unwrap_or_default(),
                            rotations: rotations.unwrap_or_default(),
                            colors: colors.unwrap_or_default(),
                            radii: radii.unwrap_or_default(),
                            labels: labels.unwrap_or_default(),
                            class_ids: class_ids.unwrap_or_default(),
                            keypoint_ids: keypoint_ids.unwrap_or_default(),
                        }
                    },
                );

                self.process_data(ctx, &mut line_builder, view_query, spatial_ctx, data);

                Ok(())
            },
        )?;

        Ok(vec![(line_builder.into_draw_data()?.into())])
    }

    fn data(&self) -> Option<&dyn std::any::Any> {
        Some(self.0.as_any())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_fallback_provider(&self) -> &dyn re_viewer_context::ComponentFallbackProvider {
        self
    }
}

impl TypedComponentFallbackProvider<Color> for Boxes3DVisualizer {
    fn fallback_for(&self, ctx: &QueryContext<'_>) -> Color {
        auto_color_for_entity_path(ctx.target_entity_path)
    }
}

re_viewer_context::impl_component_fallback_provider!(Boxes3DVisualizer => [Color]);

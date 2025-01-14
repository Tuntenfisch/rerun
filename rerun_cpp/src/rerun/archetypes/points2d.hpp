// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/points2d.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../components/class_id.hpp"
#include "../components/color.hpp"
#include "../components/draw_order.hpp"
#include "../components/keypoint_id.hpp"
#include "../components/position2d.hpp"
#include "../components/radius.hpp"
#include "../components/text.hpp"
#include "../data_cell.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: A 2D point cloud with positions and optional colors, radii, labels, etc.
    ///
    /// ## Examples
    ///
    /// ### Randomly distributed 2D points with varying color and radius
    /// ![image](https://static.rerun.io/point2d_random/8e8ac75373677bd72bd3f56a15e44fcab309a168/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// #include <algorithm>
    /// #include <random>
    /// #include <vector>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_points2d_random");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     std::default_random_engine gen;
    ///     std::uniform_real_distribution<float> dist_pos(-3.0f, 3.0f);
    ///     std::uniform_real_distribution<float> dist_radius(0.1f, 1.0f);
    ///     // On MSVC uint8_t distributions are not supported.
    ///     std::uniform_int_distribution<int> dist_color(0, 255);
    ///
    ///     std::vector<rerun::Position2D> points2d(10);
    ///     std::generate(points2d.begin(), points2d.end(), [&] {
    ///         return rerun::Position2D(dist_pos(gen), dist_pos(gen));
    ///     });
    ///     std::vector<rerun::Color> colors(10);
    ///     std::generate(colors.begin(), colors.end(), [&] {
    ///         return rerun::Color(
    ///             static_cast<uint8_t>(dist_color(gen)),
    ///             static_cast<uint8_t>(dist_color(gen)),
    ///             static_cast<uint8_t>(dist_color(gen))
    ///         );
    ///     });
    ///     std::vector<rerun::Radius> radii(10);
    ///     std::generate(radii.begin(), radii.end(), [&] { return dist_radius(gen); });
    ///
    ///     rec.log("random", rerun::Points2D(points2d).with_colors(colors).with_radii(radii));
    ///
    ///     // TODO(#5520): log VisualBounds2D
    /// }
    /// ```
    ///
    /// ### Log points with radii given in UI points
    /// ![image](https://static.rerun.io/point2d_ui_radius/ce804fc77300d89c348b4ab5960395171497b7ac/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_points2d_ui_radius");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     // Two blue points with scene unit radii of 0.1 and 0.3.
    ///     rec.log(
    ///         "scene_units",
    ///         rerun::Points2D({{0.0f, 0.0f}, {0.0f, 1.0f}})
    ///             // By default, radii are interpreted as world-space units.
    ///             .with_radii({0.1f, 0.3f})
    ///             .with_colors(rerun::Color(0, 0, 255))
    ///     );
    ///
    ///     // Two red points with ui point radii of 40 and 60.
    ///     // UI points are independent of zooming in Views, but are sensitive to the application UI scaling.
    ///     // For 100% ui scaling, UI points are equal to pixels.
    ///     rec.log(
    ///         "ui_points",
    ///         rerun::Points2D({{1.0f, 0.0f}, {1.0f, 1.0f}})
    ///             // rerun::Radius::ui_points produces radii that the viewer interprets as given in ui points.
    ///             .with_radii({
    ///                 rerun::Radius::ui_points(40.0f),
    ///                 rerun::Radius::ui_points(60.0f),
    ///             })
    ///             .with_colors(rerun::Color(255, 0, 0))
    ///     );
    ///
    ///     // TODO(#5521): log VisualBounds2D
    /// }
    /// ```
    struct Points2D {
        /// All the 2D positions at which the point cloud shows points.
        Collection<rerun::components::Position2D> positions;

        /// Optional radii for the points, effectively turning them into circles.
        std::optional<Collection<rerun::components::Radius>> radii;

        /// Optional colors for the points.
        std::optional<Collection<rerun::components::Color>> colors;

        /// Optional text labels for the points.
        ///
        /// If there's a single label present, it will be placed at the center of the entity.
        /// Otherwise, each instance will have its own label.
        std::optional<Collection<rerun::components::Text>> labels;

        /// An optional floating point value that specifies the 2D drawing order.
        ///
        /// Objects with higher values are drawn on top of those with lower values.
        std::optional<rerun::components::DrawOrder> draw_order;

        /// Optional class Ids for the points.
        ///
        /// The class ID provides colors and labels if not specified explicitly.
        std::optional<Collection<rerun::components::ClassId>> class_ids;

        /// Optional keypoint IDs for the points, identifying them within a class.
        ///
        /// If keypoint IDs are passed in but no class IDs were specified, the class ID will
        /// default to 0.
        /// This is useful to identify points within a single classification (which is identified
        /// with `class_id`).
        /// E.g. the classification might be 'Person' and the keypoints refer to joints on a
        /// detected skeleton.
        std::optional<Collection<rerun::components::KeypointId>> keypoint_ids;

      public:
        static constexpr const char IndicatorComponentName[] = "rerun.components.Points2DIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        Points2D() = default;
        Points2D(Points2D&& other) = default;

        explicit Points2D(Collection<rerun::components::Position2D> _positions)
            : positions(std::move(_positions)) {}

        /// Optional radii for the points, effectively turning them into circles.
        Points2D with_radii(Collection<rerun::components::Radius> _radii) && {
            radii = std::move(_radii);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional colors for the points.
        Points2D with_colors(Collection<rerun::components::Color> _colors) && {
            colors = std::move(_colors);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional text labels for the points.
        ///
        /// If there's a single label present, it will be placed at the center of the entity.
        /// Otherwise, each instance will have its own label.
        Points2D with_labels(Collection<rerun::components::Text> _labels) && {
            labels = std::move(_labels);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// An optional floating point value that specifies the 2D drawing order.
        ///
        /// Objects with higher values are drawn on top of those with lower values.
        Points2D with_draw_order(rerun::components::DrawOrder _draw_order) && {
            draw_order = std::move(_draw_order);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional class Ids for the points.
        ///
        /// The class ID provides colors and labels if not specified explicitly.
        Points2D with_class_ids(Collection<rerun::components::ClassId> _class_ids) && {
            class_ids = std::move(_class_ids);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional keypoint IDs for the points, identifying them within a class.
        ///
        /// If keypoint IDs are passed in but no class IDs were specified, the class ID will
        /// default to 0.
        /// This is useful to identify points within a single classification (which is identified
        /// with `class_id`).
        /// E.g. the classification might be 'Person' and the keypoints refer to joints on a
        /// detected skeleton.
        Points2D with_keypoint_ids(Collection<rerun::components::KeypointId> _keypoint_ids) && {
            keypoint_ids = std::move(_keypoint_ids);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::Points2D> {
        /// Serialize all set component batches.
        static Result<std::vector<DataCell>> serialize(const archetypes::Points2D& archetype);
    };
} // namespace rerun

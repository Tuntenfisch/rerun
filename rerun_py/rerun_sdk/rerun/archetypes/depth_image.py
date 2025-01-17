# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/archetypes/depth_image.fbs".

# You can extend this class by creating a "DepthImageExt" class in "depth_image_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from .. import components, datatypes
from .._baseclasses import (
    Archetype,
)
from ..error_utils import catch_and_log_exceptions
from .depth_image_ext import DepthImageExt

__all__ = ["DepthImage"]


@define(str=False, repr=False, init=False)
class DepthImage(DepthImageExt, Archetype):
    """
    **Archetype**: A depth image.

    The shape of the `TensorData` must be mappable to an `HxW` tensor.
    Each pixel corresponds to a depth value in units specified by `meter`.

    Example
    -------
    ### Depth to 3D example:
    ```python
    import numpy as np
    import rerun as rr

    depth_image = 65535 * np.ones((200, 300), dtype=np.uint16)
    depth_image[50:150, 50:150] = 20000
    depth_image[130:180, 100:280] = 45000

    rr.init("rerun_example_depth_image_3d", spawn=True)

    # If we log a pinhole camera model, the depth gets automatically back-projected to 3D
    rr.log(
        "world/camera",
        rr.Pinhole(
            width=depth_image.shape[1],
            height=depth_image.shape[0],
            focal_length=200,
        ),
    )

    # Log the tensor.
    rr.log("world/camera/depth", rr.DepthImage(depth_image, meter=10_000.0, colormap="viridis"))
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/1200w.png">
      <img src="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/full.png" width="640">
    </picture>
    </center>

    """

    def __init__(
        self: Any,
        data: datatypes.TensorDataLike,
        *,
        meter: components.DepthMeterLike | None = None,
        colormap: components.ColormapLike | None = None,
        point_fill_ratio: datatypes.Float32Like | None = None,
        draw_order: components.DrawOrderLike | None = None,
    ):
        """
        Create a new instance of the DepthImage archetype.

        Parameters
        ----------
        data:
            The depth-image data. Should always be a rank-2 tensor.
        meter:
            An optional floating point value that specifies how long a meter is in the native depth units.

            For instance: with uint16, perhaps meter=1000 which would mean you have millimeter precision
            and a range of up to ~65 meters (2^16 / 1000).

            Note that the only effect on 2D views is the physical depth values shown when hovering the image.
            In 3D views on the other hand, this affects where the points of the point cloud are placed.
        colormap:
            Colormap to use for rendering the depth image.

            If not set, the depth image will be rendered using the Turbo colormap.
        point_fill_ratio:
            Scale the radii of the points in the point cloud generated from this image.

            A fill ratio of 1.0 (the default) means that each point is as big as to touch the center of its neighbor
            if it is at the same depth, leaving no gaps.
            A fill ratio of 0.5 means that each point touches the edge of its neighbor if it has the same depth.

            TODO(#6744): This applies only to 3D views!
        draw_order:
            An optional floating point value that specifies the 2D drawing order, used only if the depth image is shown as a 2D image.

            Objects with higher values are drawn on top of those with lower values.

        """

        # You can define your own __init__ function as a member of DepthImageExt in depth_image_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(
                data=data, meter=meter, colormap=colormap, point_fill_ratio=point_fill_ratio, draw_order=draw_order
            )
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            data=None,  # type: ignore[arg-type]
            meter=None,  # type: ignore[arg-type]
            colormap=None,  # type: ignore[arg-type]
            point_fill_ratio=None,  # type: ignore[arg-type]
            draw_order=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> DepthImage:
        """Produce an empty DepthImage, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    data: components.TensorDataBatch = field(
        metadata={"component": "required"},
        converter=DepthImageExt.data__field_converter_override,  # type: ignore[misc]
    )
    # The depth-image data. Should always be a rank-2 tensor.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    meter: components.DepthMeterBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.DepthMeterBatch._optional,  # type: ignore[misc]
    )
    # An optional floating point value that specifies how long a meter is in the native depth units.
    #
    # For instance: with uint16, perhaps meter=1000 which would mean you have millimeter precision
    # and a range of up to ~65 meters (2^16 / 1000).
    #
    # Note that the only effect on 2D views is the physical depth values shown when hovering the image.
    # In 3D views on the other hand, this affects where the points of the point cloud are placed.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    colormap: components.ColormapBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.ColormapBatch._optional,  # type: ignore[misc]
    )
    # Colormap to use for rendering the depth image.
    #
    # If not set, the depth image will be rendered using the Turbo colormap.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    point_fill_ratio: components.FillRatioBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.FillRatioBatch._optional,  # type: ignore[misc]
    )
    # Scale the radii of the points in the point cloud generated from this image.
    #
    # A fill ratio of 1.0 (the default) means that each point is as big as to touch the center of its neighbor
    # if it is at the same depth, leaving no gaps.
    # A fill ratio of 0.5 means that each point touches the edge of its neighbor if it has the same depth.
    #
    # TODO(#6744): This applies only to 3D views!
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    draw_order: components.DrawOrderBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.DrawOrderBatch._optional,  # type: ignore[misc]
    )
    # An optional floating point value that specifies the 2D drawing order, used only if the depth image is shown as a 2D image.
    #
    # Objects with higher values are drawn on top of those with lower values.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]

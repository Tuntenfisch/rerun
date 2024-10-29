# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/map_background.fbs".

# You can extend this class by creating a "MapBackgroundExt" class in "map_background_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from ..._baseclasses import (
    Archetype,
)
from ...blueprint import components as blueprint_components
from ...error_utils import catch_and_log_exceptions

__all__ = ["MapBackground"]


@define(str=False, repr=False, init=False)
class MapBackground(Archetype):
    """**Archetype**: Configuration for the background map of the map view."""

    def __init__(self: Any, provider: blueprint_components.MapProviderLike):
        """
        Create a new instance of the MapBackground archetype.

        Parameters
        ----------
        provider:
            Map provider and style to use.

            **Note**: Requires a Mapbox API key in the `RERUN_MAPBOX_ACCESS_TOKEN` environment variable.

        """

        # You can define your own __init__ function as a member of MapBackgroundExt in map_background_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(provider=provider)
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            provider=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> MapBackground:
        """Produce an empty MapBackground, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    provider: blueprint_components.MapProviderBatch = field(
        metadata={"component": "required"},
        converter=blueprint_components.MapProviderBatch._required,  # type: ignore[misc]
    )
    # Map provider and style to use.
    #
    # **Note**: Requires a Mapbox API key in the `RERUN_MAPBOX_ACCESS_TOKEN` environment variable.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]
# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/components/range1d.fbs".

# You can extend this class by creating a "Range1DExt" class in "range1d_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import ComponentBatchMixin

__all__ = ["Range1D", "Range1DBatch", "Range1DType"]


class Range1D(datatypes.Range1D):
    """**Component**: A 1D range, specifying a lower and upper bound."""

    # You can define your own __init__ function as a member of Range1DExt in range1d_ext.py

    # Note: there are no fields here because Range1D delegates to datatypes.Range1D
    pass


class Range1DType(datatypes.Range1DType):
    _TYPE_NAME: str = "rerun.components.Range1D"


class Range1DBatch(datatypes.Range1DBatch, ComponentBatchMixin):
    _ARROW_TYPE = Range1DType()

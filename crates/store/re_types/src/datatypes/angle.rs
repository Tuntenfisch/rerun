// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/angle.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Datatype**: Angle in radians.
#[derive(Clone, Debug, Copy, Default, PartialEq, PartialOrd, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct Angle {
    /// Angle in radians. One turn is equal to 2π (or τ) radians.
    pub radians: f32,
}

impl ::re_types_core::SizeBytes for Angle {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.radians.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <f32>::is_pod()
    }
}

impl From<f32> for Angle {
    #[inline]
    fn from(radians: f32) -> Self {
        Self { radians }
    }
}

impl From<Angle> for f32 {
    #[inline]
    fn from(value: Angle) -> Self {
        value.radians
    }
}

::re_types_core::macros::impl_into_cow!(Angle);

impl ::re_types_core::Loggable for Angle {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.Angle".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::Float32
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        #![allow(clippy::manual_is_variant_and)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, radians): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| datum.into_owned().radians);
                    (datum.is_some(), datum)
                })
                .unzip();
            let radians_bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            PrimitiveArray::new(
                Self::arrow_datatype(),
                radians.into_iter().map(|v| v.unwrap_or_default()).collect(),
                radians_bitmap,
            )
            .boxed()
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok(arrow_data
            .as_any()
            .downcast_ref::<Float32Array>()
            .ok_or_else(|| {
                let expected = Self::arrow_datatype();
                let actual = arrow_data.data_type().clone();
                DeserializationError::datatype_mismatch(expected, actual)
            })
            .with_context("rerun.datatypes.Angle#radians")?
            .into_iter()
            .map(|opt| opt.copied())
            .map(|v| v.ok_or_else(DeserializationError::missing_data))
            .map(|res| res.map(|radians| Some(Self { radians })))
            .collect::<DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.datatypes.Angle#radians")
            .with_context("rerun.datatypes.Angle")?)
    }

    #[inline]
    fn from_arrow(arrow_data: &dyn arrow2::array::Array) -> DeserializationResult<Vec<Self>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        if let Some(validity) = arrow_data.validity() {
            if validity.unset_bits() != 0 {
                return Err(DeserializationError::missing_data());
            }
        }
        Ok({
            let slice = arrow_data
                .as_any()
                .downcast_ref::<Float32Array>()
                .ok_or_else(|| {
                    let expected = DataType::Float32;
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.datatypes.Angle#radians")?
                .values()
                .as_slice();
            {
                slice
                    .iter()
                    .copied()
                    .map(|radians| Self { radians })
                    .collect::<Vec<_>>()
            }
        })
    }
}

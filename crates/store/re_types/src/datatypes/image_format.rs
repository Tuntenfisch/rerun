// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/image_format.fbs".

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

/// **Datatype**: The metadata describing the contents of a [`components::ImageBuffer`][crate::components::ImageBuffer].
#[derive(Clone, Debug, Default, Copy, PartialEq, Eq, Hash)]
pub struct ImageFormat {
    /// The width of the image in pixels.
    pub width: u32,

    /// The height of the image in pixels.
    pub height: u32,

    /// Used mainly for chroma downsampled formats and differing number of bits per channel.
    ///
    /// If specified, this takes precedence over both [`datatypes::ColorModel`][crate::datatypes::ColorModel] and [`datatypes::ChannelDatatype`][crate::datatypes::ChannelDatatype] (which are ignored).
    pub pixel_format: Option<crate::datatypes::PixelFormat>,

    /// L, RGB, RGBA, …
    ///
    /// Also requires a [`datatypes::ChannelDatatype`][crate::datatypes::ChannelDatatype] to fully specify the pixel format.
    pub color_model: Option<crate::datatypes::ColorModel>,

    /// The data type of each channel (e.g. the red channel) of the image data (U8, F16, …).
    ///
    /// Also requires a [`datatypes::ColorModel`][crate::datatypes::ColorModel] to fully specify the pixel format.
    pub channel_datatype: Option<crate::datatypes::ChannelDatatype>,
}

impl ::re_types_core::SizeBytes for ImageFormat {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.width.heap_size_bytes()
            + self.height.heap_size_bytes()
            + self.pixel_format.heap_size_bytes()
            + self.color_model.heap_size_bytes()
            + self.channel_datatype.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <u32>::is_pod()
            && <u32>::is_pod()
            && <Option<crate::datatypes::PixelFormat>>::is_pod()
            && <Option<crate::datatypes::ColorModel>>::is_pod()
            && <Option<crate::datatypes::ChannelDatatype>>::is_pod()
    }
}

::re_types_core::macros::impl_into_cow!(ImageFormat);

impl ::re_types_core::Loggable for ImageFormat {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.ImageFormat".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::Struct(std::sync::Arc::new(vec![
            Field::new("width", DataType::UInt32, false),
            Field::new("height", DataType::UInt32, false),
            Field::new(
                "pixel_format",
                <crate::datatypes::PixelFormat>::arrow_datatype(),
                true,
            ),
            Field::new(
                "color_model",
                <crate::datatypes::ColorModel>::arrow_datatype(),
                true,
            ),
            Field::new(
                "channel_datatype",
                <crate::datatypes::ChannelDatatype>::arrow_datatype(),
                true,
            ),
        ]))
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
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                Self::arrow_datatype(),
                vec![
                    {
                        let (somes, width): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.width.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let width_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            DataType::UInt32,
                            width.into_iter().map(|v| v.unwrap_or_default()).collect(),
                            width_bitmap,
                        )
                        .boxed()
                    },
                    {
                        let (somes, height): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.height.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let height_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            DataType::UInt32,
                            height.into_iter().map(|v| v.unwrap_or_default()).collect(),
                            height_bitmap,
                        )
                        .boxed()
                    },
                    {
                        let (somes, pixel_format): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum
                                    .as_ref()
                                    .map(|datum| datum.pixel_format.clone())
                                    .flatten();
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let pixel_format_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = pixel_format_bitmap;
                            crate::datatypes::PixelFormat::to_arrow_opt(pixel_format)?
                        }
                    },
                    {
                        let (somes, color_model): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum
                                    .as_ref()
                                    .map(|datum| datum.color_model.clone())
                                    .flatten();
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let color_model_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = color_model_bitmap;
                            crate::datatypes::ColorModel::to_arrow_opt(color_model)?
                        }
                    },
                    {
                        let (somes, channel_datatype): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum
                                    .as_ref()
                                    .map(|datum| datum.channel_datatype.clone())
                                    .flatten();
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let channel_datatype_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = channel_datatype_bitmap;
                            crate::datatypes::ChannelDatatype::to_arrow_opt(channel_datatype)?
                        }
                    },
                ],
                bitmap,
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
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::StructArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.datatypes.ImageFormat")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.values());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let width = {
                    if !arrays_by_name.contains_key("width") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "width",
                        ))
                        .with_context("rerun.datatypes.ImageFormat");
                    }
                    let arrow_data = &**arrays_by_name["width"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<UInt32Array>()
                        .ok_or_else(|| {
                            let expected = DataType::UInt32;
                            let actual = arrow_data.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.datatypes.ImageFormat#width")?
                        .into_iter()
                        .map(|opt| opt.copied())
                };
                let height = {
                    if !arrays_by_name.contains_key("height") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "height",
                        ))
                        .with_context("rerun.datatypes.ImageFormat");
                    }
                    let arrow_data = &**arrays_by_name["height"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<UInt32Array>()
                        .ok_or_else(|| {
                            let expected = DataType::UInt32;
                            let actual = arrow_data.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.datatypes.ImageFormat#height")?
                        .into_iter()
                        .map(|opt| opt.copied())
                };
                let pixel_format = {
                    if !arrays_by_name.contains_key("pixel_format") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "pixel_format",
                        ))
                        .with_context("rerun.datatypes.ImageFormat");
                    }
                    let arrow_data = &**arrays_by_name["pixel_format"];
                    crate::datatypes::PixelFormat::from_arrow_opt(arrow_data)
                        .with_context("rerun.datatypes.ImageFormat#pixel_format")?
                        .into_iter()
                };
                let color_model = {
                    if !arrays_by_name.contains_key("color_model") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "color_model",
                        ))
                        .with_context("rerun.datatypes.ImageFormat");
                    }
                    let arrow_data = &**arrays_by_name["color_model"];
                    crate::datatypes::ColorModel::from_arrow_opt(arrow_data)
                        .with_context("rerun.datatypes.ImageFormat#color_model")?
                        .into_iter()
                };
                let channel_datatype = {
                    if !arrays_by_name.contains_key("channel_datatype") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "channel_datatype",
                        ))
                        .with_context("rerun.datatypes.ImageFormat");
                    }
                    let arrow_data = &**arrays_by_name["channel_datatype"];
                    crate::datatypes::ChannelDatatype::from_arrow_opt(arrow_data)
                        .with_context("rerun.datatypes.ImageFormat#channel_datatype")?
                        .into_iter()
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(width, height, pixel_format, color_model, channel_datatype),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(
                        |(width, height, pixel_format, color_model, channel_datatype)| {
                            Ok(Self {
                                width: width
                                    .ok_or_else(DeserializationError::missing_data)
                                    .with_context("rerun.datatypes.ImageFormat#width")?,
                                height: height
                                    .ok_or_else(DeserializationError::missing_data)
                                    .with_context("rerun.datatypes.ImageFormat#height")?,
                                pixel_format,
                                color_model,
                                channel_datatype,
                            })
                        },
                    )
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.datatypes.ImageFormat")?
            }
        })
    }
}

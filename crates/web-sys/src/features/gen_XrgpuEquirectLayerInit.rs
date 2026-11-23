#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "XRGPUEquirectLayerInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrgpuEquirectLayerInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrgpuEquirectLayerInit;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Get the `colorFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureFormat`, `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "colorFormat")]
    pub fn get_color_format(this: &XrgpuEquirectLayerInit) -> GpuTextureFormat;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Change the `colorFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureFormat`, `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "colorFormat")]
    pub fn set_color_format(this: &XrgpuEquirectLayerInit, val: GpuTextureFormat);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Get the `depthStencilFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureFormat`, `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "depthStencilFormat")]
    pub fn get_depth_stencil_format(this: &XrgpuEquirectLayerInit) -> Option<GpuTextureFormat>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Change the `depthStencilFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureFormat`, `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "depthStencilFormat")]
    pub fn set_depth_stencil_format(this: &XrgpuEquirectLayerInit, val: Option<GpuTextureFormat>);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isStatic` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "isStatic")]
    pub fn get_is_static(this: &XrgpuEquirectLayerInit) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isStatic` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "isStatic")]
    pub fn set_is_static(this: &XrgpuEquirectLayerInit, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mipLevels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "mipLevels")]
    pub fn get_mip_levels(this: &XrgpuEquirectLayerInit) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mipLevels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "mipLevels")]
    pub fn set_mip_levels(this: &XrgpuEquirectLayerInit, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSpace")]
    #[doc = "Get the `space` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSpace`, `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "space")]
    pub fn get_space(this: &XrgpuEquirectLayerInit) -> XrSpace;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSpace")]
    #[doc = "Change the `space` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSpace`, `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "space")]
    pub fn set_space(this: &XrgpuEquirectLayerInit, val: &XrSpace);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `textureUsage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "textureUsage")]
    pub fn get_texture_usage(this: &XrgpuEquirectLayerInit) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `textureUsage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "textureUsage")]
    pub fn set_texture_usage(this: &XrgpuEquirectLayerInit, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `viewPixelHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "viewPixelHeight")]
    pub fn get_view_pixel_height(this: &XrgpuEquirectLayerInit) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `viewPixelHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "viewPixelHeight")]
    pub fn set_view_pixel_height(this: &XrgpuEquirectLayerInit, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `viewPixelWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "viewPixelWidth")]
    pub fn get_view_pixel_width(this: &XrgpuEquirectLayerInit) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `viewPixelWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "viewPixelWidth")]
    pub fn set_view_pixel_width(this: &XrgpuEquirectLayerInit, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `centralHorizontalAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "centralHorizontalAngle")]
    pub fn get_central_horizontal_angle(this: &XrgpuEquirectLayerInit) -> Option<f32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `centralHorizontalAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "centralHorizontalAngle")]
    pub fn set_central_horizontal_angle(this: &XrgpuEquirectLayerInit, val: f32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `lowerVerticalAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "lowerVerticalAngle")]
    pub fn get_lower_vertical_angle(this: &XrgpuEquirectLayerInit) -> Option<f32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `lowerVerticalAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "lowerVerticalAngle")]
    pub fn set_lower_vertical_angle(this: &XrgpuEquirectLayerInit, val: f32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `radius` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "radius")]
    pub fn get_radius(this: &XrgpuEquirectLayerInit) -> Option<f32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `radius` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "radius")]
    pub fn set_radius(this: &XrgpuEquirectLayerInit, val: f32);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrRigidTransform")]
    #[doc = "Get the `transform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRigidTransform`, `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "transform")]
    pub fn get_transform(this: &XrgpuEquirectLayerInit) -> Option<XrRigidTransform>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrRigidTransform")]
    #[doc = "Change the `transform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRigidTransform`, `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "transform")]
    pub fn set_transform(this: &XrgpuEquirectLayerInit, val: Option<&XrRigidTransform>);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `upperVerticalAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "upperVerticalAngle")]
    pub fn get_upper_vertical_angle(this: &XrgpuEquirectLayerInit) -> Option<f32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `upperVerticalAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "upperVerticalAngle")]
    pub fn set_upper_vertical_angle(this: &XrgpuEquirectLayerInit, val: f32);
}
#[cfg(web_sys_unstable_apis)]
impl XrgpuEquirectLayerInit {
    #[cfg(all(feature = "GpuTextureFormat", feature = "XrSpace",))]
    #[doc = "Construct a new `XrgpuEquirectLayerInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureFormat`, `XrSpace`, `XrgpuEquirectLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        color_format: GpuTextureFormat,
        space: &XrSpace,
        view_pixel_height: u32,
        view_pixel_width: u32,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_color_format(color_format);
        ret.set_space(space);
        ret.set_view_pixel_height(view_pixel_height);
        ret.set_view_pixel_width(view_pixel_width);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[deprecated = "Use `set_color_format()` instead."]
    pub fn color_format(&mut self, val: GpuTextureFormat) -> &mut Self {
        self.set_color_format(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[deprecated = "Use `set_depth_stencil_format()` instead."]
    pub fn depth_stencil_format(&mut self, val: Option<GpuTextureFormat>) -> &mut Self {
        self.set_depth_stencil_format(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_is_static()` instead."]
    pub fn is_static(&mut self, val: bool) -> &mut Self {
        self.set_is_static(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_mip_levels()` instead."]
    pub fn mip_levels(&mut self, val: u32) -> &mut Self {
        self.set_mip_levels(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSpace")]
    #[deprecated = "Use `set_space()` instead."]
    pub fn space(&mut self, val: &XrSpace) -> &mut Self {
        self.set_space(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_texture_usage()` instead."]
    pub fn texture_usage(&mut self, val: u32) -> &mut Self {
        self.set_texture_usage(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_view_pixel_height()` instead."]
    pub fn view_pixel_height(&mut self, val: u32) -> &mut Self {
        self.set_view_pixel_height(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_view_pixel_width()` instead."]
    pub fn view_pixel_width(&mut self, val: u32) -> &mut Self {
        self.set_view_pixel_width(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_central_horizontal_angle()` instead."]
    pub fn central_horizontal_angle(&mut self, val: f32) -> &mut Self {
        self.set_central_horizontal_angle(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_lower_vertical_angle()` instead."]
    pub fn lower_vertical_angle(&mut self, val: f32) -> &mut Self {
        self.set_lower_vertical_angle(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_radius()` instead."]
    pub fn radius(&mut self, val: f32) -> &mut Self {
        self.set_radius(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrRigidTransform")]
    #[deprecated = "Use `set_transform()` instead."]
    pub fn transform(&mut self, val: Option<&XrRigidTransform>) -> &mut Self {
        self.set_transform(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_upper_vertical_angle()` instead."]
    pub fn upper_vertical_angle(&mut self, val: f32) -> &mut Self {
        self.set_upper_vertical_angle(val);
        self
    }
}

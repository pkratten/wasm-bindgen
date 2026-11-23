#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "XRGPUCubeLayerInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrgpuCubeLayerInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrgpuCubeLayerInit;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Get the `colorFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureFormat`, `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "colorFormat")]
    pub fn get_color_format(this: &XrgpuCubeLayerInit) -> GpuTextureFormat;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Change the `colorFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureFormat`, `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "colorFormat")]
    pub fn set_color_format(this: &XrgpuCubeLayerInit, val: GpuTextureFormat);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Get the `depthStencilFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureFormat`, `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "depthStencilFormat")]
    pub fn get_depth_stencil_format(this: &XrgpuCubeLayerInit) -> Option<GpuTextureFormat>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Change the `depthStencilFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureFormat`, `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "depthStencilFormat")]
    pub fn set_depth_stencil_format(this: &XrgpuCubeLayerInit, val: Option<GpuTextureFormat>);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isStatic` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "isStatic")]
    pub fn get_is_static(this: &XrgpuCubeLayerInit) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isStatic` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "isStatic")]
    pub fn set_is_static(this: &XrgpuCubeLayerInit, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mipLevels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "mipLevels")]
    pub fn get_mip_levels(this: &XrgpuCubeLayerInit) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mipLevels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "mipLevels")]
    pub fn set_mip_levels(this: &XrgpuCubeLayerInit, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSpace")]
    #[doc = "Get the `space` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSpace`, `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "space")]
    pub fn get_space(this: &XrgpuCubeLayerInit) -> XrSpace;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSpace")]
    #[doc = "Change the `space` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSpace`, `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "space")]
    pub fn set_space(this: &XrgpuCubeLayerInit, val: &XrSpace);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `textureUsage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "textureUsage")]
    pub fn get_texture_usage(this: &XrgpuCubeLayerInit) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `textureUsage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "textureUsage")]
    pub fn set_texture_usage(this: &XrgpuCubeLayerInit, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `viewPixelHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "viewPixelHeight")]
    pub fn get_view_pixel_height(this: &XrgpuCubeLayerInit) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `viewPixelHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "viewPixelHeight")]
    pub fn set_view_pixel_height(this: &XrgpuCubeLayerInit, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `viewPixelWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "viewPixelWidth")]
    pub fn get_view_pixel_width(this: &XrgpuCubeLayerInit) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `viewPixelWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "viewPixelWidth")]
    pub fn set_view_pixel_width(this: &XrgpuCubeLayerInit, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomPointReadOnly")]
    #[doc = "Get the `orientation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`, `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "orientation")]
    pub fn get_orientation(this: &XrgpuCubeLayerInit) -> Option<DomPointReadOnly>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomPointReadOnly")]
    #[doc = "Change the `orientation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`, `XrgpuCubeLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "orientation")]
    pub fn set_orientation(this: &XrgpuCubeLayerInit, val: Option<&DomPointReadOnly>);
}
#[cfg(web_sys_unstable_apis)]
impl XrgpuCubeLayerInit {
    #[cfg(all(feature = "GpuTextureFormat", feature = "XrSpace",))]
    #[doc = "Construct a new `XrgpuCubeLayerInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureFormat`, `XrSpace`, `XrgpuCubeLayerInit`*"]
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
    #[cfg(feature = "DomPointReadOnly")]
    #[deprecated = "Use `set_orientation()` instead."]
    pub fn orientation(&mut self, val: Option<&DomPointReadOnly>) -> &mut Self {
        self.set_orientation(val);
        self
    }
}

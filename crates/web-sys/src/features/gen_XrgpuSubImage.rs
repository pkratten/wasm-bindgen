#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "XRGPUSubImage",
        typescript_type = "XRGPUSubImage"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrgpuSubImage` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRGPUSubImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuSubImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrgpuSubImage;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTexture")]
    #[wasm_bindgen(method, getter, js_class = "XRGPUSubImage", js_name = "colorTexture")]
    #[doc = "Getter for the `colorTexture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRGPUSubImage/colorTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`, `XrgpuSubImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn color_texture(this: &XrgpuSubImage) -> GpuTexture;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTexture")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "XRGPUSubImage",
        js_name = "depthStencilTexture"
    )]
    #[doc = "Getter for the `depthStencilTexture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRGPUSubImage/depthStencilTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`, `XrgpuSubImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_stencil_texture(this: &XrgpuSubImage) -> Option<GpuTexture>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTexture")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "XRGPUSubImage",
        js_name = "motionVectorTexture"
    )]
    #[doc = "Getter for the `motionVectorTexture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRGPUSubImage/motionVectorTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`, `XrgpuSubImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn motion_vector_texture(this: &XrgpuSubImage) -> Option<GpuTexture>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureViewDescriptor")]
    #[wasm_bindgen(method, js_class = "XRGPUSubImage", js_name = "getViewDescriptor")]
    #[doc = "The `getViewDescriptor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRGPUSubImage/getViewDescriptor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`, `XrgpuSubImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_view_descriptor(this: &XrgpuSubImage) -> GpuTextureViewDescriptor;
}

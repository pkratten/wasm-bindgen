#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "XRGPUBinding",
        typescript_type = "XRGPUBinding"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrgpuBinding` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRGPUBinding)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuBinding`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrgpuBinding;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "XRGPUBinding",
        js_name = "nativeProjectionScaleFactor"
    )]
    #[doc = "Getter for the `nativeProjectionScaleFactor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRGPUBinding/nativeProjectionScaleFactor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuBinding`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn native_projection_scale_factor(this: &XrgpuBinding) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "XRGPUBinding", js_name = "usesDepthValues")]
    #[doc = "Getter for the `usesDepthValues` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRGPUBinding/usesDepthValues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrgpuBinding`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn uses_depth_values(this: &XrgpuBinding) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuDevice", feature = "XrSession",))]
    #[wasm_bindgen(catch, constructor, js_class = "XRGPUBinding")]
    #[doc = "The `new XrgpuBinding(..)` constructor, creating a new instance of `XrgpuBinding`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRGPUBinding/XRGPUBinding)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `XrSession`, `XrgpuBinding`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(session: &XrSession, device: &GpuDevice) -> Result<XrgpuBinding, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[wasm_bindgen(method, js_class = "XRGPUBinding", js_name = "getPreferredColorFormat")]
    #[doc = "The `getPreferredColorFormat()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRGPUBinding/getPreferredColorFormat)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureFormat`, `XrgpuBinding`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_preferred_color_format(this: &XrgpuBinding) -> GpuTextureFormat;
}

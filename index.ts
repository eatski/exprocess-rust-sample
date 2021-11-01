import * as app from "./pkg/roll_role";
import * as js from "./js";
//@ts-expect-error
window["_wasm_js_bridge"] = js
app.start(process.env.BUILD_MODE === "dev" ? app.AppMode.Dev : app.AppMode.Production);
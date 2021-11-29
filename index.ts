const wasmPromise = import("./pkg/roll_role");
import("./js").then(js => {
    //@ts-expect-error
    window["_wasm_js_bridge"] = js
    wasmPromise.then(app => app.start(process.env.BUILD_MODE === "dev" ? app.AppMode.Dev : app.AppMode.Production))
})

export {}
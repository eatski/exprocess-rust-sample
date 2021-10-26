import * as app from "./pkg/router";
import * as repo from "./firestore";
//@ts-expect-error
window["_wasm_js_bridge"] = repo
app.main()
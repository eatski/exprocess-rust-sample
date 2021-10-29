import * as app from "./pkg/roll_role";
import * as repo from "./firestore";
//@ts-expect-error
window["_wasm_js_bridge"] = repo
app.main()
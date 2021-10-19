import { connectFirestoreEmulator } from "firebase/firestore";
import { getStore } from "./js/firestore";
const db = getStore();
connectFirestoreEmulator(db,"localhost",3000);
export * from "./index";

import { initializeApp } from "firebase/app";
import { getFirestore } from "firebase/firestore";

if(!process.env.FIREBASE_CONFIG){
    throw new Error("No FIREBASE_CONFIG")
}
const firebaseConfig = JSON.parse(process.env.FIREBASE_CONFIG);
const app = initializeApp(firebaseConfig);
const db = getFirestore(app);


export const getStore = () => {
    return db
}
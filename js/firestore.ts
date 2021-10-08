
import { initializeApp, FirebaseApp } from "firebase/app";
import { Firestore, getFirestore } from "firebase/firestore";

const firebaseConfig = {
    apiKey: "AIzaSyD12rp1aJ_I59NZphs-nr4cbarLOlnBdW8",
    authDomain: "confusers-68c94.firebaseapp.com",
    projectId: "confusers-68c94",
    storageBucket: "confusers-68c94.appspot.com",
    messagingSenderId: "261511961647",
    appId: "1:261511961647:web:c8a81333a78eece6023ada",
    measurementId: "G-R7RCY9J3M9"
};

let db : Firestore | undefined = undefined;
export const getStore = () => {
    if(!db){
        const app = initializeApp(firebaseConfig);
        db = getFirestore(app)
    
    }
    return db
}
import { collection,doc,addDoc,onSnapshot } from "@firebase/firestore";
import { getStore } from "./firestore";

export const registerMember = (roomId:string,name:string) => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    const members = collection(room,"members");
    addDoc(members,{name})
}

export const syncMember = (roomId:string,listener:(json:string) => void) => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    const members = collection(room,"members");
    onSnapshot(members,(snapshot) => {
        const json = JSON.stringify(snapshot.docs.map(doc => ({...doc.data(),id:doc.id})));
        listener(json);
    })
}
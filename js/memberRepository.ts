import { collection,doc,setDoc,onSnapshot } from "@firebase/firestore";
import { getStore } from "./firestore";

const toYourIdKey = (roomId: string) => `${roomId}:yourid`;

export const registerMember = (roomId:string,name:string) => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    const members = collection(room,"members");
    const member = doc(members);
    window.localStorage.setItem(toYourIdKey(roomId),member.id);
    setDoc(member,{name})
}

export const syncMember = (roomId:string,listener:(json:string) => void) => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    const members = collection(room,"members");
    onSnapshot(members,(snapshot) => {
        const yourId = window.localStorage.getItem(toYourIdKey(roomId));
        const json = JSON.stringify(
            snapshot.docs.map(doc => ({
                ...doc.data(),
                id:doc.id,
                you:yourId === doc.id
            }))
        );
        listener(json);
    })
}
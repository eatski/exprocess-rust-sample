import { collection,addDoc,doc,onSnapshot } from "@firebase/firestore";
import { getStore } from "./firestore";

export const createRoom = async ():Promise<string> => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = await addDoc(rooms,{phase:"MEETING"});
    return room.id;
}

export const syncRoom = (roomId:string,listener:(phase:string | null) => void) => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    onSnapshot(room, (data) => {
        const phase = data.exists() ? data.data().phase : null
        listener(phase);
    })
}
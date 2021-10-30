import { collection,doc,onSnapshot,setDoc,runTransaction } from "firebase/firestore";
import { getStore } from "./firestore";
import { getYourId } from "./yourid";

export const openRoom = async (roomId:string,hostMemberId: string):Promise<void> => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    await setDoc(room,{
        phase:"MEETING",
        host: hostMemberId
    });
}

export const syncRoom = (roomId:string,listener:(roomData:string | null) => void): () => void => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    return onSnapshot(room, (data) => {
        const yourId = getYourId(roomId);
        const room: string | null = data.exists() ? 
            JSON.stringify({ 
                phase: data.data().phase,
                is_host: data.data().host === yourId
            }) : null
        listener(room);
    })
}

export const startRoom = (roomId:string) => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    runTransaction(db,async (t) => {
        const data = await t.get(room);
        const newData = {
            ...data.data(),
            phase:"STARTED"
        }
        await t.set(room,newData);
    })
}
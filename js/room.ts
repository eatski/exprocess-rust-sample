import { collection,doc,onSnapshot,setDoc } from "@firebase/firestore";
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

export const publishRoomId = () => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms);
    return room.id;
}

export const syncRoom = (roomId:string,listener:(roomData:string | null) => void) => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    onSnapshot(room, (data) => {
        const yourId = getYourId(roomId);
        const room: string | null = data.exists() ? 
            JSON.stringify({ 
                phase: data.data().phase,
                is_host: data.data().host === yourId
            }) : null
        console.log(room);
        listener(room);
    })
}

export const startRoom = (roomId:string) => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    setDoc(room,{phase:"STARTED"});
}
import { collection,addDoc } from "@firebase/firestore";
import { getStore } from "./firestore";

export const createRoom = async ():Promise<string> => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = await addDoc(rooms,{phase:"meeting"});
    return room.id;
}
import { collection,doc,onSnapshot,setDoc,runTransaction, writeBatch } from "firebase/firestore";
import { getStore } from "./firestore";
import { getYourId, setYourId } from "./yourid";

/**
 * FIXME: バッチのせいで色々ごちゃってる
 * @param roomId 
 * @param hostUserName 
 */
export const createRoom = async (roomId:string,hostUserName:string):Promise<void> => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    const members = collection(room,"members");
    const hostMember = doc(members);
    const batch = writeBatch(db);
    batch.set(hostMember,{
        name: hostUserName
    });
    batch.set(room, {
        phase:"MEETING",
        host: hostMember.id
    })
    // commit こけた時のハンドリング 本音を言うとコミット成功判定とローカルDB書き換えの間に実行したいぽよ
    setYourId(roomId,hostMember.id);
    await batch.commit();
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
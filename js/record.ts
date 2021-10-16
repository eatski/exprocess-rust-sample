import { addDoc, collection, doc, onSnapshot } from "firebase/firestore";
import { getStore } from "./firestore";

export const pushRecord = (roomId:string,recordId:string,commandJson:string,resultJson:string) => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    const records = collection(room,"records");
    addDoc(records,{
        id:recordId,
        command:commandJson,
        result: resultJson
    })
}

export const syncRecordUpdate = (roomId:string,listener: (recordId:string,commandJson:string,resultJson:string) => void) => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    const records = collection(room,"records");
    onSnapshot(records,(snapshot) => {
        snapshot
            .docChanges()
            .filter(change => change.type === "added")
            .forEach(change => {
                const data = change.doc.data();
                listener(data.id,data.command,data.result);
            })
    })
}
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

export const syncRecordUpdate = (roomId:string,listener: (recordsJson:string) => void) => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    const records = collection(room,"records");
    onSnapshot(records,(snapshot) => {
        const recordsObj = snapshot
            .docChanges()
            .filter(change => change.type === "added")
            .map(change => {
                const data = change.doc.data();
                return {
                    id: data.id,
                    command: JSON.parse(data.command),
                    result: JSON.parse(data.result)
                }
            });
        listener(JSON.stringify(recordsObj));
    })
}
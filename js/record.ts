import { collection, doc, onSnapshot,runTransaction,getDocs,orderBy,query } from "firebase/firestore";
import { getStore } from "./firestore";

export const pushRecord = (roomId:string,recordId:string,commandJson:string,resultJson:string) => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    const records = collection(room,"records");
    const newRecordRef = doc(records)
    runTransaction(db,async (t) => {
        const recordsData = await getDocs(records);
        const size = recordsData.size;
        await t.set(newRecordRef,{
            id:recordId,
            command:commandJson,
            result: resultJson,
            seq_no: size
        })
    });
}

export const syncRecordUpdate = (roomId:string,listener: (recordsJson:string) => void) : () => void => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    const records = collection(room,"records");
    const orderedRecord = query(records,orderBy("seq_no"));
    return onSnapshot(orderedRecord,(snapshot) => {
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
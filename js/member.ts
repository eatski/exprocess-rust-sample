import { collection,doc,setDoc,onSnapshot,getDocs } from "firebase/firestore";
import { getStore } from "./firestore";
import { getYourId, setYourId } from "./yourid";


export const registerMember = async (roomId:string,name:string):Promise<string> => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    const members = collection(room,"members");
    const member = doc(members);
    setYourId(roomId,member.id);
    await setDoc(member,{name})
    return member.id;
}

export const syncMember = (roomId:string,listener:(json:string) => void,onError: () => void) : () => void => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    const members = collection(room,"members");
    return onSnapshot(
        members,
        (snapshot) => {
            const yourId = getYourId(roomId);
            const json = JSON.stringify(
                snapshot.docs.map(doc => ({
                    ...doc.data(),
                    id:doc.id,
                    you:yourId === doc.id
                }))
            );
            listener(json);
        },
        onError
    )
}

export const fetchMembers = (roomId:string,callback:(json:string) => void, onError:() => void) => {
    const db = getStore();
    const rooms = collection(db,"rooms");
    const room = doc(rooms,roomId);
    const members = collection(room,"members");
    getDocs(members)
        .then(data => {
            const yourId = getYourId(roomId);
            const json = JSON.stringify(data.docs.map(doc => ({
                ...doc.data(),
                id:doc.id,
                you:yourId === doc.id
            })));
            callback(json);
        })
        .catch(onError)
}
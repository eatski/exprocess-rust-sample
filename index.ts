import { registerMember, syncMember } from "./js/memberRepository";

export function sync(roomid:string,fn: (json:string) => void) {
    syncMember(roomid,fn);
}

export function save(roomid:string,name:string) {
    registerMember(roomid,name);
}
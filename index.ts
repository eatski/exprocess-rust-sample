import { registerMember, syncMember } from "./js/memberRepository";
import { createRoom } from "./js/room";

export function sync(roomid:string,fn: (json:string) => void) {
    syncMember(roomid,fn);
}

export function save(roomid:string,name:string) {
    registerMember(roomid,name);
}

export function create(fn:(id:string) => void ) {
    createRoom().then(fn);
}
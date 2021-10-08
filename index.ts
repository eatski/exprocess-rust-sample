import * as member from "./js/memberRepository";
import * as room from "./js/room";

export function syncMember(roomid:string,fn: (json:string) => void) {
    member.syncMember(roomid,fn);
}

export function registerMember(roomid:string,name:string) {
    member.registerMember(roomid,name);
}

export function createRoom(callback:(id:string) => void) {
    room.createRoom().then(callback);
}

export function syncRoom(roomId:string,callback:(id:string | null) => void) {
    room.syncRoom(roomId,callback)
}
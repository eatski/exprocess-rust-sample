import * as member from "./js/memberRepository";
import * as room from "./js/room";

export function syncMember(roomid:string,fn: (json:string) => void) {
    member.syncMember(roomid,fn);
}

export function registerMember(roomid:string,name:string) {
    member.registerMember(roomid,name);
}

export async function createRoom(hostUserName:string,callback:(roomId:string) => void) {
    const roomId = room.publishRoomId();
    const hostId = await member.registerMember(roomId,hostUserName);
    await room.openRoom(roomId,hostId);
    callback(roomId);
}

export function syncRoom(roomId:string,callback:(id:room.Room | null) => void) {
    room.syncRoom(roomId,callback)
}

export function startRoom(roomId:string) {
    room.startRoom(roomId);
}
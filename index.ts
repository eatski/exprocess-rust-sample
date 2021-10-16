import * as member from "./js/member";
import * as room from "./js/room";
import * as record from "./js/record"; 

export function syncMember(roomid:string,fn: (json:string) => void) {
    member.syncMember(roomid,fn);
}

export function registerMember(roomid:string,name:string) {
    member.registerMember(roomid,name);
}

export function fetchMembers(roomid:string,fn: (json:string) => void) {
    member.fetchMembers(roomid,fn);
}

export async function createRoom(hostUserName:string,callback:(roomId:string) => void) {
    const roomId = room.publishRoomId();
    const hostId = await member.registerMember(roomId,hostUserName);
    await room.openRoom(roomId,hostId);
    callback(roomId);
}

export function syncRoom(roomId:string,callback:(id:string | null) => void) {
    room.syncRoom(roomId,callback)
}

export function startRoom(roomId:string) {
    room.startRoom(roomId);
}

export const pushRecord = (roomId:string,recordId:string,commandJson:string,resultJson:string) => {
    record.pushRecord(roomId,recordId,commandJson,resultJson);
}

export const syncRecordUpdate = (roomId:string,listener: (recordId:string,commandJson:string,resultJson:string) => void) => {
    record.syncRecordUpdate(roomId,listener);
}
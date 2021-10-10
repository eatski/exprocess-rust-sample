
const toYourIdKey = (roomId: string) => `${roomId}:yourid`;
export const getYourId = (roomId:string) => window.localStorage.getItem(toYourIdKey(roomId));

export const setYourId = (roomId:string,yourId: string) => window.localStorage.setItem(toYourIdKey(roomId),yourId);
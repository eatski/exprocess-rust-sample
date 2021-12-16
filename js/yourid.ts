
const toYourIdKey = (roomId: string) => `${roomId}:yourid`;

export const getYourId = (roomId:string) => {
    if(process.env.BUILD_MODE === "dev") {
        //@ts-expect-error
        return window.developmentYourId;
    } else {
        return window.localStorage.getItem(toYourIdKey(roomId)) 
    }
};

export const setYourId = (roomId:string,yourId: string) => {
    if(process.env.BUILD_MODE === "dev") {
        //@ts-expect-error
        window.developmentYourId = yourId;
    } else {
        window.localStorage.setItem(toYourIdKey(roomId),yourId)
    }
    
};
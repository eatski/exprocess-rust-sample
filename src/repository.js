export function getPayload() {
    return "This is JavaScript.";
}

export function getPayloadLater(fn) {
    setInterval(() => fn("hogeo"),1000);
}

export function sync(roomid,namespace,fn) {
    let cnt = 0;
    setInterval(() => {
        const json = JSON.stringify([{
            id:roomid + ":" + cnt.toString(),
            name: namespace + ":" + cnt.toString()
        }]);
        cnt++;
        fn(json);
    },1000);
}

export function save(roomid,namespace,name) {
    console.log(roomid,namespace,name);
}
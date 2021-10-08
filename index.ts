export function sync(roomid:string,namespace:string,fn: (json:string) => void) {
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

export function save(roomid:string,namespace:string,name:string) {
    console.log(roomid,namespace,name);
}
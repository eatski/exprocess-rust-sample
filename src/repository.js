export function getPayload() {
    return "This is JavaScript.";
}

export function getPayloadLater(fn) {
    setInterval(() => fn("hogeo"),1000);
}
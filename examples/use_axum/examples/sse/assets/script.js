var eventSource = new EventSource("sse");

evnetSource.onmessage = function (event) {
    console.log("Message from server ", event.data);
};

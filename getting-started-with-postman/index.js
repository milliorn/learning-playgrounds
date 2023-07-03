import { WebSocketServer } from "ws";
import si from "systeminformation";

// Set up a WebSockets server in Node.js
const webSocketServer = new WebSocketServer({ port: 8080 });
const milliseconds = 60000;

webSocketServer.on("connection", (websocket) => {
  websocket.on("message", (data) => {
    console.log("received: %s", data);
  });
  websocket.send(`Sending system information every ${milliseconds}ms.`);

  setInterval(async () => {
    const cpuTemp = JSON.stringify(await si.currentLoad());
    websocket.send(cpuTemp);
  }, 60000);
});
// end

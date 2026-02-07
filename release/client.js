function startAir(event) {
  let socket = new WebSocket("ws://localhost:10001/lv/air");
  socket.addEventListener("open", function (event) {
    console.log("Connected to air search");
    // Start sending ping messages every 25 seconds
    setInterval(() => {
      if (socket.readyState === WebSocket.OPEN) {
        socket.send(JSON.stringify({ ping: true }));
      }
    }, 25000);
  });

  // Handle connection close
  socket.addEventListener("close", function (event) {
    console.log("Disconnected from air search");
  });

  // Log errors
  socket.addEventListener("error", function (event) {
    console.error("WebSocket error: ", event);
  });

  return socket;
}

function listenToAir(event) {
  try {
    let data = JSON.parse(event.data);
    if (data.pong) {
      // PING RECEIVED A RECPONSE
      return false;
    } else if (
      data &&
      data.result &&
      data.result.length &&
      data.result.length > 0
    ) {
      return data;
    }
  } catch (err) {
    console.error(err);
    return false;
  }
}

// USE IT:
let socket = startAir();
// LISTEN TO MESSAGES
socket.addEventListener("message", function (event) {
  let result = listenToAir(event);
  if (result && result.result && result.result[0]) {
    console.log(result.result[0]);
  }
});
// SEND TEST MESSAGE
socket.addEventListener("open", function () {
  socket.send(JSON.stringify({ name: "Raimond Fantastic" }));
});

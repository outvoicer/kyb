function startAir(event) {
  let socket = new WebSocket("ws://localhost:10001/lv/air");
  socket.addEventListener("open", function (event) {
    console.log("Connected to air search");
    socket.send(JSON.stringify({ "name": "Raimond Fantastic" }));
  });
  // Handle connection close
  socket.addEventListener("close", function (event) {
    console.log("Disconnected from air search");
  });
  // Handle errors
  socket.addEventListener("error", function (event) {
    console.error("WebSocket error: ", event);
  });
  return socket;
}

function listenToAir(event) {
  try {
    let data = JSON.parse(event.data);
    if (data.result && data.result.length > 0) {
      return data;
    }
  } catch (err) {
    console.log(err);
    return false;
  }
}

let socket = startAir();
socket.addEventListener("message", function (event) {
  let result = listenToAir(event);
  console.log(result);
});

// socket.send({ "name": "Raimond Fantastic" });

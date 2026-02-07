let socket;

function startAir(event) {
  const maxRetries = 3;
  let retryCount = 0;

  function connect() {
    //socket = new WebSocket("ws://localhost:10001/lv/air");
    socket = new WebSocket("wss://kyb.outvoicer.com/lv/air");

    socket.addEventListener("open", function (event) {
      console.log("Connected to air search");
      retryCount = 0; // Reset retry count on successful connection

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
      attemptReconnect();
    });

    // Log errors
    socket.addEventListener("error", function (event) {
      console.error("WebSocket error: ", event);
      attemptReconnect();
    });
  }

  function attemptReconnect() {
    if (retryCount < maxRetries) {
      retryCount++;
      console.log(`Attempting to reconnect... (${retryCount}/${maxRetries})`);
      setTimeout(connect, 2000); // Wait 2 seconds before retrying
    } else {
      console.log("Max reconnection attempts reached. Giving up.");
    }
  }

  connect();
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

function searchAir(searchTerm) {
  socket.send(JSON.stringify({ name: searchTerm }));
}

/*
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
 */

/*

Frontend client to perform Latvian company search by name over socket.

+ Connect to kyb server socket,
+ send messages
+ listen to messages (search results + errors)
+ dispatch message to front end via document.airMessage & document.airError

// MOST SIMPLE USAGE:
// CREATE SOCKET
let socket = air.start();
// LISTEN TO document.airMessage FOR SEARCH RESULTS
document.addEventListener("airMessage", function (event) {
  console.log(event.detail.result[0]);
});
// WAIT FOR SOCKET OPEN
socket.addEventListener("open", function () {
  // AND SEND MESSAGE
  socket.send(JSON.stringify({ name: "Raimond Fantastic" }));
});
*/
/// Client to kyb server
const air = (function () {
  // OUR HERO OF THE DAY - SOCKET
  let socket;
  // SET UP RETRIES
  const maxRetries = 5;
  let retryCount = 0;
  // CONNECT TO SERVER
  function connect() {
    socket = new WebSocket("ws://localhost:10001/lv/air");
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
    // HANDLE CLOSE CONNECTION EVENT
    socket.addEventListener("close", function (event) {
      console.error("Air search: Disconnected from server");
      attemptReconnect();
    });
    // LOG ERRORS
    socket.addEventListener("error", function (event) {
      console.error("Air search error: ", event);
      attemptReconnect();
    });
    // HANDLE MESSAGES
    socket.addEventListener("message", function (event) {
      // LISTEN TO MESSAGE, PARSE AND DISPACTH TO INDEX.HTML
      try {
        // ATTEMPT TO PARSE
        let data = JSON.parse(event.data);

        if (data.error) {
          const messageEvent = new CustomEvent("airError", {
            detail: data.error,
          });
          // DSICPATCH TO FRONT END
          document.dispatchEvent(messageEvent);
        } else {
          const messageEvent = new CustomEvent("airMessage", {
            detail: data,
          });
          // DSICPATCH TO FRONT END
          document.dispatchEvent(messageEvent);
        }
      } catch (err) {
        // LOG ERRORS
        console.error(err);
      }
    });
    // RETURN SOCKET TO FRONT END
    return socket;
  }

  // ATTEMT RECONNECT
  function attemptReconnect() {
    if (retryCount < maxRetries) {
      retryCount++;
      console.log(
        `Air Attempting to reconnect... (${retryCount}/${maxRetries})`,
      );
      setTimeout(() => {
        socket = connect(); // Reconnect and update the socket
      }, 4000); // Wait 4 seconds before retrying
    } else {
      console.log("Air Max reconnection attempts reached. Giving up.");
    }
  }

  /// Initiate connection
  function start(event) {
    return connect();
  }

  // LISTEN TO SINGLE EVENT
  function listen(event) {
    try {
      let data = JSON.parse(event.data);
      if (data.pong) {
        // PING RECEIVED A RESPONSE
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
  /// Send search request to socket
  function search(searchTerm) {
    if (socket && socket.readyState === WebSocket.OPEN) {
      socket.send(JSON.stringify({ name: searchTerm }));
    } else {
      console.error("Air: Socket is not open. Cannot send search request.");
    }
  }

  return {
    start,
    listen,
    search,
  };
})();

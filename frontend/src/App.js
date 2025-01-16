import React, { useEffect, useState } from "react";
import axios from "axios";

function App() {
  const [status, setStatus] = useState("");

  useEffect(() => {
    axios.get("/api/health")
      .then((response) => setStatus(response.data.status))
      .catch((error) => console.error("Error fetching health status:", error));
  }, []);

  return (
    <div>
      <h1>Health Status</h1>
      <p>{status}</p>
    </div>
  );
}

export default App;

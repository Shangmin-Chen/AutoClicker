import React from "react";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import LandingPage from "./components/LandingPage";
import RoomPage from "./components/RoomPage";

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<LandingPage />} />
        <Route path="/rooms/:roomName" element={<RoomPage />} />
      </Routes>
    </Router>
  );
}

export default App;

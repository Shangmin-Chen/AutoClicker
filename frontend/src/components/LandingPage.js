import React, { useState } from "react";
import { useNavigate } from "react-router-dom";

function LandingPage() {
    const [roomName, setRoomName] = useState("");
    const navigate = useNavigate();

    const handleNavigate = () => {
        if (roomName.trim()) {
            navigate(`/rooms/${roomName}`);
        }
    };

    return (
        <div style={{ textAlign: "center", marginTop: "50px" }}>
            <h1>Welcome to RustPaint</h1>
            <input
                type="text"
                placeholder="Enter room name"
                value={roomName}
                onChange={(e) => setRoomName(e.target.value)}
            />
            <button onClick={handleNavigate}>Go</button>
        </div>
    );
}

export default LandingPage;

import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

function RoomPage() {
    const { roomName } = useParams();
    const [roomDetails, setRoomDetails] = useState(null);
    const [error, setError] = useState("");

    useEffect(() => {
        fetch(`http://127.0.0.1:8080/rooms/${roomName}`)
            .then((response) => {
                if (!response.ok) {
                    throw new Error("Room not found.");
                }
                return response.json();
            })
            .then((data) => setRoomDetails(data))
            .catch((err) => setError(err.message));
    }, [roomName]);

    if (error) {
        return <h2>{error}</h2>;
    }

    return (
        <div style={{ textAlign: "center", marginTop: "50px" }}>
            {roomDetails ? (
                <>
                    <h1>Room: {roomDetails.name}</h1>
                    <p>Room ID: {roomDetails.id}</p>
                    <p>Created At: {roomDetails.created_at}</p>
                </>
            ) : (
                <h2>Loading...</h2>
            )}
        </div>
    );
}

export default RoomPage;

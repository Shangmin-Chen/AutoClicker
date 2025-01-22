# RustPaint

## Overview
This project is a **collaborative pixel art application** that will run for **exactly one year**. Inspired to be later developed into a **NFT** project. Each day, users can collaborate to create a pixel art canvas based on a **daily prompt**. The application supports real-time collaboration, and at the end of each day, the completed canvas is converted into a **high-definition image** using AI. All daily canvases are publicly viewable, with archived snapshots available for past days.

---

## Features
1. **Daily Canvas**:
   - A new **500x500 pixel canvas** is created every day.
   - Users can collaboratively color pixels in real-time.
   - Daily prompts are pre-set and stored in a file (e.g., `prompts.json`).

2. **Real-Time Collaboration**:
   - Pixel updates are synchronized instantly across all users via **WebSockets**.
   - A **cooldown** mechanism limits how often users can update pixels.

3. **AI Integration**:
   - At the end of each day, the user-created canvas and the daily prompt are sent to an AI to generate a **high-definition image**.

4. **Public Accessibility**:
   - All canvases, prompts, and AI-generated images are publicly accessible.
   - Past canvases remain viewable as snapshots and AI-enhanced images.

5. **Landing Page**:
   - Displays:
     - **Current day's canvas** as a button for users to participate.
     - **Archived canvases** with clickable previews of snapshots and AI-generated images.

---

## Technical Stack
- **Frontend**:
  - **React**: For building the user interface.
  - **Konva.js**: For rendering the pixel canvas with zoom and pan functionality.
  - **WebSockets**: For real-time pixel updates.

- **Backend**:
  - **Rust**: High-performance backend using `actix-web` or `warp`.
  - **PostgreSQL**: Database for storing room metadata and pixel data.
  - **ChatGPT API**: For pre-set prompts and AI-powered image generation.

- **Storage**:
  - Snapshots and AI-generated images stored in **AWS S3** or equivalent cloud storage.

---

## Workflow
1. **Daily Workflow**:
   - At midnight:
     - A new canvas is initialized.
     - The daily prompt is loaded from `prompts.json`.
   - Throughout the day:
     - Users collaboratively fill in the canvas.
     - Pixel updates are synced in real-time.
   - At the end of the day:
     - The canvas becomes immutable.
     - A snapshot is taken and saved.
     - The canvas and prompt are sent to AI to generate a high-definition image.

2. **One-Year Timeline**:
   - The application runs for **365 days**, with one canvas and prompt per day.
   - After one year, all canvases and AI images remain publicly viewable as a permanent archive.

---

## Database Schema
1. **Tables**:
   - **Daily Canvas**:
     - `date`: Unique key for each day.
     - `prompt`: The daily prompt.
     - `snapshot_path`: Path to the saved user-created canvas snapshot.
     - `ai_image_path`: Path to the AI-generated high-definition image.
   - **Pixels**:
     - `date`: Foreign key linking to `Daily Canvas`.
     - `x`: X-coordinate of the pixel.
     - `y`: Y-coordinate of the pixel.
     - `color`: Color of the pixel in HEX or RGB format.

---

## Project Roadmap

### **Phase 1: Setup**
- [ ] Initialize **React** frontend.
- [ ] Initialize **Rust** backend with a basic API.
- [ ] Set up **PostgreSQL** database and define schema.

### **Phase 2: Core Features**
- [ ] Create landing page to display current and past canvases.
- [ ] Implement **500x500 pixel canvas** with zoom and pan.
- [ ] Add **color picker** and implement real-time pixel updates with WebSockets.
- [ ] Set up cooldown logic for pixel updates.

### **Phase 3: Daily Workflow**
- [ ] Automate daily canvas initialization with prompts from `prompts.json`.
- [ ] Schedule daily snapshot generation and canvas locking.
- [ ] Integrate AI for generating high-definition images from canvases and prompts.

### **Phase 4: Deployment**
- [ ] Dockerize the application (frontend, backend, database).
- [ ] Deploy to a cloud hosting platform (e.g., AWS, DigitalOcean).
- [ ] Configure cloud storage for snapshots and AI images.

### **Phase 5: Post-Deployment**
- [ ] Set up monitoring and logging for backend performance.
- [ ] Optimize WebSocket performance and database queries.
- [ ] Launch and collect user feedback for improvements.

---

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/Shangmin-Chen/RustPaint.git
   cd RustPaint
   ```

2. Set up the backend:
   ```bash
   cd backend
   cargo run
   ```

3. Set up the frontend:
   ```bash
   cd frontend
   npm install
   npm start
   ```

4. Start PostgreSQL and initialize the database:
   ```bash
   createdb RustPaint_db
   ```

---

## Contributing
We welcome contributions! Feel free to submit issues, feature requests, or pull requests.

---

## License
This project is licensed under the MIT License.

---

Let me know if you'd like me to customize further or add more details!
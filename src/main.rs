// Import necessary crates for GUI and threading
use eframe::egui; // For GUI elements
use std::sync::{Arc, Mutex}; // For safely sharing data across threads
use std::thread; // For running the clicking process in a separate thread
use std::time::{Duration, Instant}; // For timing the clicks

// Enum for selecting the clicking method (Enigo or WinAPI)
#[derive(Clone, PartialEq)]
enum ClickMethod {
    Enigo,
    WinAPI,
}

// State to manage the GUI and clicking process
#[derive(Clone)]
struct AppState {
    clicks_per_second: Arc<Mutex<u32>>, // Shared state for clicks per second
    click_method: Arc<Mutex<ClickMethod>>, // Shared state for the selected clicking method
    is_running: Arc<Mutex<bool>>, // Shared state for whether the auto-clicker is running
}

fn main() {
    // Options for the GUI window
    let options = eframe::NativeOptions::default();
    // Run the GUI application
    eframe::run_native(
        "Auto-Clicker", // Title of the window
        options, // GUI options
        Box::new(|_cc| Box::new(MyApp::default())), // Create a new instance of MyApp
    );
}

// GUI application structure
struct MyApp {
    state: AppState, // The application state
}

impl Default for MyApp {
    fn default() -> Self {
        // Initialize default state for the application
        Self {
            state: AppState {
                clicks_per_second: Arc::new(Mutex::new(10)), // Default to 10 clicks per second
                click_method: Arc::new(Mutex::new(ClickMethod::Enigo)), // Default to Enigo
                is_running: Arc::new(Mutex::new(false)), // Default to not running
            },
        }
    }
}

// Implement the GUI logic
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // The main GUI panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Auto-Clicker"); // GUI title

            // Input for setting clicks per second
            ui.horizontal(|ui| {
                ui.label("Clicks per second:");
                let mut cps = *self.state.clicks_per_second.lock().unwrap();
                if ui.add(egui::DragValue::new(&mut cps).clamp_range(1..=10000)).changed() {
                    *self.state.clicks_per_second.lock().unwrap() = cps;
                }
            });

            // Radio buttons for selecting the clicking method
            ui.horizontal(|ui| {
                ui.label("Click Method:");
                if ui
                    .radio_value(
                        &mut *self.state.click_method.lock().unwrap(),
                        ClickMethod::Enigo,
                        "Enigo",
                    )
                    .clicked()
                {
                    *self.state.click_method.lock().unwrap() = ClickMethod::Enigo;
                }
                if ui
                    .radio_value(
                        &mut *self.state.click_method.lock().unwrap(),
                        ClickMethod::WinAPI,
                        "WinAPI",
                    )
                    .clicked()
                {
                    *self.state.click_method.lock().unwrap() = ClickMethod::WinAPI;
                }
            });

            // Start button to begin clicking
            if ui.button("Start").clicked() {
                let state = self.state.clone(); // Clone the state for the thread
                thread::spawn(move || start_clicking(state)); // Run the clicking process in a new thread
            }

            // Stop button to stop clicking
            if ui.button("Stop").clicked() {
                *self.state.is_running.lock().unwrap() = false; // Set running state to false
            }
        });

        ctx.request_repaint(); // Ensure the GUI keeps updating
    }
}

// Function to handle the clicking process
fn start_clicking(state: AppState) {
    *state.is_running.lock().unwrap() = true; // Set running state to true

    let cps = *state.clicks_per_second.lock().unwrap(); // Get clicks per second
    let delay = Duration::from_micros((1_000_000 / cps).into()); // Calculate delay between clicks
    let click_method = state.click_method.lock().unwrap().clone(); // Get the selected clicking method

    match click_method {
        // Clicking using Enigo
        ClickMethod::Enigo => {
            let mut enigo = enigo::Enigo::new();
            while *state.is_running.lock().unwrap() {
                enigo.mouse_click(enigo::MouseButton::Left); // Perform a left mouse click
                thread::sleep(delay); // Wait before the next click
            }
        }
        // Clicking using WinAPI
        ClickMethod::WinAPI => unsafe {
            use winapi::um::winuser::{mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP};
            while *state.is_running.lock().unwrap() {
                mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0); // Press mouse button
                mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0); // Release mouse button
                thread::sleep(delay); // Wait before the next click
            }
        },
    }
}

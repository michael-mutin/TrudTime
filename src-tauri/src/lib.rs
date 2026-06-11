use std::{fs::File};
use std::time::Duration;
use std::sync::Mutex;
use tauri::Emitter;
use tauri::{AppHandle, Manager, path::BaseDirectory, State};
use rodio::{Decoder, DeviceSinkBuilder, Player};

struct TimerAppState {
    timer: Option<Timer>,
}

enum TimerState {
    NotStarted,
    Running(tokio::task::JoinHandle<()>, tokio::time::Instant),
    Finished,
}

struct Timer {
    state: TimerState,
    duration: Duration,
}

impl Timer {
    fn new(duration: Duration) -> Self {
        Self {
            state: TimerState::NotStarted,
            duration: duration
        }
    }

    fn start(&mut self, app_handle: AppHandle) {
        let duration = self.duration.clone();
        let handle = tokio::spawn(async move {
            tokio::time::sleep(duration).await;
            let state = app_handle.state::<Mutex<TimerAppState>>();
            let mut state = state.lock().unwrap();
            let opt_timer = &mut state.timer;
            match opt_timer.as_mut() {
                Some(timer) => {
                    timer.change_state(TimerState::Finished);
                    app_handle.emit("over", ()).unwrap();
                }
                None => { unreachable!() }
            }
        });
        self.change_state(TimerState::Running(handle, tokio::time::Instant::now()));
    }

    fn change_state(&mut self, new_state: TimerState) {
        self.try_abort_handle();
        self.state = new_state;
    }

    fn try_abort_handle(&mut self) {
        match &self.state {
            TimerState::Running(handle, _) => handle.abort(),
            _ => ()
        };
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.try_abort_handle();
    }
}

#[tauri::command]
async fn start_timer(app_handle: AppHandle, state: State<'_, Mutex<TimerAppState>>, duration_in_secs: u64) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    let opt_timer = &mut state.timer;

    let a_timer_is_already_running = match opt_timer.as_mut() {
        Some(timer) => {
            match timer.state {
                TimerState::Running(..) => true,
                _ => false
            }
        },
        None => false,
    };

    if !a_timer_is_already_running {
        let mut timer = Timer::new(Duration::from_secs(duration_in_secs));
        timer.start(app_handle);
        *opt_timer = Some(timer);
        Ok(())
    } else {
        Err(String::from("A timer is already running"))
    }
}

#[tauri::command]
fn get_time_left(state: State<'_, Mutex<TimerAppState>>) -> Result<u64, String> {
    let mut state = state.lock().unwrap();
    let opt_timer = &mut state.timer;

    match opt_timer.as_mut() {
        Some(timer) => {
            match timer.state {
                TimerState::NotStarted => Ok(timer.duration.as_secs()),
                TimerState::Running(_, start_time) => Ok(match timer.duration.checked_sub(tokio::time::Instant::now().duration_since(start_time)) {
                    Some(result) => result.as_secs(),
                    None => 0
                }),
                TimerState::Finished => Ok(0)
            }
        },
        None => Err(String::from("No timer running")),
    }
}

#[tauri::command]
fn stop_timer(state: State<'_, Mutex<TimerAppState>>) -> Result<u64, String> {
    let mut state = state.lock().unwrap();
    let opt_timer = &mut state.timer;

    match opt_timer.as_mut() {
        Some(timer) => {
            match timer.state {
                TimerState::NotStarted => unreachable!(),
                TimerState::Running(_, start_time) => {
                    let time_left = match timer.duration.checked_sub(tokio::time::Instant::now().duration_since(start_time)) {
                        Some(result) => result.as_secs(),
                        None => 0
                    };
                    *opt_timer = None;
                    Ok(time_left)
                },
                TimerState::Finished => Err(String::from("There is no timer running."))
            }
        },
        None => Err(String::from("There is no timer running."))
    }
}

#[tauri::command]
async fn play_sound(app_handle: AppHandle) -> Result<(), String> {
    let resource_path = app_handle
        .path()
        .resolve("assets/universfield-new-notification-060-494264.mp3", BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

    let system_sink = DeviceSinkBuilder::open_default_sink()
        .map_err(|e| format!("No Audio device found!: {}", e))?;

    let file = File::open(resource_path).map_err(|e| e.to_string())?;
    let source = Decoder::try_from(file).map_err(|e| e.to_string())?;

    let player = Player::connect_new(system_sink.mixer());
    player.append(source);
    
    player.sleep_until_end();

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(TimerAppState {
                timer: None,
            }));
            Ok(())
        })
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_timer, get_time_left, stop_timer, play_sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

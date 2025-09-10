use crate::config::app_config::AppConfig;
use crate::enums::tray_menu_event_enum::TrayMenuEventEnum;
use crate::enums::ui_command_enum::UICommandEnum;
use crate::error::app_error::Result;
use crate::i18n::i18n_manager::I18nManager;
use crate::monitor::monitor::{MonitorManager, SystemMonitor};
use crate::tray::tray::{SystemTray, Tray};
use crate::ui::ui;
use auto_launch::AutoLaunch;
use eframe::egui;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use tray_icon::menu::{MenuEvent, MenuId};

pub struct App {
    app_config: Arc<Mutex<AppConfig>>,
    i18n: Arc<Mutex<I18nManager>>,
    monitor_manager: Box<dyn MonitorManager>,
    tray_manager: Option<Box<dyn Tray>>,
    last_update: Instant,
    ui_command_rx: Option<mpsc::Receiver<UICommandEnum>>,
    tray_thread_handle: Option<JoinHandle<()>>,
    tray_shutdown_tx: Option<mpsc::Sender<()>>,
    is_shutting_down: bool,
    auto_launch: AutoLaunch,
}

impl App {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        app_config: AppConfig,
        i18n: Arc<Mutex<I18nManager>>,
        auto_launch: AutoLaunch,
    ) -> Result<Self> {
        if app_config.general.run_on_startup {
            auto_launch.enable().ok();
        } else {
            auto_launch.disable().ok();
        }

        if app_config.general.minimized_window_on_startup {
            cc.egui_ctx
                .send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        }

        let (_tx, rx) = mpsc::channel();
        Ok(Self {
            app_config: Arc::new(Mutex::new(app_config)),
            i18n,
            monitor_manager: Box::new(SystemMonitor::new()),
            tray_manager: None,
            last_update: Instant::now(),
            ui_command_rx: Some(rx),
            tray_thread_handle: None,
            tray_shutdown_tx: None,
            is_shutting_down: false,
            auto_launch,
        })
    }

    fn spawn_tray_handler_thread(
        &mut self,
        ctx: &egui::Context,
        menu_rx: crossbeam_channel::Receiver<MenuEvent>,
        id_map: HashMap<MenuId, TrayMenuEventEnum>,
    ) -> mpsc::Receiver<UICommandEnum> {
        let (ui_tx, ui_rx) = mpsc::channel();
        let (shutdown_tx, shutdown_rx) = mpsc::channel();
        let ctx_clone = ctx.clone();
        let app_config_clone = self.app_config.clone();

        let handle = thread::Builder::new()
            .name("tray-handler".to_string())
            .spawn(move || {
                Self::tray_event_loop(
                    menu_rx,
                    id_map,
                    ui_tx,
                    ctx_clone,
                    shutdown_rx,
                    app_config_clone,
                );
            })
            .expect("Failed to spawn tray handler thread");

        self.tray_thread_handle = Some(handle);
        self.tray_shutdown_tx = Some(shutdown_tx);
        ui_rx
    }

    fn shutdown_tray_handler_thread(&mut self) {
        if let Some(shutdown_tx) = self.tray_shutdown_tx.take() {
            let _ = shutdown_tx.send(());
        }

        if let Some(handle) = self.tray_thread_handle.take() {
            if let Err(e) = handle.join() {
                eprintln!("Tray thread panicked during shutdown: {:?}", e);
            }
        }

        self.ui_command_rx.take();
    }

    fn tray_event_loop(
        menu_rx: crossbeam_channel::Receiver<MenuEvent>,
        id_map: HashMap<MenuId, TrayMenuEventEnum>,
        ui_tx: mpsc::Sender<UICommandEnum>,
        ctx: egui::Context,
        shutdown_rx: mpsc::Receiver<()>,
        app_config: Arc<Mutex<AppConfig>>,
    ) {
        loop {
            if shutdown_rx.try_recv().is_ok() {
                break;
            }

            if let Ok(event) = menu_rx.try_recv() {
                if let Some(action) = id_map.get(&event.id) {
                    let command = match action {
                        TrayMenuEventEnum::Settings => UICommandEnum::ShowSettings,
                        TrayMenuEventEnum::Quit => UICommandEnum::Quit,
                    };

                    if ui_tx.send(command).is_ok() {
                        ctx.request_repaint();
                    }

                    if matches!(action, TrayMenuEventEnum::Quit) {
                        break;
                    }
                }
            } else {
                let delay = app_config.lock().unwrap().timing.tray_error_retry_delay_ms;
                thread::sleep(Duration::from_millis(delay.min(50)));
            }
        }
    }

    fn process_ui_commands(&mut self, ctx: &egui::Context) -> bool {
        let mut wants_to_quit = false;

        let commands: Vec<UICommandEnum> = if let Some(rx) = &self.ui_command_rx {
            rx.try_iter().collect()
        } else {
            Vec::new()
        };

        for command in commands {
            match command {
                UICommandEnum::ShowSettings => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                }
                UICommandEnum::Quit => {
                    self.initiate_shutdown();
                    wants_to_quit = true;
                }
            }
        }

        wants_to_quit
    }

    fn update_tray_icons(&mut self, app_config: &AppConfig) {
        if let Some(tray_manager) = &self.tray_manager {
            let stats = self.monitor_manager.update_all(app_config);
            let i18n_guard = self.i18n.lock().unwrap();
            if let Err(e) = tray_manager.update(&app_config.active_monitors, &i18n_guard, &stats) {
                eprintln!("Error updating tray icon: {}", e);
            }
        }
    }

    fn initiate_shutdown(&mut self) {
        if self.is_shutting_down {
            return;
        }
        self.is_shutting_down = true;

        if let Some(shutdown_tx) = &self.tray_shutdown_tx {
            let _ = shutdown_tx.send(());
        }
    }

    fn cleanup_resources(&mut self) {
        if let Err(e) = self.app_config.lock().unwrap().save() {
            eprintln!("Error saving app config on exit: {}", e);
        }

        if !self.is_shutting_down {
            self.initiate_shutdown();
        }

        if let Some(handle) = self.tray_thread_handle.take() {
            if let Err(e) = handle.join() {
                eprintln!("Tray thread panicked during shutdown: {:?}", e);
            }
        }

        self.tray_shutdown_tx.take();
        self.ui_command_rx.take();
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.tray_manager.is_none() && !self.is_shutting_down {
            let tray_result = {
                let i18n_guard = self.i18n.lock().unwrap();
                SystemTray::new(&i18n_guard)
            };

            match tray_result {
                Ok((tray_manager, menu_rx, id_map)) => {
                    self.tray_manager = Some(Box::new(tray_manager));
                    let rx = self.spawn_tray_handler_thread(ctx, menu_rx, id_map);
                    self.ui_command_rx = Some(rx);
                }
                Err(e) => {
                    eprintln!("Failed to initialize tray manager: {}", e);
                    self.initiate_shutdown();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    return;
                }
            }
        }

        if self.is_shutting_down {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        if self.process_ui_commands(ctx) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        let app_config_snapshot = { self.app_config.lock().unwrap().clone() };

        if self.last_update.elapsed()
            >= Duration::from_secs(app_config_snapshot.refresh.default_refresh_seconds)
        {
            self.update_tray_icons(&app_config_snapshot);
            self.last_update = Instant::now();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let (shutdown_requested, language_changed, autostart_changed) =
                ui::draw_ui(ui, self.app_config.clone(), self.i18n.clone());

            if language_changed {
                self.shutdown_tray_handler_thread();
                self.tray_manager = None;
                ctx.request_repaint();
            }

            if autostart_changed {
                let app_config = self.app_config.lock().unwrap();
                if app_config.general.run_on_startup {
                    if let Err(e) = self.auto_launch.enable() {
                        eprintln!("Failed to enable autostart: {}", e);
                    }
                } else {
                    if let Err(e) = self.auto_launch.disable() {
                        eprintln!("Failed to disable autostart: {}", e);
                    }
                }
            }

            if shutdown_requested {
                self.initiate_shutdown();
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });

        ctx.request_repaint_after(Duration::from_millis(
            app_config_snapshot.timing.ui_repaint_interval.min(100),
        ));
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.cleanup_resources();
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.cleanup_resources();
    }
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::collections::HashMap;
use std::sync::Arc;
use turbosql::{execute, select, Turbosql};
use crate::auth::get_auth;
//use crate::loader::{get_client_version, get_lockfile, get_player_info, get_region_shard};
use crate::loader::Loader;

use std::time;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use eframe::egui;
use eframe::egui::{Color32, Layout, Pos2, Rounding, Vec2};
use poll_promise::Promise;
use crate::database::{MatchHistory, NameHistory};
use crate::images::ImageData;
use crate::r#match::MatchHandler;

mod loader;
mod auth;
mod pre_game;
mod r#match;
mod name_service;
mod database;
mod images;

pub enum ApplicationError {
    RetryError(String),
    RestartError(String),
}

#[derive(Debug, Clone)]
struct LoadedPlayer {
    uuid: String,
    name: String,
    tag: String,
    team: TeamType,

    match_history: Vec<MatchHistory>,
    name_history: Vec<NameHistory>,

    times_played: i64,
    last_played: i64,

    agent_id: String,
    incognito: bool,
}
//
#[derive(Debug, Clone)]
enum TeamType {
    Ally,
    Enemy
}
//
#[derive(Turbosql, Default, Debug, Clone)]
struct UserDatabase {
    rowid: Option<i64>,
    uuid: Option<String>,
    times_played: Option<i64>,
    last_played: Option<i64>,
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([350.0, 350.0])
            .with_resizable(false)
            .with_maximize_button(false),
        ..Default::default()
    };
    eframe::run_native(
        "Val+",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )
}

#[derive(Default)]
struct MyApp {
    auth: Option<Arc<RiotAuth>>,
    state: State,

    current_match: Option<MatchHandler>,

    settings: Settings,

    selected_user: Option<u8>,

    promise: Option<Promise<Option<MatchHandler>>>,
    image_promise: Option<Promise<Option<ImageData>>>,

    images: Option<ImageData>,
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Load,
    Refresh,
    ButtonRefresh,
    WaitValorant,
    CheckPromise,
    WaitMatch,
    Settings,
}

impl Default for State {
    fn default() -> Self {
        Self::WaitValorant
    }
}

#[derive(Debug, Clone)]
struct RiotAuth {
    access_token: String,
    client_ver: String,
    puuid: String,
    port: String,
    password: String,
    region: String,
    shard: String,
    token: String,
}

#[derive(Debug, Clone)]
pub struct Settings {
    auto_refresh: bool,
    wait_time: u64,
    refresh_time: u64,
    last_checked: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            auto_refresh: true,
            wait_time: 15,
            refresh_time: 10,
            last_checked: 0,
        }
    }
}

impl Settings {

    pub fn time_now(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }

    // Checks whether refresh time passed
    pub fn can_refresh(&mut self) -> bool {
        let time_now = self.time_now();

        if (time_now - self.last_checked) > self.refresh_time {
            self.last_checked = time_now;
            return true;
        }

        return false
    }

    pub fn can_wait(&mut self) -> bool {
        let time_now = self.time_now();

        if (time_now - self.last_checked) > self.wait_time {
            self.last_checked = time_now;
            return true;
        }

        return false
    }

    pub fn get_refresh_time(&mut self) -> u64 {
        let time_now = self.time_now();

        // .max(0) makes cast into u64 turn into 0 if i64 is negative
        (self.refresh_time as i64 - (time_now - self.last_checked) as i64).max(0) as u64
    }
}

impl RiotAuth {
    fn load() -> Option<Self> {
        let loader = Loader::new();

        if let Some((port, password)) = loader.get_port_and_password() {
            if let Some((token, access_token)) = get_auth(port.clone(), password.clone()) {
                println!("here2");
                if let Some((region, shard)) = loader.get_region_and_shard() {
                    println!("here3");
                    return Some(Self {
                        access_token: access_token.clone(),
                        client_ver: loader.get_client_version(port.clone(), password.clone()).unwrap(),
                        puuid: loader.get_player_info(access_token.clone()).unwrap(),
                        port,
                        password,
                        region,
                        shard,
                        token: token.clone(),
                    })
                }
            }
        }

        return None
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            // ---- TOP MENU -----

            ui.horizontal(|ui| {
                if ui.button("Home").clicked() {
                    self.state = State::Refresh;
                };

                if self.auth.is_none() {
                    ui.add_enabled(false, egui::Button::new("Refresh"));
                } else {
                    if ui.button(format!("Refresh (Auto: {})", if self.settings.auto_refresh { self.settings.get_refresh_time() } else { 999 })).clicked {
                        self.settings.last_checked = 0;
                        self.state = State::ButtonRefresh;
                    };
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Max),|ui| {
                    if ui.button("⚙").clicked() {
                        self.state = State::Settings;
                    };
                });

            });

            // Padding
            ui.vertical(|ui| ui.add(egui::widgets::Separator::default().spacing(10.0)));

            // ---- MAIN BODY ----

            match &self.state {

                State::Load => {
                    self.image_promise = Some(Promise::spawn_thread("load_images", || {
                        let mut image_data = ImageData::new();

                        if let Ok(_) = image_data.get_agents() {
                            if let Ok(_) = image_data.get_maps() {
                                return Some(image_data)
                            }
                        }

                        return None
                    }));

                    self.state = State::CheckPromise;
                }

                State::Settings => {
                    ui.horizontal(|ui| {
                        ui.label("Auto Refresh: ");
                        ui.checkbox(&mut self.settings.auto_refresh, "");
                    });
                }

                State::WaitValorant => {
                    if self.settings.can_wait() {
                        println!("passed");

                        // check for lockfile

                        if let Some(auth) = RiotAuth::load() {
                            self.auth = Some(Arc::new(auth));
                            self.state = State::Load;
                        }
                    } else {
                        // do something while not?
                    }
                }

                State::Refresh | State::ButtonRefresh => {
                    if (self.settings.can_refresh() && self.settings.auto_refresh) || self.state == State::ButtonRefresh {
                        self.state = State::Refresh; // In case state was on button refresh


                        match &self.promise {
                            Some(_) => {
                                self.state = State::CheckPromise;
                            }
                            _ => {
                                if let Some(auth) = &self.auth {
                                    // Cloned data to pass into promise
                                    let new_auth = Arc::clone(auth);

                                    self.promise = Some(Promise::spawn_thread("look_for_match", move || {
                                       // TODO: Implement pre-game
                                        let mut match_handler = MatchHandler::new();

                                        if let Ok(_) = match_handler.get_match_id(Arc::clone(&new_auth)) {
                                            if let Ok(_) = match_handler.get_match_details(Arc::clone(&new_auth)) {
                                                return Some(match_handler)
                                            }
                                        }


                                        None

                                       // match r#match::CurrentGamePlayer::get_players(new_auth, match_id) {
                                       //     Ok((loaded_players, match_id)) => {
                                       //         // (Players, MatchId)
                                       //         Some((loaded_players, match_id))
                                       //     }
                                       //     _ => {
                                       //         None
                                       //     }
                                       // }
                                    }));

                                    self.state = State::CheckPromise;
                                }
                            }
                        }
                    }
                }

                State::CheckPromise => {
                    if let Some(promise) = &self.promise {
                        if let Some(promise) = promise.ready() {
                            match promise {
                                Some(match_handler) => {
                                    println!("promise returned Some");
                                    self.current_match = Some(match_handler.clone());
                                    self.promise = None;
                                }
                                None => {
                                    println!("promise returned None");
                                    // returns none (so not exist?)
                                    //self.loaded_players = None;
                                    self.promise = None;
                                }
                            }

                            self.state = State::Refresh;
                        }
                    }

                    if let Some(promise) = &self.image_promise {
                        if let Some(promise) = promise.ready() {
                            match promise {
                                Some(image_data) => {
                                   self.images = Some(image_data.to_owned());
                                }
                                None => {
                                    println!("promise returned None");
                                    // returns none (so not exist?)
                                    //self.loaded_players = None;
                                    self.promise = None;
                                }
                            }

                        } else {
                            self.state = State::CheckPromise;
                            ui.label("Loading images...");
                        }
                    }
                }

                _ => {

                }
            }

            if let Some(current_match) = &self.current_match {
                let players = &current_match.players;

                let f = timeago::Formatter::new();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, player) in players.iter().enumerate() {
                        if player.times_played > 0 {

                            ui.horizontal(|ui| {

                                if let Some(image_data) = &self.images {
                                    ui.add(
                                        egui::Image::new(image_data.agents.iter().find(|x| x.uuid == player.agent_id).unwrap().icon.clone())
                                            .fit_to_exact_size(Vec2::new(80.0, 80.0))
                                            .maintain_aspect_ratio(false)
                                            .rounding(10.0)
                                    );
                                }

                                ui.colored_label(Color32::WHITE,  format!("{}#{} ({})", if !player.incognito { player.name.clone() } else { "N/A".to_string() }, if !player.incognito { player.tag.clone() } else { "N/A".to_string() }, f.convert(time::Duration::from_secs((self.settings.time_now() as i64 - player.last_played).max(0) as u64))));

                                if player.match_history.len() > 0usize {
                                    let button = ui.button("Show More");

                                    if button.clicked() {
                                        if let Some(index) = self.selected_user {
                                            if i == index.to_owned() as usize {
                                                // USER CLICKED ALREADY SELECTED
                                                self.selected_user = None;
                                            }  else {
                                                self.selected_user = Some(i as u8);
                                            }
                                        } else {
                                            self.selected_user = Some(i as u8);
                                        }
                                    }
                                }
                            });

                            if let Some(index) = self.selected_user {
                                if i == index.to_owned() as usize {

                                    if &player.name_history.len() > &0usize {
                                        ui.vertical(|ui| ui.add(egui::widgets::Separator::default().spacing(10.0)));

                                        ui.label("Previous Usernames: ");

                                        for name in &player.name_history {
                                            ui.label(format!("{}#{} ({})", name.name.clone().unwrap(), name.tag.clone().unwrap(), f.convert(time::Duration::from_secs((self.settings.time_now() as i64 - name.name_time.clone().unwrap()).max(0) as u64))));
                                        }
                                    }

                                    if &player.match_history.len() > &0usize {

                                        ui.vertical(|ui| ui.add(egui::widgets::Separator::default().spacing(10.0)));

                                        for log in player.match_history.iter().rev().take(5) {

                                            let mut map_icon_url = "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fcdn.thespike.gg%2FEmmanuel%2Fhaven4_1666929773076.jpg&f=1&nofb=1&ipt=ca70048ad86df92c1a1482f4c1d0f55ef9c4ba898331097417ac015ddba5a086&ipo=images".to_string();
                                            let mut map_name = "Unknown".to_string();
                                            let mut enemy = false;
                                            let mut agent_icon_url = "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fcdn.thespike.gg%2FEmmanuel%2Fhaven4_1666929773076.jpg&f=1&nofb=1&ipt=ca70048ad86df92c1a1482f4c1d0f55ef9c4ba898331097417ac015ddba5a086&ipo=images".to_string();

                                            if let Some(image_data) = &self.images {
                                                let map = image_data.maps.iter().find(|x| x.path.trim().to_lowercase() == log.map_id.clone().unwrap().trim().to_lowercase()).unwrap();

                                                agent_icon_url = image_data.agents.iter().find(|x| x.uuid == log.agent_id.clone().unwrap()).unwrap().icon.clone();

                                                map_icon_url = map.icon.clone();
                                                map_name = map.name.clone();
                                                if log.enemy.clone().unwrap() {
                                                    enemy = true;
                                                }
                                            }

                                            egui::Frame::none()
                                                .fill(Color32::from_rgb(31, 31, 31))
                                                .rounding(10.0)
                                                .show(ui, |ui| {
                                                    ui.set_width(ui.available_width());
                                                    ui.set_height(80.0);
                                                    ui.set_max_height(80.0);

                                                    ui.horizontal(|ui| {
                                                        ui.add(
                                                            egui::Image::new(agent_icon_url)
                                                                .fit_to_exact_size(Vec2::new(80.0, 80.0))
                                                                .maintain_aspect_ratio(false)
                                                                .rounding(10.0)
                                                        );

                                                        ui.vertical(|ui| {

                                                            ui.add_space(15.0);

                                                            ui.colored_label(Color32::WHITE, map_name);

                                                            if enemy {
                                                                ui.colored_label(Color32::RED, "Enemy");
                                                            } else {
                                                                ui.colored_label(Color32::GREEN, "Team");
                                                            }


                                                            ui.colored_label(Color32::WHITE, format!("{}", f.convert(time::Duration::from_secs((self.settings.time_now() as i64 - log.match_time.unwrap()).max(0) as u64))));
                                                        });

                                                        ui.add_space(ui.available_width() - 80.0);

                                                        ui.add(
                                                            egui::Image::new(map_icon_url)
                                                                .fit_to_exact_size(Vec2::new(80.0, 80.0))
                                                                .maintain_aspect_ratio(false)
                                                                .rounding(10.0)
                                                        );
                                                    });
                                                });
                                        }
                                    } else {
                                        ui.label("No history");
                                    }
                                }
                            }

                            ui.vertical(|ui| ui.add(egui::widgets::Separator::default().spacing(10.0)));
                        }
                    }

                    ui.label("Made by: nedasv | Discord: 3eu");
                });
            }
        });
    }
}

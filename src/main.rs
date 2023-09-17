#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use cli_clipboard::{ClipboardContext, ClipboardProvider};
use eframe::egui;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod ini;
mod numbers;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320., 240.)),
        resizable: false,
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "This Number Does Not Exist",
        options,
        Box::new(|_cc| Box::<App>::default()),
    )
}

#[derive(PartialEq)]
enum Tab {
    Codes,
    Options,
}

#[derive(EnumIter, PartialEq, Clone, Copy)]
pub enum Location {
    Mobile,
    Freephone,
    Premium,
    UK,
    None,
    Leeds,
    Sheffield,
    Nottingham,
    Leicester,
    Bristol,
    Reading,
    Birmingham,
    Edinburgh,
    Glasgow,
    Liverpool,
    Manchester,
    London,
    TynesideDurhamSunderland,
    NI,
    Cardiff,
}

impl Location {
    // Could be made by impl display
    fn as_string(&self) -> String {
        match self {
            Location::Mobile => String::from("Mobile (07)"),
            Location::Freephone => String::from("Freephone (08)"),
            Location::Premium => String::from("Premium (09)"),
            Location::UK => String::from("UK-Wide (03)"),
            Location::None => String::from("No Area (01632)"),
            Location::Leeds => String::from("Leeds (0113)"),
            Location::Sheffield => String::from("Sheffield (0114)"),
            Location::Nottingham => String::from("Nottingham (0115)"),
            Location::Leicester => String::from("Leicester (0116)"),
            Location::Bristol => String::from("Bristol (0117)"),
            Location::Reading => String::from("Reading (0118)"),
            Location::Birmingham => String::from("Birmingham (0121)"),
            Location::Edinburgh => String::from("Edinburgh (0131)"),
            Location::Glasgow => String::from("Glasgow (0141)"),
            Location::Liverpool => String::from("Liverpool (0151)"),
            Location::Manchester => String::from("Manchester (0161)"),
            Location::London => String::from("London (020)"),
            Location::TynesideDurhamSunderland => String::from("Tyneside/Durham/Sunderland (0191)"),
            Location::NI => String::from("Northern Ireland (028)"),
            Location::Cardiff => String::from("Cardiff (029)"),
        }
    }

    fn to_prefix(&self) -> String {
        match self {
            Location::Mobile => String::from("07700 900"),
            Location::Freephone => String::from("08081 570"),
            Location::Premium => String::from("0909 8790"),
            Location::UK => String::from("03069 990"),
            Location::None => String::from("01632 960"),
            Location::Leeds => String::from("0113 496 0"),
            Location::Sheffield => String::from("0114 496 0"),
            Location::Nottingham => String::from("0115 496 0"),
            Location::Leicester => String::from("0116 496 0"),
            Location::Bristol => String::from("0117 496 0"),
            Location::Reading => String::from("0118 496 0"),
            Location::Birmingham => String::from("0121 496 0"),
            Location::Edinburgh => String::from("0131 496 0"),
            Location::Glasgow => String::from("0141 496 0"),
            Location::Liverpool => String::from("0151 496 0"),
            Location::Manchester => String::from("0161 496 0"),
            Location::London => String::from("020 7846 0"),
            Location::TynesideDurhamSunderland => String::from("0191 498 0"),
            Location::NI => String::from("028 9649 6"),
            Location::Cardiff => String::from("029 2018 0"),
        }
    }

    fn from_str(from: &str) -> Result<Location, ()> {
        match from {
            "Mobile (07)" => Ok(Location::Mobile),
            "Freephone (08)" => Ok(Location::Freephone),
            "Premium (09)" => Ok(Location::Premium),
            "UK-Wide (03)" => Ok(Location::UK),
            "No Area (01632)" => Ok(Location::None),
            "Leeds (0113)" => Ok(Location::Leeds),
            "Sheffield (0114)" => Ok(Location::Sheffield),
            "Nottingham (0115)" => Ok(Location::Nottingham),
            "Leicester (0116)" => Ok(Location::Leicester),
            "Bristol (0117)" => Ok(Location::Bristol),
            "Reading (0118)" => Ok(Location::Reading),
            "Birmingham (0121)" => Ok(Location::Birmingham),
            "Edinburgh (0131)" => Ok(Location::Edinburgh),
            "Glasgow (0141)" => Ok(Location::Glasgow),
            "Liverpool (0151)" => Ok(Location::Liverpool),
            "Manchester (0161)" => Ok(Location::Manchester),
            "London (020)" => Ok(Location::London),
            "Tyneside/Durham/Sunderland (0191)" => Ok(Location::TynesideDurhamSunderland),
            "Northern Ireland (028)" => Ok(Location::NI),
            "Cardiff (029)" => Ok(Location::Cardiff),
            _ => Err(()),
        }
    }
}

struct App {
    num: String,
    tab: Tab,
    copied: bool,
    config: ini::Config,
}

impl Default for App {
    fn default() -> Self {
        let config = if let Ok(conf) = ini::load() {
            conf
        } else {
            if let Err(e) = ini::save(&Default::default()) {
                eprintln!("Error: {:?}", e);
            }
            Default::default()
        };

        Self {
            num: numbers::display(&config),
            tab: Tab::Codes,
            copied: false,
            config,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.tab {
            Tab::Codes => self.draw_code_tab(&ctx),
            Tab::Options => self.draw_option_tab(&ctx),
        }
    }
}

impl App {
    fn menu(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(self.tab == Tab::Codes, "Number")
                    .clicked()
                {
                    if self.tab != Tab::Codes {
                        self.num = numbers::display(&self.config);
                    }
                    self.tab = Tab::Codes;
                }
                if ui
                    .selectable_label(self.tab == Tab::Options, "Options")
                    .clicked()
                {
                    self.tab = Tab::Options;
                }
            })
        });
    }

    fn draw_code_tab(&mut self, ctx: &egui::Context) {
        self.menu(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().text_edit_width = 240.;
                ui.add_enabled(false, egui::TextEdit::singleline(&mut self.num));
                if ui
                    .add_sized(
                        [51., 20.],
                        egui::Button::new(if self.copied { "Copied" } else { "Copy" }),
                    )
                    .clicked()
                {
                    let mut ctx = ClipboardContext::new().expect("Could not copy value");
                    ctx.set_contents(self.num.to_owned())
                        .expect("Could not copy value");
                    self.copied = true;
                }
            });

            ui.horizontal(|ui| {
                if ui
                    .add_sized([300., 20.], egui::Button::new("Generate"))
                    .clicked()
                {
                    self.num = numbers::display(&self.config);
                    self.copied = false;
                }
            });
        });
    }

    fn draw_option_tab(&mut self, ctx: &egui::Context) {
        self.menu(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Location");
            self.location_option_menu(ui);

            ui.add_space(10.);
            ui.checkbox(&mut self.config.evil, "Evil mode");

            ui.add_space(10.);
            if ui.button("Save Options").clicked() {
                if let Err(e) = ini::save(&self.config) {
                    eprintln!("Error: {:?}", e);
                }
            }
        });
    }

    fn location_option_menu(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_id_source("Location Option")
            .selected_text(format!("{}", self.config.location.as_string()))
            .width(148.)
            .show_ui(ui, |ui| {
                for location in Location::iter() {
                    ui.selectable_value(
                        &mut self.config.location,
                        location,
                        format!("{}", location.as_string()),
                    );
                }
            });
    }
}

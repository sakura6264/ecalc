use std::collections::BTreeMap;
use eframe::egui::ScrollArea;
use eframe::egui;
use egui_extras::{TableBuilder, Column};
use super::calc;
pub struct App {
    inputs: BTreeMap<char,String>,
    consts: Vec<String>,
    truthtable: calc::TruthTable,
    gen_input_num: usize,
    gen_output_num: usize,
}
impl App {
    pub fn new() -> Self {
        Self {
            inputs: BTreeMap::new(),
            consts: Vec::new(),
            truthtable: calc::TruthTable::new(0,0).unwrap(),
            gen_input_num: 2,
            gen_output_num: 1,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut cusor = ui.cursor();
                cusor.set_height(0.0);
                cusor.set_width(ui.available_width() / 2.0 - 5.0);
                ui.allocate_ui_at_rect(cusor, |ui| {
                    ui.vertical(|ui| {
                        ScrollArea::vertical()
                            .scroll_bar_visibility(egui::containers::scroll_area::ScrollBarVisibility::AlwaysVisible)
                            .min_scrolled_height(500.0)
                            .id_source("Scroll 1")
                            .show(ui, |ui| {
                                let mut remove = Vec::new();
                                for (key, value) in self.inputs.iter_mut() {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("{} = ", key));
                                        ui.text_edit_singleline(value);
                                        if ui.button(" - ").clicked() {
                                            remove.push(*key);
                                        }
                                    ui.separator();
                                    });
                                    
                                }
                                for key in remove {
                                    self.inputs.remove(&key);
                                }
                            });
                        ui.separator();
                        ui.horizontal(|u| {
                            if u.button(" + ").clicked() {
                                for i in calc::CHARLIST.iter() {
                                    if !self.inputs.contains_key(i) {
                                        self.inputs.insert(*i, String::new());
                                        break;
                                    }
                                }
                            }
                            if u.button("Clear").clicked() {
                                self.inputs.clear();
                            }
                        });
                    });
                });
                ui.separator();
                cusor = ui.cursor();
                cusor.set_height(0.0);
                cusor.set_width(ui.available_width());
                ui.allocate_ui_at_rect(cusor, |ui| {
                    ui.vertical(|ui| {
                        ScrollArea::vertical()
                            .scroll_bar_visibility(egui::containers::scroll_area::ScrollBarVisibility::AlwaysVisible)
                            .min_scrolled_height(500.0)
                            .id_source("Scroll 2")
                            .show(ui, |ui| {
                                let mut remove = Vec::new();
                                let mut id = 0;
                                while id < self.consts.len() {
                                    ui.horizontal(|ui| {
                                        ui.label("0 =");
                                        ui.text_edit_singleline(&mut self.consts[id]);
                                        if ui.button(" - ").clicked() {
                                            remove.push(id);
                                        }});
                                    id +=1;
                                }
                                for key in remove {
                                    self.consts.remove(key);
                                }
                            });
                        ui.separator();
                        ui.horizontal(|u| {
                                if u.button(" + ").clicked() {
                                   self.consts.push(String::new());
                                }
                                if u.button("Clear").clicked() {
                                    self.consts.clear();
                                }
                        });
                    });
                });
            });
            ui.separator();
            ui.horizontal(|ui|{
                if ui.button("Calculate").clicked() {
                    match calc::TruthTable::calc(&self.inputs, &self.consts) {
                        Ok(table) => {
                            self.truthtable = table;
                        },
                        Err(err) => {
                            simple_message_box::create_message_box(&err, "Error");
                        }
                    }
                }
                if ui.button("Export").clicked() {
                    if let Some(p) = rfd::FileDialog::new().add_filter("JSON File", &["json"]).save_file() {
                        if let Ok(s) = serde_json::to_string(&self.truthtable) {
                            if let Err(err) = std::fs::write(p, s) {
                                simple_message_box::create_message_box(&err.to_string(), "Error");
                            }
                        }
                        else {
                            simple_message_box::create_message_box("Failed to serialize", "Error");
                        };
                    }
                }
                if ui.button("Import").clicked() {
                    if let Some(p) = rfd::FileDialog::new().add_filter("JSON File", &["json"]).pick_file() {
                        if let Ok(s) = std::fs::read_to_string(p) {
                            if let Ok(table) = serde_json::from_str::<calc::TruthTable>(&s) {
                                self.truthtable = table;
                            }
                            else {
                                simple_message_box::create_message_box("Failed to deserialize", "Error");
                            }
                        }
                        else {
                            simple_message_box::create_message_box("Failed to read file", "Error");
                        }
                    }
                }
                if ui.button("Simplify").clicked() {
                    self.inputs = calc::qmc_simplify(&self.truthtable);
                    self.consts.clear();
                }
                if ui.button("Clear").clicked() {
                    self.truthtable = calc::TruthTable::new(0,0).unwrap();
                }
                if ui.button("Generate").on_hover_text("Generate a graphic in dot format").clicked() {
                    match calc::generate_graphic(&self.truthtable) {
                        Ok(s) => {
                            if let Some(p) = rfd::FileDialog::new().add_filter("DOT File", &["dot"]).save_file() {
                                if let Err(err) = std::fs::write(p, s) {
                                    simple_message_box::create_message_box(&err.to_string(), "Error");
                                }
                            }
                        },
                        Err(e) => {
                            simple_message_box::create_message_box(&e, "Error");
                        }
                    }
                }
                if ui.button("New").clicked() {
                    match calc::TruthTable::new(self.gen_input_num, self.gen_output_num) {
                        Some(table) => {
                            self.truthtable = table;
                        },
                        None => {
                            simple_message_box::create_message_box("Too Many Variables", "Error");
                        }
                    }
                }
                ui.label("Input:");
                ui.add(egui::DragValue::new(&mut self.gen_input_num).speed(1.0).clamp_range(0..=calc::CHARLIST.len()));
                ui.label("Output:");
                ui.add(egui::DragValue::new(&mut self.gen_output_num).speed(1.0).clamp_range(0..=calc::CHARLIST.len()));
            });

            let mut cusor = ui.cursor();
            cusor.set_height(0.0);
            cusor.set_width(ui.available_width());
            ui.allocate_ui_at_rect(cusor, |ui| {
                ui.vertical(|ui| {
                    
                            let mut builder = TableBuilder::new(ui);
                            builder = builder.min_scrolled_height(500.0);
                            builder = builder.column(Column::auto().resizable(true).at_least(60.0));
                            for _ in 0..self.truthtable.vars.len() {
                                builder = builder.column(Column::auto().resizable(true).at_least(40.0));
                            }
                            builder = builder.column(Column::auto().resizable(true).at_least(60.0));
                            for _ in 0..self.truthtable.outputs.len() {
                                builder = builder.column(Column::auto().resizable(true).at_least(40.0));
                            }
                            builder = builder.column(Column::auto().resizable(false).at_least(20.0));
                            let table = builder.header(20.0, |mut header|{
                                header.col(|ui| {ui.label("Input");});
                                for i in 0..self.truthtable.vars.len() {
                                    header.col(|ui| {ui.label(format!(" {} ",&self.truthtable.vars[i]));});
                                }
                                header.col(|ui| {ui.label("Output");});
                                for i in 0..self.truthtable.outputs.len() {
                                    header.col(|ui| {ui.label(format!(" {} ",&self.truthtable.outputs[i]));});
                                }
                                header.col(|_| {});
                            });
                            table.body(|mut body|{
                                let mut input = BTreeMap::new();
                                let mut index = 0;
                                for i in self.truthtable.vars.iter() {
                                    input.insert(i.clone(), false);
                                }
                                for _ in 0..2usize.pow(input.len( ) as u32) {
                                    body.row(30.0, |mut row|{
                                        row.col(|_|{});
                                        for i in 0..self.truthtable.vars.len() {
                                            row.col(|ui| {
                                                ui.label(if *input.get(&self.truthtable.vars[i]).unwrap() {"1"} else {"0"});
                                            });
                                        }
                                        row.col(|_|{});
                                        for i in 0..self.truthtable.outputs.len() {
                                            row.col(|ui|{
                                                if ui.button(
                                                    match self.truthtable.table[index][i] {
                                                        calc::TruthTableResult::Val(true) => " 1 ",
                                                        calc::TruthTableResult::Val(false) => " 0 ",
                                                        calc::TruthTableResult::NotCare => " X ",
                                                    }
                                                ).clicked() {
                                                    match self.truthtable.table[index][i] {
                                                        calc::TruthTableResult::Val(true) => self.truthtable.table[index][i] = calc::TruthTableResult::Val(false),
                                                        calc::TruthTableResult::Val(false) => self.truthtable.table[index][i] = calc::TruthTableResult::NotCare,
                                                        calc::TruthTableResult::NotCare => self.truthtable.table[index][i] = calc::TruthTableResult::Val(true),
                                                    }
                                                }
                                            });
                                        }
                                        row.col(|_|{});
                                    });
                                    calc::TruthTable::next(&mut input);
                                    index +=1;
                                }
                            });
                        });
                });
        });
    }
}

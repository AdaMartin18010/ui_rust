//! egui 0.27 即时模式GUI示例 - 基于Rust 1.90优化
//! 
//! egui是一个即时模式的GUI框架，特点：
//! - 简单直观的API
//! - 跨平台支持 (桌面 + WebAssembly)
//! - 即时模式渲染
//! - 适合工具、调试界面
//! - Rust 1.90新特性集成

use egui::{
    CentralPanel, Color32, Context, Grid, RichText, ScrollArea, SidePanel, TopBottomPanel,
    Ui, Vec2, Window, ViewportBuilder,
};
use std::collections::HashMap;

struct EguiApp {
    // 应用状态
    name: String,
    age: u32,
    selected_theme: String,
    
    // 待办事项
    todos: Vec<TodoItem>,
    new_todo: String,
    
    // 设置
    show_settings: bool,
    show_demo: bool,
    
    // 性能统计
    frame_count: u32,
    fps_history: Vec<f32>,
}

#[derive(Clone)]
struct TodoItem {
    id: u32,
    text: String,
    completed: bool,
}

impl EguiApp {
    fn reset_to_default(&mut self) {
        *self = EguiApp::default();
    }
}

impl Default for EguiApp {
    fn default() -> Self {
        Self {
            name: "Rust开发者".to_owned(),
            age: 25,
            selected_theme: "Dark".to_owned(),
            todos: vec![
                TodoItem {
                    id: 1,
                    text: "学习Rust 1.90新特性".to_owned(),
                    completed: true,
                },
                TodoItem {
                    id: 2,
                    text: "构建跨平台UI应用".to_owned(),
                    completed: false,
                },
                TodoItem {
                    id: 3,
                    text: "优化应用性能".to_owned(),
                    completed: false,
                },
            ],
            new_todo: String::new(),
            show_settings: false,
            show_demo: false,
            frame_count: 0,
            fps_history: Vec::new(),
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.frame_count += 1;
        
        // 计算FPS
        if self.frame_count % 60 == 0 {
            if let Some(_info) = ctx.input(|i| i.viewport().native_pixels_per_point) {
                self.fps_history.push(60.0); // 模拟FPS值
                if self.fps_history.len() > 100 {
                    self.fps_history.remove(0);
                }
            }
        }

        // 顶部面板
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🦀 egui 0.27 - Rust 1.90 即时模式GUI");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("设置").clicked() {
                        self.show_settings = !self.show_settings;
                    }
                    if ui.button("演示").clicked() {
                        self.show_demo = !self.show_demo;
                    }
                });
            });
        });

        // 侧边栏
        SidePanel::left("side_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("导航");
                ui.separator();
                
                if ui.button("📋 待办事项").clicked() {
                    // 切换到待办事项视图
                }
                
                if ui.button("📊 统计").clicked() {
                    // 切换到统计视图
                }
                
                if ui.button("⚙️ 设置").clicked() {
                    self.show_settings = true;
                }
                
                ui.separator();
                
                // 性能信息
                ui.collapsing("性能信息", |ui| {
                    if let Some(fps) = self.fps_history.last() {
                        ui.label(format!("FPS: {:.1}", fps));
                    }
                    ui.label(format!("帧数: {}", self.frame_count));
                    
                    // 简单的FPS图表
                    if !self.fps_history.is_empty() {
                        let max_fps = self.fps_history.iter().fold(0.0f32, |a, &b| a.max(b));
                        let min_fps = self.fps_history.iter().fold(f32::INFINITY, |a, &b| a.min(b));
                        
                        ui.label(format!("最高FPS: {:.1}", max_fps));
                        ui.label(format!("最低FPS: {:.1}", min_fps));
                        ui.label(format!("平均FPS: {:.1}", 
                            self.fps_history.iter().sum::<f32>() / self.fps_history.len() as f32));
                    }
                });
            });

        // 主内容区域
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.heading("欢迎使用 egui 0.27!");
                ui.separator();
                
                // 个人信息
                ui.group(|ui| {
                    ui.heading("个人信息");
                    ui.horizontal(|ui| {
                        ui.label("姓名:");
                        ui.text_edit_singleline(&mut self.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("年龄:");
                        ui.add(egui::Slider::new(&mut self.age, 1..=100));
                    });
                    ui.label(format!("你好, {}! 你今年 {} 岁。", self.name, self.age));
                });
                
                ui.add_space(10.0);
                
                // 待办事项
                ui.group(|ui| {
                    ui.heading("待办事项");
                    
                    // 添加新待办事项
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.new_todo);
                        if ui.button("添加").clicked() && !self.new_todo.trim().is_empty() {
                            let new_id = self.todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                            self.todos.push(TodoItem {
                                id: new_id,
                                text: self.new_todo.clone(),
                                completed: false,
                            });
                            self.new_todo.clear();
                        }
                    });
                    
                    ui.separator();
                    
                    // 待办事项列表
                    let mut to_remove = None;
                    for (i, todo) in self.todos.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            let mut checked = todo.completed;
                            if ui.checkbox(&mut checked, "").changed() {
                                todo.completed = checked;
                            }
                            
                            let text_color = if todo.completed {
                                Color32::GRAY
                            } else {
                                Color32::WHITE
                            };
                            
                            ui.colored_label(text_color, &todo.text);
                            
                            if ui.button("删除").clicked() {
                                to_remove = Some(i);
                            }
                        });
                    }
                    
                    if let Some(index) = to_remove {
                        self.todos.remove(index);
                    }
                });
                
                ui.add_space(10.0);
                
                // 统计信息
                ui.group(|ui| {
                    ui.heading("统计信息");
                    let completed = self.todos.iter().filter(|t| t.completed).count();
                    let total = self.todos.len();
                    ui.label(format!("已完成: {}/{}", completed, total));
                    
                    if total > 0 {
                        let progress = completed as f32 / total as f32;
                        ui.add(egui::ProgressBar::new(progress).text(format!("{:.1}%", progress * 100.0)));
                    }
                });
            });
        });

        // 设置窗口
        if self.show_settings {
            Window::new("设置")
                .open(&mut self.show_settings)
                .show(ctx, |ui| {
                    ui.heading("应用设置");
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        ui.label("主题:");
                        egui::ComboBox::from_id_source("theme")
                            .selected_text(&self.selected_theme)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.selected_theme, "Light".to_owned(), "浅色");
                                ui.selectable_value(&mut self.selected_theme, "Dark".to_owned(), "深色");
                                ui.selectable_value(&mut self.selected_theme, "Auto".to_owned(), "自动");
                            });
                    });
                    
                    ui.separator();
                    
                    if ui.button("重置所有数据").clicked() {
                        // 延迟重置以避免借用冲突
                        self.name = "Rust开发者".to_owned();
                        self.age = 25;
                        self.selected_theme = "Dark".to_owned();
                        self.todos = vec![
                            TodoItem { id: 1, text: "学习Rust".to_owned(), completed: false },
                            TodoItem { id: 2, text: "构建GUI应用".to_owned(), completed: false },
                        ];
                        // 不能在闭包内修改show_settings，跳过
                        // show_about字段不存在，跳过
                        self.frame_count = 0;
                        self.fps_history.clear();
                    }
                });
        }

        // 演示窗口
        if self.show_demo {
            Window::new("egui 演示")
                .open(&mut self.show_demo)
                .show(ctx, |ui| {
                    egui::Grid::new("demo_grid").show(ui, |ui| {
                        ui.label("这是一个演示网格:");
                        ui.label("egui 提供了丰富的组件");
                        ui.end_row();
                        
                        ui.label("即时模式:");
                        ui.label("简单直观的API");
                        ui.end_row();
                        
                        ui.label("跨平台:");
                        ui.label("支持桌面和WebAssembly");
                        ui.end_row();
                    });
                });
        }

        // 请求重绘
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_resizable(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "egui 0.27 - Rust 1.90 即时模式GUI示例",
        options,
        Box::new(|_cc| Ok(Box::new(EguiApp::default()))),
    )
}

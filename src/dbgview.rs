use eframe::{
    egui::{self, ComboBox},
    EventLoopBuilderHook, RequestRepaintEvent,
};
use egui_winit::winit::{
    event_loop::EventLoopBuilder, platform::windows::EventLoopBuilderExtWindows,
};
use rrplug::prelude::ScriptContext;
use std::sync::mpsc::Receiver;

use crate::{exports::PLUGIN, stacktrace::StackTrace, VmSpecific};

pub fn init_window(recv: Receiver<(ScriptContext, StackTrace)>) {
    let func = |event_loop_builder: &mut EventLoopBuilder<RequestRepaintEvent>| {
        event_loop_builder.with_any_thread(true);
    };

    let event_loop_builder: Option<EventLoopBuilderHook> = Some(Box::new(func));

    let options = eframe::NativeOptions {
        drag_and_drop_support: false,
        icon_data: None,
        resizable: true,
        follow_system_theme: false,
        run_and_return: false,
        event_loop_builder,
        centered: true,
        initial_window_size: Some(eframe::epaint::Vec2::new(500., 400.)),

        ..Default::default()
    };

    eframe::run_native(
        "Debugger :3",
        options,
        Box::new(move |_cc| Box::new(Window::new(recv))),
    );
}
struct Window {
    context: ScriptContext,
    recv: Receiver<(ScriptContext, StackTrace)>,
    sqlog: VmSpecific<Vec<StackTrace>>,
}

impl Window {
    fn new(recv: Receiver<(ScriptContext, StackTrace)>) -> Self {
        Self {
            context: ScriptContext::UI,
            recv,
            sqlog: VmSpecific::new(),
        }
    }
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok((context, log)) = self.recv.try_recv() {
            self.sqlog.get_mut(context).push(log)
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ComboBox::from_label("SQVM context")
                .selected_text(format!("{:?}", self.context))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.context, ScriptContext::SERVER, "SERVER");
                    ui.selectable_value(&mut self.context, ScriptContext::CLIENT, "CLIENT");
                    ui.selectable_value(&mut self.context, ScriptContext::UI, "UI");
                });

            ui.horizontal(|ui| {
                if ui.button("Pause").clicked() {
                    (*PLUGIN.wait().debug_info.get(self.context).paused.lock()) = true;
                }

                if ui.button("Unpause").clicked() {
                    (*PLUGIN.wait().debug_info.get(self.context).paused.lock()) = false;
                }

                if ui.button("continue").clicked() {
                    _ = PLUGIN
                        .wait()
                        .debug_info
                        .get(self.context)
                        .unpause_breaker
                        .lock()
                        .send(());
                }
            });

            ui.label("Squirrel Functions log");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for trace in self.sqlog.get(self.context) {
                    ui.label(format!("{trace}"));
                }
            });
        });
    }
}

use crate::gui::app::VERSION;
use crate::gui::tab_types::all_colors::AllColorsPlot;
use crate::gui::tab_types::PlotType::*;
use crate::gui::tab_types::{default_plot, PlotStruct, PlotType};
use crate::gui::tabs::TabAction::*;
use egui::{warn_if_debug_build, Context, Id, Ui};
use egui_dock::Node::Leaf;
use egui_dock::{DockArea, NodeIndex, Style, TabIndex, Tree};
use std::fmt::{Debug, Formatter};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use strum::IntoEnumIterator;

// todo: use atomic usize
static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
pub struct Tab {
    pub name: String,
    pub plot_type: PlotType,
    pub plot: Box<dyn PlotStruct>,
    id: usize,
    node: NodeIndex,
}

impl Debug for Tab {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tab")
            .field("name", &self.name)
            .field("plot_type", &self.plot_type)
            .field("id", &self.id)
            .field("node", &self.node)
            .finish()
    }
}

impl Default for Tab {
    fn default() -> Self {
        Self {
            name: "New Tab".to_string(),
            plot: Box::<AllColorsPlot>::default(),
            plot_type: AllColors,
            id: ID_COUNTER.fetch_add(1, Relaxed),
            node: NodeIndex::root(),
        }
    }
}

impl Tab {
    pub fn new(plot_type: PlotType, node_index: usize) -> Self {
        Self {
            name: plot_type.to_string(),
            plot: default_plot(plot_type),
            plot_type,
            id: ID_COUNTER.fetch_add(1, Relaxed),
            node: NodeIndex(node_index),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MyTabs {
    pub tree: Tree<Tab>,
    counter: usize,
}

enum TabAction {
    AddTabs(Vec<Tab>),
    CloseAll,
    CloseAllExcept(usize),
}

impl Default for MyTabs {
    fn default() -> Self {
        Self::new()
    }
}

impl MyTabs {
    pub fn new() -> Self {
        let tree = Tree::new(vec![
            Tab::new(AllColors, 1),

            Tab::new(NeuralNetwork, 2),
            Tab::new(Other, 3),
        ]);

        Self { tree, counter: 3 }
    }

    pub fn ui(&mut self, ctx: &Context) {
        let mut tab_action = None;
        DockArea::new(&mut self.tree)
            .show_add_buttons(true)
            .show_add_popup(true)
            .style({
                let style = Style::from_egui(ctx.style().as_ref());
                // style.tabs.fill_tab_bar = true;
                style
            })
            .show(
                ctx,
                &mut TabViewer {
                    tab_action: &mut tab_action,
                },
            );

        if let Some(tab_action) = tab_action {
            match tab_action {
                AddTabs(mut added_nodes) => {
                    added_nodes.drain(..).for_each(|node| {
                        self.tree.set_focused_node(node.node);
                        self.tree.push_to_focused_leaf(node);
                        self.counter += 1;
                    });
                }
                CloseAllExcept(tab_except) => {
                    println!("close all except {:?}", tab_except);
                    let mut to_focus = (NodeIndex::root(), 0);
                    for tab in self.tree.iter_mut() {
                        if let Leaf { ref mut tabs, .. } = tab {
                            for i in (0..tabs.len()).rev() {
                                if tabs[i].id != tab_except {
                                    tabs.remove(i);
                                } else {
                                    // focus on the tab
                                    to_focus = (tabs[i].node, i);
                                }
                            }
                        }
                    }
                    self.tree.set_focused_node(to_focus.0);
                    self.tree
                        .set_active_tab(to_focus.0, TabIndex::from(to_focus.1));
                }
                CloseAll => {
                    println!("close all tabs");
                    for tab in self.tree.iter_mut() {
                        // while !tab.tabs_count() > 0 {
                        //     tab.remove_tab(TabIndex::from(0));
                        // }
                        // dbg!(tab);
                        if let Leaf { tabs, .. } = tab {
                            tabs.clear();
                        }
                    }
                }
            }
        }
    }
}

// #[derive(serde::Deserialize, serde::Serialize)]
struct TabViewer<'a> {
    tab_action: &'a mut Option<TabAction>,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = Tab;

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        ui.heading(tab.plot.title());
        // ui.add(egui::github_link_file!(
        //         "https://github.com/TomtheCoder2/EV3_summer_2022/blob/master/",
        //         "Source code."
        //     ));
        // egui::warn_if_debug_build(ui);
        ui.separator();
        // ui.label("Double click to auto adjust zoom.");
        if tab.plot.show_interface() {
            egui::SidePanel::left(format!("left_panel_{}", tab.id))
                .resizable(true)
                .show_inside(ui, |ui| {
                    tab.plot.interface(ui);
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                        warn_if_debug_build(ui);
                        ui.label(format!("Version: {}", VERSION));
                    });
                });
        }
        egui::CentralPanel::default().show_inside(ui, |ui| {
            tab.plot.plot(ui);
        });
    }

    fn context_menu(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        // close all button
        if ui.button("Close all").clicked() {
            *self.tab_action = Some(CloseAll);
        }
        // close others
        if ui.button("Close others").clicked() {
            *self.tab_action = Some(CloseAllExcept(tab.id));
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.plot.name().into()
    }

    fn id(&mut self, tab: &mut Self::Tab) -> Id {
        Id::new(tab.id)
    }

    fn add_popup(&mut self, ui: &mut Ui, node: NodeIndex) {
        ui.set_min_width(120.0);
        ui.style_mut().visuals.button_frame = false;

        ui.label("Add a new plot:");
        ui.separator();
        ui.label("Plot type:");
        let mut tab = Tab::new(Other, node.0);
        for plot_type in PlotType::iter() {
            if ui.button(plot_type.to_string()).clicked() {
                tab.plot_type = plot_type;
                tab.plot = default_plot(plot_type);
                tab.name = plot_type.to_string();
                *self.tab_action = Some(AddTabs(vec![tab]));
                break;
            }
        }
    }
}

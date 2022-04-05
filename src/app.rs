use eframe::{egui, epi};

struct Element {
 label: String,
 fontsize: f32,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
#[derive(Default)]
pub struct TemplateApp {
 elements: Vec<Element>,
 selected_id: usize,
}

impl epi::App for TemplateApp {
 fn name(&self) -> &str {
  "eframe template"
 }

 /// Called once before the first frame.
 fn setup(
  &mut self,
  ctx: &egui::Context,
  _frame: &epi::Frame,
  _storage: Option<&dyn epi::Storage>,
 ) {
  ctx.set_pixels_per_point(3.0);
  // Load previous app state (if any).
  // Note that you must enable the `persistence` feature for this to work.
  #[cfg(feature = "persistence")]
  if let Some(storage) = _storage {
   *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
  }
 }

 /// Called by the frame work to save state before shutdown.
 /// Note that you must enable the `persistence` feature for this to work.
 #[cfg(feature = "persistence")]
 fn save(&mut self, storage: &mut dyn epi::Storage) {
  epi::set_value(storage, epi::APP_KEY, self);
 }

 /// Called each time the UI needs repainting, which may be many times per second.
 /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
 fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
  let Self { elements, selected_id } = self;

  // Examples of how to create different panels and windows.
  // Pick whichever suits you.
  // Tip: a good default choice is to just keep the `CentralPanel`.
  // For inspiration and more examples, go to https://emilk.github.io/egui

  egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
   // The top panel is often a good place for a menu bar:
   egui::menu::bar(ui, |ui| {
    ui.menu_button("File", |ui| {
     if ui.button("Quit").clicked() {
      frame.quit();
     }
    });
   });
  });

  egui::SidePanel::left("side_panel").show(ctx, |ui| {
   if ui.button("New Label").clicked() {
    elements.push(Element { label: "New Label".to_string(), fontsize: 20.0 });
   }

   for (i, el) in elements.iter().enumerate() {
    let mut checked = *selected_id == i;
    ui.checkbox(&mut checked, el.label.clone());
    if checked {
     *selected_id = i;
    }
   }
  });

  egui::SidePanel::right("inspector").show(ctx, |ui| {
   ui.heading("Inspector");

   if !elements.is_empty() {
    let Element { label, fontsize } = &mut elements[*selected_id];

    ui.label("Title");
    ui.text_edit_singleline(label);

    ui.label("Font Size");
    ui.add(egui::Slider::new(fontsize, 10.0..=40.0));
   }
  });

  egui::CentralPanel::default().show(ctx, |ui| {
   // The central panel the region left after adding TopPanel's and SidePanel's
   for (_i, el) in elements.iter().enumerate() {
    if (ui.add(
     egui::Label::new(
      egui::RichText::new(el.label.clone()).font(egui::FontId::proportional(el.fontsize)),
     )
     .sense(egui::Sense::click()),
    ))
    .clicked()
    {};
   }

   ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
    ui.horizontal(|ui| {
     egui::warn_if_debug_build(ui);
    });
   });
  });

  if false {
   egui::Window::new("Window").show(ctx, |ui| {
    ui.label("Windows can be moved by dragging them.");
    ui.label("They are automatically sized based on contents.");
    ui.label("You can turn on resizing and scrolling if you like.");
    ui.label("You would normally chose either panels OR windows.");
   });
  }
 }
}

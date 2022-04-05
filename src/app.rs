use eframe::{egui, epi};

enum InnerElement {
 Label {},
 Button {},
}

struct Element {
 name: String,
 fontsize: f32,
 el: InnerElement,
}

#[derive(Default)]
pub struct TemplateApp {
 elements: Vec<Element>,
 selected_id: usize,
}

impl epi::App for TemplateApp {
 fn name(&self) -> &str {
  "egui_play"
 }

 /// Called once before the first frame.
 fn setup(
  &mut self,
  ctx: &egui::Context,
  _frame: &epi::Frame,
  _storage: Option<&dyn epi::Storage>,
 ) {
  ctx.set_pixels_per_point(3.0);
 }

 /// Called each time the UI needs repainting, which may be many times per second.
 /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
 fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
  let Self { elements, selected_id } = self;

  egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
   egui::menu::bar(ui, |ui| {
    ui.menu_button("File", |ui| {
     if ui.button("Quit").clicked() {
      frame.quit();
     }
    });
   });
  });

  egui::SidePanel::left("side_panel").show(ctx, |ui| {
   ui.menu_button("New Element", |ui| {
    if ui.button("Label").clicked() {
     elements.push(Element {
      name: "New Label".to_string(),
      fontsize: 25.0,
      el: InnerElement::Label {},
     });
     ui.close_menu();
    }
    if ui.button("Button").clicked() {
     elements.push(Element {
      name: "New Button".to_string(),
      fontsize: 15.0,
      el: InnerElement::Button {},
     });
     ui.close_menu();
    }
   });

   for (i, el) in elements.iter().enumerate() {
    let mut checked = *selected_id == i;
    ui.checkbox(&mut checked, el.name.clone());
    if checked {
     *selected_id = i;
    }
   }
  });

  egui::SidePanel::right("inspector").show(ctx, |ui| {
   ui.heading("Inspector");

   if !elements.is_empty() {
    let Element { name, fontsize, .. } = &mut elements[*selected_id];

    ui.label("Title");
    ui.text_edit_singleline(name);

    ui.label("Font Size");
    ui.add(egui::Slider::new(fontsize, 10.0..=40.0));
   }
  });

  egui::CentralPanel::default().show(ctx, |ui| {
   for (_i, el) in elements.iter().enumerate() {
    match el {
     Element { name, fontsize, el: InnerElement::Label {} } => {
      if (ui.add(
       egui::Label::new(
        egui::RichText::new(el.name.clone()).font(egui::FontId::proportional(el.fontsize)),
       )
       .sense(egui::Sense::click()),
      ))
      .clicked()
      {};
     }
     Element { name, fontsize, el: InnerElement::Button {} } => {
      if (ui.add(
       egui::Button::new(
        egui::RichText::new(el.name.clone()).font(egui::FontId::proportional(el.fontsize)),
       )
       .sense(egui::Sense::click()),
      ))
      .clicked()
      {};
     }
    }
   }

   ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
    ui.horizontal(|ui| {
     egui::warn_if_debug_build(ui);
    });
   });
  });
 }
}

fn nested_menus(ui: &mut egui::Ui) {
 if ui.button("Label").clicked() {}
 if ui.button("Button").clicked() {}

 //  ui.menu_button("SubMenu", |ui| {
 //   ui.menu_button("SubMenu", |ui| {
 //    if ui.button("Open...").clicked() {
 //     ui.close_menu();
 //    }
 //    let _ = ui.button("Item");
 //   });
 //   ui.menu_button("SubMenu", |ui| {
 //    if ui.button("Open...").clicked() {
 //     ui.close_menu();
 //    }
 //    let _ = ui.button("Item");
 //   });
 //   let _ = ui.button("Item");
 //   if ui.button("Open...").clicked() {
 //    ui.close_menu();
 //   }
 //  });
 //  ui.menu_button("SubMenu", |ui| {
 //   let _ = ui.button("Item1");
 //   let _ = ui.button("Item2");
 //   let _ = ui.button("Item3");
 //   let _ = ui.button("Item4");
 //   if ui.button("Open...").clicked() {
 //    ui.close_menu();
 //   }
 //  });
 //  let _ = ui.button("Very long text for this item");
}

use std::{collections::BTreeMap, sync::Arc};

use bevy_inspector_egui::{
    bevy_egui::EguiContexts,
    egui::{self, CornerRadius, Stroke, Style, Visuals},
};

pub fn init_egui_font(mut egui_context: EguiContexts) {
    let ctx: &mut egui::Context = egui_context.ctx_mut().expect("Can't get mut ctx");

    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "Font".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/kaivs_minegram_v1.ttf"
        ))),
    );

    fonts.families.insert(
        egui::FontFamily::Name("kaivs_minegram_v1".into()),
        vec!["regular".to_owned()],
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "Font".to_owned());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "Font".to_owned());

    ctx.set_fonts(fonts);

    let style = Style {
        //override_text_style: Some(egui::TextStyle::Monospace),

        //drag_value_text_style: todo!(),
        //wrap: todo!(),
        //spacing: todo!(),
        //interaction: todo!(),
        text_styles: BTreeMap::from([
            (
                egui::TextStyle::Heading,
                egui::FontId::new(32.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::new(32.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Monospace,
                egui::FontId::new(32.0, egui::FontFamily::Monospace),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::new(32.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Small,
                egui::FontId::new(32.0, egui::FontFamily::Proportional),
            ),
        ]),
        visuals: Visuals {
            dark_mode: true,
            override_text_color: Some(egui::Color32::from_rgba_unmultiplied(220, 220, 220, 255)),
            //widgets: todo!(),
            //selection: todo!(),
            //hyperlink_color: todo!(),
            //faint_bg_color: todo!(),

            // window_rounding: Rounding::ZERO,
            window_shadow: egui::Shadow::NONE,
            window_fill: egui::Color32::from_rgba_unmultiplied(20, 20, 20, 230),
            // window_stroke: egui::Stroke{
            //     width: 1.,
            //     color: egui::Color32::from_rgba_unmultiplied(220, 220, 220, 255)
            // },
            window_stroke: Stroke::NONE,
            button_frame: false,
            interact_cursor: None,
            menu_corner_radius: CornerRadius::ZERO,
            window_corner_radius: CornerRadius::ZERO,

            //menu_rounding: todo!(),
            //panel_fill: todo!(),
            //popup_shadow: todo!(),
            //resize_corner_size: todo!(),
            //text_cursor_width: todo!(),
            //text_cursor_preview: todo!(),
            //clip_rect_margin: todo!(),
            //button_frame: todo!(),
            //collapsing_header_frame: todo!(),
            //indent_has_left_vline: todo!(),
            //striped: todo!(),
            //slider_trailing_fill: todo!(),
            ..Default::default()
        },
        animation_time: 0.,
        //debug: todo!(),
        //explanation_tooltips: todo!(),
        ..Default::default()
    };

    ctx.set_style(style.clone());
}

#![allow(clippy::uninlined_format_args)]

use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::gradient::Gradient;
use lemao_core::lemao_math::gradient::GradientStep;
use lemao_core::lemao_math::gradient::GradientType;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::Color;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::storage::FontStorage;
use lemao_core::renderer::fonts::Font;
use lemao_core::renderer::textures::bmp;
use lemao_core::renderer::textures::storage::TextureStorage;
use lemao_core::renderer::textures::Texture;
use lemao_core::window::context::WindowContext;
use lemao_ui::components::button::Button;
use lemao_ui::components::checkbox::Checkbox;
use lemao_ui::components::label::Label;
use lemao_ui::components::panel::Panel;
use lemao_ui::components::progressbar::ProgressBar;
use lemao_ui::components::scrollbox::Scrollbox;
use lemao_ui::components::slider::Slider;
use lemao_ui::components::textbox::TextBox;
use lemao_ui::components::Component;
use lemao_ui::components::ComponentBorderThickness;
use lemao_ui::components::ComponentCornerRounding;
use lemao_ui::components::ComponentMargin;
use lemao_ui::components::ComponentPosition;
use lemao_ui::components::ComponentShape;
use lemao_ui::components::ComponentSize;
use lemao_ui::components::HorizontalAlignment;
use lemao_ui::context::UiContext;
use lemao_ui::events::UiEvent;
use std::sync::Arc;
use std::sync::Mutex;

pub fn main() -> Result<(), String> {
    let textures = Arc::new(Mutex::new(TextureStorage::default()));
    let fonts = Arc::new(Mutex::new(FontStorage::default()));

    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = match WindowContext::new("UI", WindowStyle::Window { position: window_position, size: window_size }) {
        Ok(window) => window,
        Err(message) => panic!("{}", message),
    };

    let mut renderer = match window.create_renderer(textures.clone(), fonts.clone()) {
        Ok(renderer) => renderer,
        Err(message) => panic!("{}", message),
    };

    let coin_icon = bmp::load("./assets/coin.bmp")?;
    let hammer_icon = bmp::load("./assets/hammer.bmp")?;
    let happiness_icon = bmp::load("./assets/happiness.bmp")?;
    let header_font = bff::load("./assets/header.bff")?;
    let mut regular_font = bff::load("./assets/regular.bff")?;
    let mut bold_font = bff::load("./assets/bold.bff")?;

    regular_font.set_character(200, Vec2::new(0.0, 4.0), &coin_icon);
    regular_font.set_character(201, Vec2::new(0.0, 3.0), &hammer_icon);
    regular_font.set_character(202, Vec2::new(0.0, 3.0), &happiness_icon);

    bold_font.set_character(200, Vec2::new(0.0, 4.0), &coin_icon);
    bold_font.set_character(201, Vec2::new(0.0, 3.0), &hammer_icon);
    bold_font.set_character(202, Vec2::new(0.0, 3.0), &happiness_icon);

    let regular_font_id = fonts.lock().unwrap().store(Font::new(&renderer, &regular_font));
    let bold_font_id = fonts.lock().unwrap().store(Font::new(&renderer, &bold_font));
    let header_font_id = fonts.lock().unwrap().store(Font::new(&renderer, &header_font));
    let texture_id = textures.lock().unwrap().store(Texture::new(&renderer, &bmp::load("./assets/wheat.bmp")?));
    let box_checked_id = textures.lock().unwrap().store(Texture::new(&renderer, &bmp::load("./assets/box_checked.bmp")?));
    let box_unchecked_id = textures.lock().unwrap().store(Texture::new(&renderer, &bmp::load("./assets/box_unchecked.bmp")?));
    let mut ui = UiContext::new(&mut renderer)?;

    let mut progressbar_background_gradient = Gradient::new(GradientType::Vertical, Vec2::new(0.0, 0.0));
    progressbar_background_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(72, 79, 92, 255), 0.0));
    progressbar_background_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(8, 12, 20, 255), 0.5));
    progressbar_background_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(34, 34, 42, 255), 1.0));

    let mut bar_1_gradient = Gradient::new(GradientType::Vertical, Vec2::new(0.0, 0.0));
    bar_1_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(8, 161, 41, 255), 0.0));
    bar_1_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(8, 161, 41, 255), 0.55));
    bar_1_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(50, 172, 78, 255), 0.55));
    bar_1_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(50, 172, 78, 255), 1.0));

    let mut bar_2_gradient = Gradient::new(GradientType::Vertical, Vec2::new(0.0, 0.0));
    bar_2_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(8, 131, 21, 255), 0.0));
    bar_2_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(8, 131, 21, 255), 0.55));
    bar_2_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(10, 152, 38, 255), 0.55));
    bar_2_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(10, 152, 38, 255), 1.0));

    let progressbar_id = ui.create_progressbar(&mut renderer, regular_font_id)?;
    let progressbar = ui.get_component_with_type_mut::<ProgressBar>(progressbar_id)?;
    progressbar.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0)));
    progressbar.set_size(ComponentSize::Relative(Vec2::new(0.6, 1.0)));
    progressbar.set_max_size(Vec2::new(f32::MAX, 20.0));
    progressbar.set_anchor(Vec2::new(0.5, 1.0));
    progressbar.set_offset(Vec2::new(0.0, -10.0));
    progressbar.set_color(Color::Gradient(progressbar_background_gradient));
    progressbar.set_border_color(Color::SolidColor(SolidColor::new_rgb(72, 79, 92, 255)));
    progressbar.set_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0))?;
    progressbar.set_corner_rounding(ComponentCornerRounding::new(3.0, 3.0, 3.0, 3.0));
    progressbar.set_bar_visibility(0, true);
    progressbar.set_bar_from(0, 0.0);
    progressbar.set_bar_to(0, 0.45);
    progressbar.set_bar_color(0, Color::Gradient(bar_1_gradient.clone()));
    progressbar.set_bar_corner_rounding(0, ComponentCornerRounding::new(3.0, 3.0, 3.0, 3.0));
    progressbar.set_bar_visibility(1, true);
    progressbar.set_bar_from(1, 0.0);
    progressbar.set_bar_to(1, 0.55);
    progressbar.set_bar_color(1, Color::Gradient(bar_2_gradient.clone()));
    progressbar.set_bar_corner_rounding(1, ComponentCornerRounding::new(3.0, 3.0, 3.0, 3.0));
    progressbar.set_label_text("Research: Iron Working (3)".to_string());
    progressbar.set_label_horizontal_alignment(lemao_ui::components::HorizontalAlignment::Middle);
    progressbar.set_label_vertical_alignment(lemao_ui::components::VerticalAlignment::Middle);
    progressbar.set_label_offset(Vec2::new(0.0, -1.0));
    progressbar.set_label_shadow_enabled_flag(true);
    progressbar.set_label_shadow_offset(Vec2::new(1.0, -1.0));
    progressbar.set_label_shadow_color(Color::SolidColor(SolidColor::new_rgb(0, 0, 0, 120)));
    ui.get_main_canvas_mut()?.add_child(progressbar_id);

    let mut window_filling_gradient = Gradient::new(GradientType::Radial, Vec2::new(0.3, -0.3));
    window_filling_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(66, 90, 150, 255), 0.0));
    window_filling_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(50, 67, 114, 255), 1.0));

    let mut window_border_gradient = Gradient::new(GradientType::Radial, Vec2::new(0.3, -0.3));
    window_border_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(66, 90, 190, 255), 0.0));
    window_border_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(50, 67, 154, 255), 1.0));

    let mut window_shadow_gradient = Gradient::new(GradientType::Rectangular, Vec2::new(0.0, 0.0));
    window_shadow_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(0, 0, 0, 255), 0.0));
    window_shadow_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(0, 0, 0, 255), 0.99));
    window_shadow_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(0, 0, 0, 0), 1.0));

    let main_window_id = ui.create_panel(&mut renderer, ComponentShape::Rectangle)?;
    let main_window = ui.get_component_with_type_mut::<Panel>(main_window_id)?;
    main_window.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.5, 0.5)));
    main_window.set_size(ComponentSize::Absolute(Vec2::new(690.0, 620.0)));
    main_window.set_anchor(Vec2::new(0.5, 0.5));
    main_window.set_color(Color::Gradient(window_filling_gradient.clone()));
    main_window.set_border_color(Color::Gradient(window_border_gradient.clone()));
    main_window.set_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0))?;
    main_window.set_corner_rounding(ComponentCornerRounding::new(10.0, 10.0, 10.0, 10.0));
    main_window.set_shadow_enabled_flag(true);
    main_window.set_shadow_offset(Vec2::new(5.0, -5.0));
    main_window.set_shadow_color(Color::Gradient(window_shadow_gradient.clone()));
    main_window.set_shadow_corner_rounding(ComponentCornerRounding::new(20.0, 20.0, 20.0, 20.0))?;
    main_window.set_shadow_scale(Vec2::new(1.03, 1.03));
    ui.get_main_canvas_mut()?.add_child(main_window_id);

    let window_title_id = ui.create_label(&mut renderer, header_font_id)?;
    let window_title = ui.get_component_with_type_mut::<Label>(window_title_id)?;
    window_title.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0)));
    window_title.set_anchor(Vec2::new(0.5, 0.5));
    window_title.set_offset(Vec2::new(0.0, -27.0));
    window_title.set_text("You have founded agriculture!".to_string());
    window_title.set_shadow_enabled_flag(true);
    window_title.set_shadow_offset(Vec2::new(1.0, -1.0));
    window_title.set_shadow_color(Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)));
    ui.get_component_mut(main_window_id)?.add_child(window_title_id);

    let image_id = ui.create_panel(&mut renderer, ComponentShape::Rectangle)?;
    let image = ui.get_component_with_type_mut::<Panel>(image_id)?;
    image.set_texture(textures.lock().unwrap().get(texture_id)?);
    image.set_size(ComponentSize::Absolute(Vec2::new(300.0, 150.0)));
    image.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0)));
    image.set_anchor(Vec2::new(0.0, 1.0));
    image.set_offset(Vec2::new(0.0, -50.0));
    image.set_margin(ComponentMargin::new(0.0, 0.0, 10.0, 10.0));
    image.set_border_color(Color::SolidColor(SolidColor::new_rgb(72, 45, 6, 255)));
    image.set_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0))?;
    image.set_corner_rounding(ComponentCornerRounding::new(4.0, 4.0, 4.0, 4.0));
    ui.get_component_mut(main_window_id)?.add_child(image_id);

    let mut quote_panel_gradient = Gradient::new(GradientType::Vertical, Vec2::new(0.0, 0.0));
    quote_panel_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(96, 97, 115, 0), 0.0));
    quote_panel_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(96, 97, 115, 255), 1.0));

    let quote_panel_id = ui.create_panel(&mut renderer, ComponentShape::Rectangle)?;
    let quote_panel = ui.get_component_with_type_mut::<Panel>(quote_panel_id)?;
    quote_panel.set_position(ComponentPosition::RelativeToParent(Vec2::new(1.0, 1.0)));
    quote_panel.set_size(ComponentSize::Absolute(Vec2::new(400.0, 150.0)));
    quote_panel.set_anchor(Vec2::new(1.0, 1.0));
    quote_panel.set_color(Color::SolidColor(SolidColor::new_rgb(60, 60, 60, 0)));
    quote_panel.set_max_size(Vec2::new(f32::MAX, 150.0));
    quote_panel.set_margin(ComponentMargin::new(0.0, 0.0, 10.0, 10.0));
    quote_panel.set_offset(Vec2::new(0.0, -50.0));
    quote_panel.set_corner_rounding(ComponentCornerRounding::new(5.0, 5.0, 5.0, 5.0));
    quote_panel.set_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0))?;
    quote_panel.set_border_color(Color::SolidColor(SolidColor::new_rgb(86, 92, 107, 255)));
    ui.get_component_mut(main_window_id)?.add_child(quote_panel_id);

    let quote_id = ui.create_label(&mut renderer, regular_font_id)?;
    let quote = ui.get_component_with_type_mut::<Label>(quote_id)?;
    quote.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0)));
    quote.set_anchor(Vec2::new(0.0, 1.0));
    quote.set_margin(ComponentMargin::new(10.0, 10.0, 10.0, 10.0));
    quote.set_multiline_text(
        "\"To forget how to dig the earth and to tend the soil is to forget ourselves.\" - °200,255,200,255°Mahatma Gandhi°255,255,255,255°\n\n".to_string() + 
        "\"Agriculture is our wisest pursuit, because it will in the end contribute most to real wealth, good morals, and happiness.\" - °200,255,200,255°Thomas Jefferson°255,255,255,255°",
        360.0,
    );
    quote.set_shadow_enabled_flag(true);
    quote.set_shadow_offset(Vec2::new(1.0, -1.0));
    quote.set_shadow_color(Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 0.5)));
    ui.get_component_mut(quote_panel_id)?.add_child(quote_id);

    let description_panel_id = ui.create_panel(&mut renderer, ComponentShape::Rectangle)?;
    let description_panel = ui.get_component_with_type_mut::<Panel>(description_panel_id)?;
    description_panel.set_position(ComponentPosition::RelativeToParent(Vec2::new(1.0, 1.0)));
    description_panel.set_size(ComponentSize::Relative(Vec2::new(1.0, 150.0)));
    description_panel.set_anchor(Vec2::new(1.0, 1.0));
    description_panel.set_color(Color::SolidColor(SolidColor::new_rgb(60, 60, 60, 0)));
    description_panel.set_max_size(Vec2::new(f32::MAX, 220.0));
    description_panel.set_margin(ComponentMargin::new(0.0, 0.0, 10.0, 10.0));
    description_panel.set_offset(Vec2::new(0.0, -210.0));
    description_panel.set_corner_rounding(ComponentCornerRounding::new(5.0, 8.0, 8.0, 5.0));
    description_panel.set_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0))?;
    description_panel.set_border_color(Color::SolidColor(SolidColor::new_rgb(86, 92, 107, 255)));
    ui.get_component_mut(main_window_id)?.add_child(description_panel_id);

    let mut scroll_gradient = Gradient::new(GradientType::Horizontal, Vec2::new(0.0, 0.0));
    scroll_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(213, 195, 168, 255), 0.0));
    scroll_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(167, 148, 134, 255), 1.0));

    let description_scrollbox_id = ui.create_scrollbox(&mut renderer)?;
    let description_scrollbox = ui.get_component_with_type_mut::<Scrollbox>(description_scrollbox_id)?;
    description_scrollbox.set_size(ComponentSize::Relative(Vec2::new(1.0, 1.0)));
    description_scrollbox.set_vertical_scroll_background_color(Color::SolidColor(SolidColor::new_rgb(0, 0, 0, 120)));
    description_scrollbox.set_vertical_scroll_color(Color::Gradient(scroll_gradient.clone()));
    description_scrollbox.set_vertical_scroll_corner_rounding(ComponentCornerRounding::new(8.0, 8.0, 8.0, 8.0))?;
    description_scrollbox.set_vertical_scroll_background_corner_rounding(ComponentCornerRounding::new(8.0, 8.0, 8.0, 8.0))?;
    description_scrollbox.set_vertical_scroll_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0));
    description_scrollbox.set_vertical_scroll_border_color(Color::SolidColor(SolidColor::new_rgb(199, 178, 153, 255)));
    description_scrollbox.set_vertical_scroll_background_border_color(Color::SolidColor(SolidColor::new_rgb(199, 178, 153, 255)));
    description_scrollbox.set_vertical_scroll_background_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0));

    description_scrollbox.set_horizontal_scroll_background_color(Color::SolidColor(SolidColor::new_rgb(0, 0, 0, 120)));
    description_scrollbox.set_horizontal_scroll_color(Color::Gradient(scroll_gradient.clone()));
    description_scrollbox.set_horizontal_scroll_corner_rounding(ComponentCornerRounding::new(8.0, 8.0, 8.0, 8.0))?;
    description_scrollbox.set_horizontal_scroll_background_corner_rounding(ComponentCornerRounding::new(8.0, 8.0, 8.0, 8.0))?;
    description_scrollbox.set_horizontal_scroll_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0));
    description_scrollbox.set_horizontal_scroll_border_color(Color::SolidColor(SolidColor::new_rgb(199, 178, 153, 255)));
    description_scrollbox.set_horizontal_scroll_background_border_color(Color::SolidColor(SolidColor::new_rgb(199, 178, 153, 255)));
    description_scrollbox.set_horizontal_scroll_background_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0));
    description_scrollbox.set_padding(Vec2::new(20.0, 20.0));
    description_scrollbox.set_scroll_width(Vec2::new(20.0, 0.0));
    ui.get_component_mut(description_panel_id)?.add_child(description_scrollbox_id);

    let description_id = ui.create_label(&mut renderer, regular_font_id)?;
    let description = ui.get_component_with_type_mut::<Label>(description_id)?;
    description.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0)));
    description.set_anchor(Vec2::new(0.0, 1.0));
    description.set_margin(ComponentMargin::new(10.0, 10.0, 10.0, 10.0));
    description.set_multiline_text(
        "Agriculture began independently in different parts of the globe, and included a diverse range of taxa. At least eleven separate regions of the Old and New World were involved as independent centers of origin. The development of agriculture about 12,000 years ago changed the way humans lived. They switched from nomadic hunter-gatherer lifestyles to permanent settlements and farming.\n\n".to_string() + 
        "Wild grains were collected and eaten from at least 105,000 years ago. However, domestication did not occur until much later. The earliest evidence of small-scale cultivation of edible grasses is from around 21,000 BC with the Ohalo II people on the shores of the Sea of Galilee. By around 9500 BC, the eight Neolithic founder crops - emmer wheat, einkorn wheat, hulled barley, peas, lentils, bitter vetch, chickpeas, and flax - were cultivated in the Levant. Rye may have been cultivated earlier, but this claim remains controversial. Rice was domesticated in China by 6200 BC with earliest known cultivation from 5700 BC, followed by mung, soy and azuki beans. Rice was also independently domesticated in West Africa and cultivated by 1000 BC. Pigs were domesticated in Mesopotamia around 11,000 years ago, followed by sheep. Cattle were domesticated from the wild aurochs in the areas of modern Turkey and India around 8500 BC. Camels were domesticated late, perhaps around 3000 BC.\n\n" +
        "In subsaharan Africa, sorghum was domesticated in the Sahel region of Africa by 3000 BC, along with pearl millet by 2000 BC. Yams were domesticated in several distinct locations, including West Africa (unknown date), and cowpeas by 2500 BC. Rice (African rice) was also independently domesticated in West Africa and cultivated by 1000 BC. Teff and likely finger millet were domesticated in Ethiopia by 3000 BC, along with noog, ensete, and coffee. Other plant foods domesticated in Africa include watermelon, okra, tamarind and black eyed peas, along with tree crops such as the kola nut and oil palm. Plantains were cultivated in Africa by 3000 BC and bananas by 1500 BC. The helmeted guineafowl was domesticated in West Africa. Sanga cattle was likely also domesticated in North-East Africa, around 7000 BC, and later crossbred with other species.\n\n" +
        "In South America, agriculture began as early as 9000 BC, starting with the cultivation of several species of plants that later became only minor crops. In the Andes of South America, the potato was domesticated between 8000 BC and 5000 BC, along with beans, squash, tomatoes, peanuts, coca, llamas, alpacas, and guinea pigs. Cassava was domesticated in the Amazon Basin no later than 7000 BC. Maize (Zea mays) found its way to South America from Mesoamerica, where wild teosinte was domesticated about 7000 BC and selectively bred to become domestic maize. Cotton was domesticated in Peru by 4200 BC; another species of cotton was domesticated in Mesoamerica and became by far the most important species of cotton in the textile industry in modern times. Evidence of agriculture in the Eastern United States dates to about 3000 BCE. Several plants were cultivated, later to be replaced by the Three Sisters cultivation of maize, squash, and beans.",
        630.0,
    );
    description.set_shadow_enabled_flag(true);
    description.set_shadow_offset(Vec2::new(1.0, -1.0));
    description.set_shadow_color(Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 0.5)));
    ui.get_component_mut(description_scrollbox_id)?.add_child(description_id);

    let mut effect_panel_1_gradient = Gradient::new(GradientType::Vertical, Vec2::new(0.0, 0.0));
    effect_panel_1_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(96, 97, 115, 0), 0.0));
    effect_panel_1_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(96, 97, 115, 255), 1.0));

    let effect_panel_1_id = ui.create_panel(&mut renderer, ComponentShape::Rectangle)?;
    let effect_panel_1 = ui.get_component_with_type_mut::<Panel>(effect_panel_1_id)?;
    effect_panel_1.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.5, 0.0)));
    effect_panel_1.set_size(ComponentSize::Relative(Vec2::new(1.0, 1.0)));
    effect_panel_1.set_anchor(Vec2::new(0.5, 0.0));
    effect_panel_1.set_color(Color::Gradient(effect_panel_1_gradient));
    effect_panel_1.set_max_size(Vec2::new(f32::MAX, 120.0));
    effect_panel_1.set_margin(ComponentMargin::new(0.0, 0.0, 10.0, 10.0));
    effect_panel_1.set_offset(Vec2::new(0.0, 60.0));
    effect_panel_1.set_corner_rounding(ComponentCornerRounding::new(5.0, 5.0, 5.0, 5.0));
    ui.get_component_mut(main_window_id)?.add_child(effect_panel_1_id);

    let effect_panel_2_id = ui.create_panel(&mut renderer, ComponentShape::Rectangle)?;
    let effect_panel_2 = ui.get_component_with_type_mut::<Panel>(effect_panel_2_id)?;
    effect_panel_2.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.5, 0.0)));
    effect_panel_2.set_size(ComponentSize::Relative(Vec2::new(1.0, 1.0)));
    effect_panel_2.set_anchor(Vec2::new(0.5, 0.0));
    effect_panel_2.set_color(Color::SolidColor(SolidColor::new_rgb(0, 0, 0, 60)));
    effect_panel_2.set_max_size(Vec2::new(f32::MAX, 120.0));
    effect_panel_2.set_margin(ComponentMargin::new(10.0, 0.0, 20.0, 20.0));
    effect_panel_2.set_offset(Vec2::new(0.0, 60.0));
    effect_panel_2.set_corner_rounding(ComponentCornerRounding::new(5.0, 5.0, 5.0, 5.0));
    ui.get_component_mut(main_window_id)?.add_child(effect_panel_2_id);

    let effect_id = ui.create_label(&mut renderer, regular_font_id)?;
    let effect = ui.get_component_with_type_mut::<Label>(effect_id)?;
    effect.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0)));
    effect.set_anchor(Vec2::new(0.0, 1.0));
    effect.set_margin(ComponentMargin::new(3.0, 3.0, 3.0, 3.0));
    effect.set_text(
        "+1 \u{CA} and +1 \u{C9} per military unit stationed in the city\n".to_string()
            + "+10% \u{CA} for every farm belonging to capital (but not more than 50%)\n"
            + "+20% \u{C8} for every specialist\n"
            + "+1 \u{C8} worker mainteance",
    );
    effect.set_shadow_enabled_flag(true);
    effect.set_shadow_offset(Vec2::new(1.0, -1.0));
    effect.set_shadow_color(Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 0.5)));
    ui.get_component_mut(effect_panel_2_id)?.add_child(effect_id);

    let mut button_filling_gradient = Gradient::new(GradientType::Vertical, Vec2::new(0.0, 0.0));
    button_filling_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(159, 148, 135, 255), 0.0));
    button_filling_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(234, 221, 198, 255), 1.0));

    let ok_button_id = ui.create_button(&mut renderer, ComponentShape::Rectangle, bold_font_id)?;
    let ok_button = ui.get_component_with_type_mut::<Button>(ok_button_id)?;
    ok_button.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.5, 0.0)));
    ok_button.set_size(ComponentSize::Absolute(Vec2::new(100.0, 25.0)));
    ok_button.set_anchor(Vec2::new(0.5, 0.5));
    ok_button.set_color(Color::Gradient(button_filling_gradient.clone()));
    ok_button.set_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0))?;
    ok_button.set_border_color(Color::SolidColor(SolidColor::new(0.5, 0.5, 0.5, 1.0)));
    ok_button.set_label_text("Ok".to_string());
    ok_button.set_margin(ComponentMargin::new(0.0, 0.0, 0.0, 0.0));
    ok_button.set_label_horizontal_alignment(lemao_ui::components::HorizontalAlignment::Middle);
    ok_button.set_label_vertical_alignment(lemao_ui::components::VerticalAlignment::Middle);
    ok_button.set_label_offset(Vec2::new(0.0, 0.0));
    ok_button.set_label_color(Color::SolidColor(SolidColor::new_rgb(117, 95, 72, 255)));
    ok_button.set_offset(Vec2::new(0.0, 30.0));
    ok_button.set_corner_rounding(ComponentCornerRounding::new(5.0, 5.0, 5.0, 5.0));
    ok_button.set_shadow_enabled_flag(true);
    ok_button.set_shadow_offset(Vec2::new(2.0, -2.0));
    ok_button.set_shadow_color(Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)));
    ok_button.set_shadow_corner_rounding(ComponentCornerRounding::new(5.0, 5.0, 5.0, 5.0))?;
    ok_button.set_shadow_scale(Vec2::new(1.01, 1.01));
    ok_button.on_button_pressed = Some(|button, _, _| {
        button.set_color(button.get_color().clone().set_alpha(0.8));
        button.set_border_color(button.get_border_color().clone().set_alpha(0.8));
        button.set_shadow_color(button.get_shadow_color().clone().set_alpha(0.4));
    });
    ok_button.on_button_released = Some(|button, _, _| {
        button.set_color(button.get_color().clone().set_alpha(1.0));
        button.set_border_color(button.get_border_color().clone().set_alpha(1.0));
        button.set_shadow_color(button.get_shadow_color().clone().set_alpha(1.0));
    });
    ui.get_component_mut(main_window_id)?.add_child(ok_button_id);

    let right_window_id = ui.create_panel(&mut renderer, ComponentShape::Rectangle)?;
    let right_window = ui.get_component_with_type_mut::<Panel>(right_window_id)?;
    right_window.set_position(ComponentPosition::RelativeToParent(Vec2::new(1.0, 0.5)));
    right_window.set_offset(Vec2::new(-15.0, 0.0));
    right_window.set_size(ComponentSize::Absolute(Vec2::new(300.0, 620.0)));
    right_window.set_anchor(Vec2::new(1.0, 0.5));
    right_window.set_color(Color::Gradient(window_filling_gradient.clone()));
    right_window.set_border_color(Color::Gradient(window_border_gradient.clone()));
    right_window.set_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0))?;
    right_window.set_corner_rounding(ComponentCornerRounding::new(10.0, 10.0, 10.0, 10.0));
    right_window.set_shadow_enabled_flag(true);
    right_window.set_shadow_offset(Vec2::new(5.0, -5.0));
    right_window.set_shadow_color(Color::Gradient(window_shadow_gradient.clone()));
    right_window.set_shadow_corner_rounding(ComponentCornerRounding::new(20.0, 20.0, 20.0, 20.0))?;
    right_window.set_shadow_scale(Vec2::new(1.03, 1.03));
    ui.get_main_canvas_mut()?.add_child(right_window_id);

    let right_window_title_id = ui.create_label(&mut renderer, header_font_id)?;
    let right_window_title = ui.get_component_with_type_mut::<Label>(right_window_title_id)?;
    right_window_title.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0)));
    right_window_title.set_anchor(Vec2::new(0.5, 0.5));
    right_window_title.set_offset(Vec2::new(0.0, -27.0));
    right_window_title.set_text("Settings".to_string());
    right_window_title.set_shadow_enabled_flag(true);
    right_window_title.set_shadow_offset(Vec2::new(1.0, -1.0));
    right_window_title.set_shadow_color(Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)));
    ui.get_component_mut(right_window_id)?.add_child(right_window_title_id);

    let mut checkbox_ids = Vec::new();
    let checkbox_labels = [
        "No city razing",
        "No city flipping from culture",
        "City flipping after conquest",
        "No barbarians",
        "Raging barbarians",
        "Aggresive AI",
        "No technology trading",
        "Always war",
        "Always peace",
    ];

    for checkbox_label in checkbox_labels {
        let checkbox_id = ui.create_checkbox(&mut renderer, regular_font_id, box_checked_id, box_unchecked_id)?;
        let checkbox = ui.get_component_with_type_mut::<Checkbox>(checkbox_id)?;
        checkbox.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0)));
        checkbox.set_offset(Vec2::new(0.0, -75.0 - 30.0 * checkbox_ids.len() as f32));
        checkbox.set_margin(ComponentMargin::new(3.0, 3.0, 3.0, 10.0));
        checkbox.set_label_offset(Vec2::new(25.0, 1.0));
        checkbox.set_box_offset(Vec2::new(0.0, 4.0));
        checkbox.set_label_text(checkbox_label.to_string());
        checkbox.set_label_shadow_enabled_flag(true);
        checkbox.set_label_shadow_offset(Vec2::new(1.0, -1.0));
        checkbox.set_label_shadow_color(Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)));
        checkbox.on_cursor_enter = Some(|checkbox, _| checkbox.set_box_color(checkbox.get_box_color().clone().set_alpha(0.8)));
        checkbox.on_cursor_leave = Some(|checkbox, _| checkbox.set_box_color(checkbox.get_box_color().clone().set_alpha(1.0)));
        ui.get_component_mut(right_window_id)?.add_child(checkbox_id);

        checkbox_ids.push(checkbox_id);
    }

    let mut textbox_ids = Vec::new();
    let mut textbox_label_ids = Vec::new();
    let textbox_labels = ["Player name:", "Empire name:", "World name:"];

    for textbox_label in textbox_labels {
        let label_id = ui.create_label(&mut renderer, regular_font_id)?;
        let label = ui.get_component_with_type_mut::<Label>(label_id)?;
        label.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0)));
        label.set_offset(Vec2::new(0.0, -348.0 - 30.0 * textbox_label_ids.len() as f32));
        label.set_margin(ComponentMargin::new(3.0, 3.0, 3.0, 10.0));
        label.set_text(textbox_label.to_string());
        label.set_shadow_enabled_flag(true);
        label.set_shadow_offset(Vec2::new(1.0, -1.0));
        label.set_shadow_color(Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)));
        ui.get_component_mut(right_window_id)?.add_child(label_id);

        textbox_label_ids.push(label_id);

        let textbox_id = ui.create_textbox(&mut renderer, regular_font_id)?;
        let textbox = ui.get_component_with_type_mut::<TextBox>(textbox_id)?;
        textbox.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0)));
        textbox.set_size(ComponentSize::Absolute(Vec2::new(180.0, 30.0)));
        textbox.set_offset(Vec2::new(110.0, -350.0 - 30.0 * textbox_ids.len() as f32));
        textbox.set_margin(ComponentMargin::new(3.0, 3.0, 3.0, 10.0));
        textbox.set_label_horizontal_alignment(HorizontalAlignment::Left);
        textbox.set_label_offset(Vec2::new(3.0, -1.0));
        textbox.set_color(Color::SolidColor(SolidColor::new_rgb(38, 41, 52, 255)));
        textbox.set_border_color(Color::SolidColor(SolidColor::new_rgb(199, 178, 153, 255)));
        textbox.set_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0))?;
        textbox.set_corner_rounding(ComponentCornerRounding::new(3.0, 3.0, 3.0, 3.0));
        textbox.set_label_max_length(15);
        textbox.on_cursor_enter = Some(|textbox, _| {
            if !textbox.is_active() {
                textbox.set_color(textbox.get_color().clone().set_alpha(0.8));
            }
        });
        textbox.on_cursor_leave = Some(|textbox, _| {
            if !textbox.is_active() {
                textbox.set_color(textbox.get_color().clone().set_alpha(1.0));
            }
        });
        textbox.on_activation = Some(|textbox, _| textbox.set_color(textbox.get_color().clone().set_alpha(0.5)));
        textbox.on_deactivation = Some(|textbox, _| textbox.set_color(textbox.get_color().clone().set_alpha(1.0)));
        ui.get_component_mut(right_window_id)?.add_child(textbox_id);

        textbox_ids.push(textbox_id);
    }

    let right_window_focuses_title_id = ui.create_label(&mut renderer, header_font_id)?;
    let right_window_focuses = ui.get_component_with_type_mut::<Label>(right_window_focuses_title_id)?;
    right_window_focuses.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0)));
    right_window_focuses.set_anchor(Vec2::new(0.5, 0.5));
    right_window_focuses.set_offset(Vec2::new(0.0, -445.0));
    right_window_focuses.set_text("City focuses".to_string());
    right_window_focuses.set_shadow_enabled_flag(true);
    right_window_focuses.set_shadow_offset(Vec2::new(1.0, -1.0));
    right_window_focuses.set_shadow_color(Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)));
    ui.get_component_mut(right_window_id)?.add_child(right_window_focuses_title_id);

    let mut toggle_button_ids = Vec::new();
    let toggle_button_labels = ["\u{C8}", "\u{C9}", "\u{CA}"];

    for toggle_button_label in toggle_button_labels {
        let button_id = ui.create_button(&mut renderer, ComponentShape::Rectangle, regular_font_id)?;
        let button = ui.get_component_with_type_mut::<Button>(button_id)?;
        button.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0)));
        button.set_size(ComponentSize::Absolute(Vec2::new(40.0, 30.0)));
        button.set_offset(Vec2::new(100.0 + 50.0 * toggle_button_ids.len() as f32, -480.0));
        button.set_anchor(Vec2::new(0.5, 0.5));
        button.set_color(Color::Gradient(button_filling_gradient.clone()));
        button.set_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0))?;
        button.set_border_color(Color::SolidColor(SolidColor::new(0.5, 0.5, 0.5, 1.0)));
        button.set_label_text(toggle_button_label.to_string());
        button.set_margin(ComponentMargin::new(0.0, 0.0, 0.0, 0.0));
        button.set_label_horizontal_alignment(lemao_ui::components::HorizontalAlignment::Middle);
        button.set_label_vertical_alignment(lemao_ui::components::VerticalAlignment::Middle);
        button.set_label_offset(Vec2::new(0.0, -2.0));
        button.set_corner_rounding(ComponentCornerRounding::new(5.0, 5.0, 5.0, 5.0));
        button.set_shadow_enabled_flag(true);
        button.set_shadow_offset(Vec2::new(2.0, -2.0));
        button.set_shadow_color(Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)));
        button.set_shadow_corner_rounding(ComponentCornerRounding::new(5.0, 5.0, 5.0, 5.0))?;
        button.set_shadow_scale(Vec2::new(1.01, 1.01));
        button.set_toggleable_flag(true);
        button.on_button_pressed = Some(|button, _, _| {
            button.set_color(button.get_color().clone().set_alpha(0.4));
            button.set_border_color(button.get_border_color().clone().set_alpha(0.4));
            button.set_shadow_color(button.get_shadow_color().clone().set_alpha(0.0));
            button.set_label_color(button.get_label_color().clone().set_alpha(0.2));
        });
        button.on_button_released = Some(|button, _, _| {
            button.set_color(button.get_color().clone().set_alpha(1.0));
            button.set_border_color(button.get_border_color().clone().set_alpha(1.0));
            button.set_shadow_color(button.get_shadow_color().clone().set_alpha(1.0));
            button.set_label_color(button.get_label_color().clone().set_alpha(1.0));
        });
        ui.get_component_mut(right_window_id)?.add_child(button_id);

        toggle_button_ids.push(button_id);
    }

    let mut selector_filling_gradient = Gradient::new(GradientType::Vertical, Vec2::new(0.0, 0.0));
    selector_filling_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(159, 148, 135, 255), 0.0));
    selector_filling_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(234, 221, 198, 255), 1.0));

    let mut slider_ids = Vec::new();
    let mut slider_label_ids = Vec::new();
    let slider_labels = ["\u{C8} (50%):", "\u{C9} (50%):", "\u{CA} (50%):"];
    let slider_steps_count = [5, 11, u32::MAX];

    for slider_label in slider_labels {
        let label_id = ui.create_label(&mut renderer, regular_font_id)?;
        let label = ui.get_component_with_type_mut::<Label>(label_id)?;
        label.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0)));
        label.set_anchor(Vec2::new(0.0, 0.5));
        label.set_offset(Vec2::new(0.0, -530.0 - 30.0 * slider_label_ids.len() as f32));
        label.set_margin(ComponentMargin::new(3.0, 3.0, 3.0, 10.0));
        label.set_text(slider_label.to_string());
        label.set_shadow_enabled_flag(true);
        label.set_shadow_offset(Vec2::new(1.0, -1.0));
        label.set_shadow_color(Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)));
        ui.get_component_mut(right_window_id)?.add_child(label_id);

        slider_label_ids.push(label_id);

        let slider_id = ui.create_slider(&mut renderer, ComponentShape::Disc)?;
        let slider = ui.get_component_with_type_mut::<Slider>(slider_id)?;
        slider.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0)));
        slider.set_size(ComponentSize::Absolute(Vec2::new(210.0, 10.0)));
        slider.set_anchor(Vec2::new(0.5, 0.5));
        slider.set_offset(Vec2::new(35.0, -530.0 - 30.0 * slider_ids.len() as f32));
        slider.set_margin(ComponentMargin::new(3.0, 3.0, 3.0, 10.0));
        slider.set_color(Color::SolidColor(SolidColor::new_rgb(38, 41, 52, 255)));
        slider.set_bar_color(Color::SolidColor(SolidColor::new_rgb(219, 198, 173, 255)));
        slider.set_phase(0.5);
        slider.set_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0))?;
        slider.set_border_color(Color::SolidColor(SolidColor::new_rgb(199, 178, 153, 255)));
        slider.set_corner_rounding(ComponentCornerRounding::new(1.0, 1.0, 1.0, 1.0));
        slider.set_selector_size(Vec2::new(15.0, 15.0));
        slider.set_selector_color(Color::Gradient(selector_filling_gradient.clone()));
        slider.set_steps_count(slider_steps_count[slider_ids.len()]);
        slider.set_selector_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0));
        slider.set_selector_border_color(Color::SolidColor(SolidColor::new_rgb(199, 178, 153, 255)));
        ui.get_component_mut(right_window_id)?.add_child(slider_id);

        slider_ids.push(slider_id);
    }

    let left_window_id = ui.create_panel(&mut renderer, ComponentShape::Rectangle)?;
    let left_window = ui.get_component_with_type_mut::<Panel>(left_window_id)?;
    left_window.set_position(ComponentPosition::RelativeToParent(Vec2::new(0.0, 0.5)));
    left_window.set_offset(Vec2::new(15.0, 0.0));
    left_window.set_size(ComponentSize::Absolute(Vec2::new(300.0, 620.0)));
    left_window.set_anchor(Vec2::new(0.0, 0.5));
    left_window.set_color(Color::Gradient(window_filling_gradient.clone()));
    left_window.set_border_color(Color::Gradient(window_border_gradient.clone()));
    left_window.set_border_thickness(ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0))?;
    left_window.set_corner_rounding(ComponentCornerRounding::new(10.0, 10.0, 10.0, 10.0));
    left_window.set_shadow_enabled_flag(true);
    left_window.set_shadow_offset(Vec2::new(5.0, -5.0));
    left_window.set_shadow_color(Color::Gradient(window_shadow_gradient.clone()));
    left_window.set_shadow_corner_rounding(ComponentCornerRounding::new(20.0, 20.0, 20.0, 20.0))?;
    left_window.set_shadow_scale(Vec2::new(1.03, 1.03));
    ui.get_main_canvas_mut()?.add_child(left_window_id);

    let mut is_running = true;

    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::WindowSizeChanged(size) => {
                    renderer.set_viewport_size(size);
                    renderer.get_active_camera_mut()?.set_size(size);
                }

                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }

            ui.process_window_event(&mut renderer, &event)?;
        }

        while let Some(event) = ui.poll_event() {
            match event {
                UiEvent::CursorEnter(component_id, cursor_position) => println!("ENTER {} {:?}", component_id, cursor_position),
                UiEvent::CursorLeave(component_id, cursor_position) => println!("LEAVE {} {:?}", component_id, cursor_position),
                UiEvent::MouseButtonPressed(component_id, button) => println!("PRESSED {} {:?}", component_id, button),
                UiEvent::MouseButtonReleased(component_id, button) => println!("RELEASED {} {:?}", component_id, button),

                UiEvent::ButtonPressed(component_id, button) => println!("BUTTON PRESSED {} {:?}", component_id, button),
                UiEvent::ButtonReleased(component_id, button) => println!("BUTTON RELEASED {} {:?}", component_id, button),
                UiEvent::ButtonClicked(component_id, button) => println!("BUTTON CLICKED {} {:?}", component_id, button),

                UiEvent::CheckboxChecked(component_id, button) => println!("CHECKBOX CHECKED {} {:?}", component_id, button),
                UiEvent::CheckboxUnchecked(component_id, button) => println!("CHECKBOX UNCHECKED {} {:?}", component_id, button),
                UiEvent::CheckboxChanged(component_id, button, checked) => println!("CHECKBOX CHANGED {} {:?} {}", component_id, button, checked),

                UiEvent::ScrollCursorEnter(component_id, cursor_position) => println!("SCROLL ENTER {} {:?}", component_id, cursor_position),
                UiEvent::ScrollCursorLeave(component_id, cursor_position) => println!("SCROLL LEAVE {} {:?}", component_id, cursor_position),
                UiEvent::ScrollMouseButtonPressed(component_id, button) => println!("SCROLL PRESSED {} {:?}", component_id, button),
                UiEvent::ScrollMouseButtonReleased(component_id, button) => println!("SCROLL RELEASED {} {:?}", component_id, button),
                UiEvent::ScrollMoved(component_id, direction) => println!("SCROLL {} {:?}", component_id, direction),

                UiEvent::SelectorCursorEnter(component_id, cursor_position) => println!("SELECTOR ENTER {} {:?}", component_id, cursor_position),
                UiEvent::SelectorCursorLeave(component_id, cursor_position) => println!("SELECTOR LEAVE {} {:?}", component_id, cursor_position),
                UiEvent::SelectorMouseButtonPressed(component_id, button) => println!("SELECTOR PRESSED {} {:?}", component_id, button),
                UiEvent::SelectorMouseButtonReleased(component_id, button) => println!("SELECTOR RELEASED {} {:?}", component_id, button),
                UiEvent::SelectorMoved(component_id, direction) => {
                    println!("SELECTOR {} {:?}", component_id, direction);

                    let phase = ui.get_component_with_type_mut::<Slider>(component_id)?.get_phase();
                    let label_index = slider_ids.iter().position(|&p| p == component_id).unwrap();
                    let label_content = slider_labels[label_index].replace("50%", &format!("{:.0}%", phase * 100.0));

                    ui.get_component_with_type_mut::<Label>(slider_label_ids[label_index])?.set_text(label_content);
                }
                UiEvent::TextBoxActivated(component_id, button) => println!("TEXTBOX ACTIVATED {} {:?}", component_id, button),
                UiEvent::TextBoxDeactivated(component_id, button) => println!("TEXTBOX DEACTIVATED {} {:?}", component_id, button),
                UiEvent::TextBoxContentChanged(component_id, c) => println!("TEXTBOX CHANGED {} {}", component_id, c),
            }
        }

        ui.update(&mut renderer)?;

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        ui.draw(&mut renderer, progressbar_id)?;
        ui.draw(&mut renderer, main_window_id)?;
        ui.draw(&mut renderer, quote_panel_id)?;
        ui.draw(&mut renderer, quote_id)?;
        ui.draw(&mut renderer, description_panel_id)?;

        ui.draw(&mut renderer, description_scrollbox_id)?;
        ui.begin_scrollbox(description_scrollbox_id, &renderer)?;
        ui.draw(&mut renderer, description_id)?;
        ui.end_scrollbox(&renderer);

        ui.draw(&mut renderer, effect_panel_1_id)?;
        ui.draw(&mut renderer, effect_panel_2_id)?;
        ui.draw(&mut renderer, image_id)?;
        ui.draw(&mut renderer, ok_button_id)?;
        ui.draw(&mut renderer, window_title_id)?;
        ui.draw(&mut renderer, effect_id)?;

        ui.draw(&mut renderer, right_window_id)?;
        ui.draw(&mut renderer, right_window_title_id)?;

        for checkbox_id in &checkbox_ids {
            ui.draw(&mut renderer, *checkbox_id)?;
        }

        for textbox_label_id in &textbox_label_ids {
            ui.draw(&mut renderer, *textbox_label_id)?;
        }

        for textbox_id in &textbox_ids {
            ui.draw(&mut renderer, *textbox_id)?;
        }

        ui.draw(&mut renderer, right_window_focuses_title_id)?;

        for button_id in &toggle_button_ids {
            ui.draw(&mut renderer, *button_id)?;
        }

        for slider_label_id in &slider_label_ids {
            ui.draw(&mut renderer, *slider_label_id)?;
        }

        for slider_id in &slider_ids {
            ui.draw(&mut renderer, *slider_id)?;
        }

        ui.draw(&mut renderer, left_window_id)?;

        window.swap_buffers();
    }

    Ok(())
}

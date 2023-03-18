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
use lemao_core::renderer::fonts::Font;
use lemao_core::renderer::textures::bmp;
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
use lemao_ui::components::wire::Wire;
use lemao_ui::components::wire::WireChunkData;
use lemao_ui::components::ComponentBorderThickness;
use lemao_ui::components::ComponentCornerRounding;
use lemao_ui::components::ComponentMargin;
use lemao_ui::components::ComponentPosition;
use lemao_ui::components::ComponentShape;
use lemao_ui::components::ComponentSize;
use lemao_ui::components::HorizontalAlignment;
use lemao_ui::components::VerticalAlignment;
use lemao_ui::context::UiContext;
use lemao_ui::events::UiEvent;

pub fn main() -> Result<(), String> {
    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = match WindowContext::new("UI", WindowStyle::Window { position: window_position, size: window_size }) {
        Ok(window) => window,
        Err(message) => panic!("{}", message),
    };

    let mut renderer = match window.create_renderer() {
        Ok(renderer) => renderer,
        Err(message) => panic!("{}", message),
    };
    renderer.set_swap_interval(0);

    let coin_icon = bmp::load("./assets/coin.bmp")?;
    let hammer_icon = bmp::load("./assets/hammer.bmp")?;
    let happiness_icon = bmp::load("./assets/happiness.bmp")?;
    let blank_icon = bmp::load("./assets/blank.bmp")?;
    let header_font = bff::load("./assets/header.bff")?;
    let mut regular_font = bff::load("./assets/regular.bff")?;
    let mut bold_font = bff::load("./assets/bold.bff")?;

    regular_font.set_character(200, Vec2::new(0.0, 4.0), &coin_icon);
    regular_font.set_character(201, Vec2::new(0.0, 3.0), &hammer_icon);
    regular_font.set_character(202, Vec2::new(0.0, 3.0), &happiness_icon);
    regular_font.set_character(203, Vec2::new(0.0, 3.0), &blank_icon);

    bold_font.set_character(200, Vec2::new(0.0, 4.0), &coin_icon);
    bold_font.set_character(201, Vec2::new(0.0, 3.0), &hammer_icon);
    bold_font.set_character(202, Vec2::new(0.0, 3.0), &happiness_icon);

    let regular_font_id = renderer.fonts.store(Font::new(&renderer, &regular_font)?);
    let bold_font_id = renderer.fonts.store(Font::new(&renderer, &bold_font)?);
    let header_font_id = renderer.fonts.store(Font::new(&renderer, &header_font)?);

    let texture_id = renderer.textures.store(Texture::new(&renderer, &bmp::load("./assets/wheat.bmp")?)?);
    let box_checked_id = renderer.textures.store(Texture::new(&renderer, &bmp::load("./assets/box_checked.bmp")?)?);
    let box_unchecked_id = renderer.textures.store(Texture::new(&renderer, &bmp::load("./assets/box_unchecked.bmp")?)?);

    let mut ui = UiContext::new(&mut renderer)?;
    // ui.set_debug_flag(true);

    /* #region Progress bar */
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
    let progressbar = ui.get_component_and_cast_mut::<ProgressBar>(progressbar_id)?;
    progressbar.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0));
    progressbar.size = ComponentSize::Relative(Vec2::new(0.6, 1.0));
    progressbar.max_size = Vec2::new(f32::MAX, 20.0);
    progressbar.anchor = Vec2::new(0.5, 1.0);
    progressbar.offset = Vec2::new(0.0, -10.0);
    progressbar.color = Color::Gradient(progressbar_background_gradient);
    progressbar.border_color = Color::SolidColor(SolidColor::new_rgb(72, 79, 92, 255));
    progressbar.border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
    progressbar.corner_rounding = ComponentCornerRounding::new(3.0, 3.0, 3.0, 3.0);
    progressbar.bars[0].visible = true;
    progressbar.bars[0].from = 0.0;
    progressbar.bars[0].to = 0.45;
    progressbar.bars[0].color = Color::Gradient(bar_1_gradient.clone());
    progressbar.bars[0].corner_rounding = ComponentCornerRounding::new(3.0, 3.0, 3.0, 3.0);
    progressbar.bars[1].visible = true;
    progressbar.bars[1].from = 0.0;
    progressbar.bars[1].to = 0.55;
    progressbar.bars[1].color = Color::Gradient(bar_2_gradient.clone());
    progressbar.bars[1].corner_rounding = ComponentCornerRounding::new(3.0, 3.0, 3.0, 3.0);
    progressbar.label_text = "Research: Iron Working (3)".to_string();
    progressbar.label_horizontal_alignment = lemao_ui::components::HorizontalAlignment::Middle;
    progressbar.label_vertical_alignment = lemao_ui::components::VerticalAlignment::Middle;
    progressbar.label_offset = Vec2::new(0.0, -1.0);
    progressbar.label_shadow_enabled = true;
    progressbar.label_shadow_offset = Vec2::new(1.0, -1.0);
    progressbar.label_shadow_color = Color::SolidColor(SolidColor::new_rgb(0, 0, 0, 120));
    ui.get_component_mut(ui.main_canvas_id)?.add_child(progressbar_id);
    /* #endregion */

    /* #region Main window */
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
    let main_window = ui.get_component_and_cast_mut::<Panel>(main_window_id)?;
    main_window.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 0.5));
    main_window.size = ComponentSize::Absolute(Vec2::new(690.0, 620.0));
    main_window.anchor = Vec2::new(0.5, 0.5);
    main_window.color = Color::Gradient(window_filling_gradient.clone());
    main_window.border_color = Color::Gradient(window_border_gradient.clone());
    main_window.border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
    main_window.corner_rounding = ComponentCornerRounding::new(10.0, 10.0, 10.0, 10.0);
    main_window.shadow_enabled = true;
    main_window.shadow_offset = Vec2::new(5.0, -5.0);
    main_window.shadow_color = Color::Gradient(window_shadow_gradient.clone());
    main_window.shadow_corner_rounding = ComponentCornerRounding::new(20.0, 20.0, 20.0, 20.0);
    main_window.shadow_scale = Vec2::new(1.03, 1.03);
    ui.get_component_mut(ui.main_canvas_id)?.add_child(main_window_id);

    let window_title_id = ui.create_label(&mut renderer, header_font_id)?;
    let window_title = ui.get_component_and_cast_mut::<Label>(window_title_id)?;
    window_title.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0));
    window_title.anchor = Vec2::new(0.5, 0.5);
    window_title.offset = Vec2::new(0.0, -27.0);
    window_title.label_text = "You have founded agriculture!".to_string();
    window_title.shadow_enabled = true;
    window_title.shadow_offset = Vec2::new(1.0, -1.0);
    window_title.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
    ui.get_component_mut(main_window_id)?.add_child(window_title_id);

    let image_id = ui.create_panel(&mut renderer, ComponentShape::Rectangle)?;
    let image = ui.get_component_and_cast_mut::<Panel>(image_id)?;
    image.set_texture(renderer.textures.get(texture_id)?);
    image.size = ComponentSize::Absolute(Vec2::new(300.0, 150.0));
    image.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
    image.anchor = Vec2::new(0.0, 1.0);
    image.offset = Vec2::new(0.0, -50.0);
    image.margin = ComponentMargin::new(0.0, 0.0, 10.0, 10.0);
    image.border_color = Color::SolidColor(SolidColor::new_rgb(72, 45, 6, 255));
    image.border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
    image.corner_rounding = ComponentCornerRounding::new(4.0, 4.0, 4.0, 4.0);
    ui.get_component_mut(main_window_id)?.add_child(image_id);

    let mut quote_panel_gradient = Gradient::new(GradientType::Vertical, Vec2::new(0.0, 0.0));
    quote_panel_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(96, 97, 115, 0), 0.0));
    quote_panel_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(96, 97, 115, 255), 1.0));

    let quote_panel_id = ui.create_panel(&mut renderer, ComponentShape::Rectangle)?;
    let quote_panel = ui.get_component_and_cast_mut::<Panel>(quote_panel_id)?;
    quote_panel.position = ComponentPosition::RelativeToParent(Vec2::new(1.0, 1.0));
    quote_panel.size = ComponentSize::Absolute(Vec2::new(400.0, 150.0));
    quote_panel.anchor = Vec2::new(1.0, 1.0);
    quote_panel.color = Color::SolidColor(SolidColor::new_rgb(60, 60, 60, 0));
    quote_panel.max_size = Vec2::new(f32::MAX, 150.0);
    quote_panel.margin = ComponentMargin::new(0.0, 0.0, 10.0, 10.0);
    quote_panel.offset = Vec2::new(0.0, -50.0);
    quote_panel.corner_rounding = ComponentCornerRounding::new(5.0, 5.0, 5.0, 5.0);
    quote_panel.border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
    quote_panel.border_color = Color::SolidColor(SolidColor::new_rgb(86, 92, 107, 255));
    ui.get_component_mut(main_window_id)?.add_child(quote_panel_id);

    let quote_id = ui.create_label(&mut renderer, regular_font_id)?;
    let quote = ui.get_component_and_cast_mut::<Label>(quote_id)?;
    quote.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
    quote.anchor = Vec2::new(0.0, 1.0);
    quote.margin = ComponentMargin::new(10.0, 10.0, 10.0, 10.0);
    quote.set_multiline_text(
        "\"To forget how to dig the earth and to tend the soil is to forget ourselves.\" - °200,255,200,255°Mahatma Gandhi°255,255,255,255°\n\n".to_string() + 
        "\"Agriculture is our wisest pursuit, because it will in the end contribute most to real wealth, good morals, and happiness.\" - °200,255,200,255°Thomas Jefferson°255,255,255,255°",
        360.0,
    );
    quote.shadow_enabled = true;
    quote.shadow_offset = Vec2::new(1.0, -1.0);
    quote.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 0.5));
    ui.get_component_mut(quote_panel_id)?.add_child(quote_id);

    let description_panel_id = ui.create_panel(&mut renderer, ComponentShape::Rectangle)?;
    let description_panel = ui.get_component_and_cast_mut::<Panel>(description_panel_id)?;
    description_panel.position = ComponentPosition::RelativeToParent(Vec2::new(1.0, 1.0));
    description_panel.size = ComponentSize::Relative(Vec2::new(1.0, 150.0));
    description_panel.anchor = Vec2::new(1.0, 1.0);
    description_panel.color = Color::SolidColor(SolidColor::new_rgb(60, 60, 60, 0));
    description_panel.max_size = Vec2::new(f32::MAX, 220.0);
    description_panel.margin = ComponentMargin::new(0.0, 0.0, 10.0, 10.0);
    description_panel.offset = Vec2::new(0.0, -210.0);
    description_panel.corner_rounding = ComponentCornerRounding::new(5.0, 8.0, 8.0, 5.0);
    description_panel.border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
    description_panel.border_color = Color::SolidColor(SolidColor::new_rgb(86, 92, 107, 255));
    ui.get_component_mut(main_window_id)?.add_child(description_panel_id);

    let mut scroll_gradient = Gradient::new(GradientType::Horizontal, Vec2::new(0.0, 0.0));
    scroll_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(213, 195, 168, 255), 0.0));
    scroll_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(167, 148, 134, 255), 1.0));

    let description_scrollbox_id = ui.create_scrollbox(&mut renderer)?;
    let description_scrollbox = ui.get_component_and_cast_mut::<Scrollbox>(description_scrollbox_id)?;
    description_scrollbox.size = ComponentSize::Relative(Vec2::new(1.0, 1.0));
    description_scrollbox.vertical_scroll_background_color = Color::SolidColor(SolidColor::new_rgb(0, 0, 0, 120));
    description_scrollbox.vertical_scroll_color = Color::Gradient(scroll_gradient.clone());
    description_scrollbox.vertical_scroll_corner_rounding = ComponentCornerRounding::new(8.0, 8.0, 8.0, 8.0);
    description_scrollbox.vertical_scroll_background_corner_rounding = ComponentCornerRounding::new(8.0, 8.0, 8.0, 8.0);
    description_scrollbox.vertical_scroll_border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
    description_scrollbox.vertical_scroll_border_color = Color::SolidColor(SolidColor::new_rgb(199, 178, 153, 255));
    description_scrollbox.vertical_scroll_background_border_color = Color::SolidColor(SolidColor::new_rgb(199, 178, 153, 255));
    description_scrollbox.vertical_scroll_background_border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);

    description_scrollbox.horizontal_scroll_background_color = Color::SolidColor(SolidColor::new_rgb(0, 0, 0, 120));
    description_scrollbox.horizontal_scroll_color = Color::Gradient(scroll_gradient.clone());
    description_scrollbox.horizontal_scroll_corner_rounding = ComponentCornerRounding::new(8.0, 8.0, 8.0, 8.0);
    description_scrollbox.horizontal_scroll_background_corner_rounding = ComponentCornerRounding::new(8.0, 8.0, 8.0, 8.0);
    description_scrollbox.horizontal_scroll_border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
    description_scrollbox.horizontal_scroll_border_color = Color::SolidColor(SolidColor::new_rgb(199, 178, 153, 255));
    description_scrollbox.horizontal_scroll_background_border_color = Color::SolidColor(SolidColor::new_rgb(199, 178, 153, 255));
    description_scrollbox.horizontal_scroll_background_border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
    description_scrollbox.padding = Vec2::new(20.0, 20.0);
    description_scrollbox.scroll_width = Vec2::new(20.0, 0.0);
    ui.get_component_mut(description_panel_id)?.add_child(description_scrollbox_id);

    let description_id = ui.create_label(&mut renderer, regular_font_id)?;
    let description = ui.get_component_and_cast_mut::<Label>(description_id)?;
    description.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
    description.anchor = Vec2::new(0.0, 1.0);
    description.margin = ComponentMargin::new(10.0, 10.0, 10.0, 10.0);
    description.set_multiline_text(
        "Agriculture began independently in different parts of the globe, and included a diverse range of taxa. At least eleven separate regions of the Old and New World were involved as independent centers of origin. The development of agriculture about 12,000 years ago changed the way humans lived. They switched from nomadic hunter-gatherer lifestyles to permanent settlements and farming.\n\n".to_string() + 
        "Wild grains were collected and eaten from at least 105,000 years ago. However, domestication did not occur until much later. The earliest evidence of small-scale cultivation of edible grasses is from around 21,000 BC with the Ohalo II people on the shores of the Sea of Galilee. By around 9500 BC, the eight Neolithic founder crops - emmer wheat, einkorn wheat, hulled barley, peas, lentils, bitter vetch, chickpeas, and flax - were cultivated in the Levant. Rye may have been cultivated earlier, but this claim remains controversial. Rice was domesticated in China by 6200 BC with earliest known cultivation from 5700 BC, followed by mung, soy and azuki beans. Rice was also independently domesticated in West Africa and cultivated by 1000 BC. Pigs were domesticated in Mesopotamia around 11,000 years ago, followed by sheep. Cattle were domesticated from the wild aurochs in the areas of modern Turkey and India around 8500 BC. Camels were domesticated late, perhaps around 3000 BC.\n\n" +
        "In subsaharan Africa, sorghum was domesticated in the Sahel region of Africa by 3000 BC, along with pearl millet by 2000 BC. Yams were domesticated in several distinct locations, including West Africa (unknown date), and cowpeas by 2500 BC. Rice (African rice) was also independently domesticated in West Africa and cultivated by 1000 BC. Teff and likely finger millet were domesticated in Ethiopia by 3000 BC, along with noog, ensete, and coffee. Other plant foods domesticated in Africa include watermelon, okra, tamarind and black eyed peas, along with tree crops such as the kola nut and oil palm. Plantains were cultivated in Africa by 3000 BC and bananas by 1500 BC. The helmeted guineafowl was domesticated in West Africa. Sanga cattle was likely also domesticated in North-East Africa, around 7000 BC, and later crossbred with other species.\n\n" +
        "In South America, agriculture began as early as 9000 BC, starting with the cultivation of several species of plants that later became only minor crops. In the Andes of South America, the potato was domesticated between 8000 BC and 5000 BC, along with beans, squash, tomatoes, peanuts, coca, llamas, alpacas, and guinea pigs. Cassava was domesticated in the Amazon Basin no later than 7000 BC. Maize (Zea mays) found its way to South America from Mesoamerica, where wild teosinte was domesticated about 7000 BC and selectively bred to become domestic maize. Cotton was domesticated in Peru by 4200 BC; another species of cotton was domesticated in Mesoamerica and became by far the most important species of cotton in the textile industry in modern times. Evidence of agriculture in the Eastern United States dates to about 3000 BCE. Several plants were cultivated, later to be replaced by the Three Sisters cultivation of maize, squash, and beans.",
        630.0,
    );
    description.shadow_enabled = true;
    description.shadow_offset = Vec2::new(1.0, -1.0);
    description.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 0.5));
    ui.get_component_mut(description_scrollbox_id)?.add_child(description_id);

    let mut effect_panel_1_gradient = Gradient::new(GradientType::Vertical, Vec2::new(0.0, 0.0));
    effect_panel_1_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(96, 97, 115, 0), 0.0));
    effect_panel_1_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(96, 97, 115, 255), 1.0));

    let effect_panel_1_id = ui.create_panel(&mut renderer, ComponentShape::Rectangle)?;
    let effect_panel_1 = ui.get_component_and_cast_mut::<Panel>(effect_panel_1_id)?;
    effect_panel_1.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 0.0));
    effect_panel_1.size = ComponentSize::Relative(Vec2::new(1.0, 1.0));
    effect_panel_1.anchor = Vec2::new(0.5, 0.0);
    effect_panel_1.color = Color::Gradient(effect_panel_1_gradient);
    effect_panel_1.max_size = Vec2::new(f32::MAX, 120.0);
    effect_panel_1.margin = ComponentMargin::new(0.0, 0.0, 10.0, 10.0);
    effect_panel_1.offset = Vec2::new(0.0, 60.0);
    effect_panel_1.corner_rounding = ComponentCornerRounding::new(5.0, 5.0, 5.0, 5.0);
    ui.get_component_mut(main_window_id)?.add_child(effect_panel_1_id);

    let effect_panel_2_id = ui.create_panel(&mut renderer, ComponentShape::Rectangle)?;
    let effect_panel_2 = ui.get_component_and_cast_mut::<Panel>(effect_panel_2_id)?;
    effect_panel_2.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 0.0));
    effect_panel_2.size = ComponentSize::Relative(Vec2::new(1.0, 1.0));
    effect_panel_2.anchor = Vec2::new(0.5, 0.0);
    effect_panel_2.color = Color::SolidColor(SolidColor::new_rgb(0, 0, 0, 60));
    effect_panel_2.max_size = Vec2::new(f32::MAX, 120.0);
    effect_panel_2.margin = ComponentMargin::new(10.0, 0.0, 20.0, 20.0);
    effect_panel_2.offset = Vec2::new(0.0, 60.0);
    effect_panel_2.corner_rounding = ComponentCornerRounding::new(5.0, 5.0, 5.0, 5.0);
    ui.get_component_mut(main_window_id)?.add_child(effect_panel_2_id);

    let effect_id = ui.create_label(&mut renderer, regular_font_id)?;
    let effect = ui.get_component_and_cast_mut::<Label>(effect_id)?;
    effect.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
    effect.anchor = Vec2::new(0.0, 1.0);
    effect.margin = ComponentMargin::new(3.0, 3.0, 3.0, 3.0);
    effect.label_text = "+1 \u{CA} and +1 \u{C9} per military unit stationed in the city\n".to_string()
        + "+10% \u{CA} for every farm belonging to capital (but not more than 50%)\n"
        + "+20% \u{C8} for every specialist\n"
        + "+1 \u{C8} worker mainteance";
    effect.shadow_enabled = true;
    effect.shadow_offset = Vec2::new(1.0, -1.0);
    effect.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 0.5));
    ui.get_component_mut(effect_panel_2_id)?.add_child(effect_id);

    let mut button_filling_gradient = Gradient::new(GradientType::Vertical, Vec2::new(0.0, 0.0));
    button_filling_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(159, 148, 135, 255), 0.0));
    button_filling_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(234, 221, 198, 255), 1.0));

    let ok_button_id = ui.create_button(&mut renderer, ComponentShape::Rectangle, bold_font_id)?;
    let ok_button = ui.get_component_and_cast_mut::<Button>(ok_button_id)?;
    ok_button.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 0.0));
    ok_button.size = ComponentSize::Absolute(Vec2::new(100.0, 25.0));
    ok_button.anchor = Vec2::new(0.5, 0.5);
    ok_button.color = Color::Gradient(button_filling_gradient.clone());
    ok_button.border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
    ok_button.border_color = Color::SolidColor(SolidColor::new(0.5, 0.5, 0.5, 1.0));
    ok_button.label_text = "Ok".to_string();
    ok_button.margin = ComponentMargin::new(0.0, 0.0, 0.0, 0.0);
    ok_button.label_horizontal_alignment = HorizontalAlignment::Middle;
    ok_button.label_vertical_alignment = VerticalAlignment::Middle;
    ok_button.label_offset = Vec2::new(0.0, 0.0);
    ok_button.label_color = Color::SolidColor(SolidColor::new_rgb(117, 95, 72, 255));
    ok_button.offset = Vec2::new(0.0, 30.0);
    ok_button.corner_rounding = ComponentCornerRounding::new(5.0, 5.0, 5.0, 5.0);
    ok_button.shadow_enabled = true;
    ok_button.shadow_offset = Vec2::new(2.0, -2.0);
    ok_button.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
    ok_button.shadow_corner_rounding = ComponentCornerRounding::new(5.0, 5.0, 5.0, 5.0);
    ok_button.shadow_scale = Vec2::new(1.01, 1.01);
    ok_button.on_button_pressed = Some(|button, _, _| {
        button.color.set_alpha(0.8);
        button.border_color.set_alpha(0.8);
        button.shadow_color.set_alpha(0.4);
    });
    ok_button.on_button_released = Some(|button, _, _| {
        button.color.set_alpha(1.0);
        button.border_color.set_alpha(1.0);
        button.shadow_color.set_alpha(1.0);
    });
    ui.get_component_mut(main_window_id)?.add_child(ok_button_id);
    /* #endregion */

    /* #region Right side window */
    let right_window_id = ui.create_panel(&mut renderer, ComponentShape::Rectangle)?;
    let right_window = ui.get_component_and_cast_mut::<Panel>(right_window_id)?;
    right_window.position = ComponentPosition::RelativeToParent(Vec2::new(1.0, 0.5));
    right_window.offset = Vec2::new(-15.0, 0.0);
    right_window.size = ComponentSize::Absolute(Vec2::new(300.0, 620.0));
    right_window.anchor = Vec2::new(1.0, 0.5);
    right_window.color = Color::Gradient(window_filling_gradient.clone());
    right_window.border_color = Color::Gradient(window_border_gradient.clone());
    right_window.border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
    right_window.corner_rounding = ComponentCornerRounding::new(10.0, 10.0, 10.0, 10.0);
    right_window.shadow_enabled = true;
    right_window.shadow_offset = Vec2::new(5.0, -5.0);
    right_window.shadow_color = Color::Gradient(window_shadow_gradient.clone());
    right_window.shadow_corner_rounding = ComponentCornerRounding::new(20.0, 20.0, 20.0, 20.0);
    right_window.shadow_scale = Vec2::new(1.03, 1.03);
    ui.get_component_mut(ui.main_canvas_id)?.add_child(right_window_id);

    let right_window_title_id = ui.create_label(&mut renderer, header_font_id)?;
    let right_window_title = ui.get_component_and_cast_mut::<Label>(right_window_title_id)?;
    right_window_title.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0));
    right_window_title.anchor = Vec2::new(0.5, 0.5);
    right_window_title.offset = Vec2::new(0.0, -27.0);
    right_window_title.label_text = "Settings".to_string();
    right_window_title.shadow_enabled = true;
    right_window_title.shadow_offset = Vec2::new(1.0, -1.0);
    right_window_title.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
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
        let checkbox = ui.get_component_and_cast_mut::<Checkbox>(checkbox_id)?;
        checkbox.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
        checkbox.offset = Vec2::new(0.0, -75.0 - 30.0 * checkbox_ids.len() as f32);
        checkbox.margin = ComponentMargin::new(3.0, 3.0, 3.0, 10.0);
        checkbox.label_offset = Vec2::new(25.0, 1.0);
        checkbox.box_offset = Vec2::new(0.0, 4.0);
        checkbox.label_text = checkbox_label.to_string();
        checkbox.label_shadow_enabled = true;
        checkbox.label_shadow_offset = Vec2::new(1.0, -1.0);
        checkbox.label_shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
        checkbox.on_cursor_enter = Some(|checkbox, _| checkbox.box_color.set_alpha(0.8));
        checkbox.on_cursor_leave = Some(|checkbox, _| checkbox.box_color.set_alpha(1.0));
        ui.get_component_mut(right_window_id)?.add_child(checkbox_id);

        checkbox_ids.push(checkbox_id);
    }

    let mut textbox_ids = Vec::new();
    let mut textbox_label_ids = Vec::new();
    let textbox_labels = ["Player name:", "Empire name:", "World name:"];

    for textbox_label in textbox_labels {
        let label_id = ui.create_label(&mut renderer, regular_font_id)?;
        let label = ui.get_component_and_cast_mut::<Label>(label_id)?;
        label.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
        label.offset = Vec2::new(0.0, -348.0 - 30.0 * textbox_label_ids.len() as f32);
        label.margin = ComponentMargin::new(3.0, 3.0, 3.0, 10.0);
        label.label_text = textbox_label.to_string();
        label.shadow_enabled = true;
        label.shadow_offset = Vec2::new(1.0, -1.0);
        label.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
        ui.get_component_mut(right_window_id)?.add_child(label_id);

        textbox_label_ids.push(label_id);

        let textbox_id = ui.create_textbox(&mut renderer, regular_font_id)?;
        let textbox = ui.get_component_and_cast_mut::<TextBox>(textbox_id)?;
        textbox.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
        textbox.size = ComponentSize::Absolute(Vec2::new(180.0, 30.0));
        textbox.offset = Vec2::new(110.0, -350.0 - 30.0 * textbox_ids.len() as f32);
        textbox.margin = ComponentMargin::new(3.0, 3.0, 3.0, 10.0);
        textbox.label_horizontal_alignment = HorizontalAlignment::Left;
        textbox.label_offset = Vec2::new(3.0, -1.0);
        textbox.color = Color::SolidColor(SolidColor::new_rgb(38, 41, 52, 255));
        textbox.border_color = Color::SolidColor(SolidColor::new_rgb(199, 178, 153, 255));
        textbox.border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
        textbox.corner_rounding = ComponentCornerRounding::new(3.0, 3.0, 3.0, 3.0);
        textbox.label_max_length = 15;
        textbox.on_cursor_enter = Some(|textbox, _| {
            if !textbox.active {
                textbox.color.set_alpha(0.8);
            }
        });
        textbox.on_cursor_leave = Some(|textbox, _| {
            if !textbox.active {
                textbox.color.set_alpha(1.0);
            }
        });
        textbox.on_activation = Some(|textbox, _| textbox.color.set_alpha(0.5));
        textbox.on_deactivation = Some(|textbox, _| textbox.color.set_alpha(1.0));
        ui.get_component_mut(right_window_id)?.add_child(textbox_id);

        textbox_ids.push(textbox_id);
    }

    let right_window_focuses_title_id = ui.create_label(&mut renderer, header_font_id)?;
    let right_window_focuses_title = ui.get_component_and_cast_mut::<Label>(right_window_focuses_title_id)?;
    right_window_focuses_title.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0));
    right_window_focuses_title.anchor = Vec2::new(0.5, 0.5);
    right_window_focuses_title.offset = Vec2::new(0.0, -445.0);
    right_window_focuses_title.label_text = "City focuses".to_string();
    right_window_focuses_title.shadow_enabled = true;
    right_window_focuses_title.shadow_offset = Vec2::new(1.0, -1.0);
    right_window_focuses_title.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
    ui.get_component_mut(right_window_id)?.add_child(right_window_focuses_title_id);

    let mut toggle_button_ids = Vec::new();
    let toggle_button_labels = ["\u{C8}", "\u{C9}", "\u{CA}"];

    for toggle_button_label in toggle_button_labels {
        let button_id = ui.create_button(&mut renderer, ComponentShape::Rectangle, regular_font_id)?;
        let button = ui.get_component_and_cast_mut::<Button>(button_id)?;
        button.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
        button.size = ComponentSize::Absolute(Vec2::new(40.0, 30.0));
        button.offset = Vec2::new(100.0 + 50.0 * toggle_button_ids.len() as f32, -480.0);
        button.anchor = Vec2::new(0.5, 0.5);
        button.color = Color::Gradient(button_filling_gradient.clone());
        button.border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
        button.border_color = Color::SolidColor(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        button.label_text = toggle_button_label.to_string();
        button.margin = ComponentMargin::new(0.0, 0.0, 0.0, 0.0);
        button.label_horizontal_alignment = lemao_ui::components::HorizontalAlignment::Middle;
        button.label_vertical_alignment = lemao_ui::components::VerticalAlignment::Middle;
        button.label_offset = Vec2::new(0.0, -2.0);
        button.corner_rounding = ComponentCornerRounding::new(5.0, 5.0, 5.0, 5.0);
        button.shadow_enabled = true;
        button.shadow_offset = Vec2::new(2.0, -2.0);
        button.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
        button.shadow_corner_rounding = ComponentCornerRounding::new(5.0, 5.0, 5.0, 5.0);
        button.shadow_scale = Vec2::new(1.01, 1.01);
        button.toggleable = true;
        button.on_button_pressed = Some(|button, _, _| {
            button.color.set_alpha(0.4);
            button.border_color.set_alpha(0.4);
            button.shadow_color.set_alpha(0.0);
            button.label_color.set_alpha(0.2);
        });
        button.on_button_released = Some(|button, _, _| {
            button.color.set_alpha(1.0);
            button.border_color.set_alpha(1.0);
            button.shadow_color.set_alpha(1.0);
            button.label_color.set_alpha(1.0);
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
        let label = ui.get_component_and_cast_mut::<Label>(label_id)?;
        label.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
        label.anchor = Vec2::new(0.0, 0.5);
        label.offset = Vec2::new(0.0, -530.0 - 30.0 * slider_label_ids.len() as f32);
        label.margin = ComponentMargin::new(3.0, 3.0, 3.0, 10.0);
        label.label_text = slider_label.to_string();
        label.shadow_enabled = true;
        label.shadow_offset = Vec2::new(1.0, -1.0);
        label.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
        ui.get_component_mut(right_window_id)?.add_child(label_id);

        slider_label_ids.push(label_id);

        let slider_id = ui.create_slider(&mut renderer, ComponentShape::Disc)?;
        let slider = ui.get_component_and_cast_mut::<Slider>(slider_id)?;
        slider.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0));
        slider.size = ComponentSize::Absolute(Vec2::new(210.0, 10.0));
        slider.anchor = Vec2::new(0.5, 0.5);
        slider.offset = Vec2::new(35.0, -530.0 - 30.0 * slider_ids.len() as f32);
        slider.margin = ComponentMargin::new(3.0, 3.0, 3.0, 10.0);
        slider.color = Color::SolidColor(SolidColor::new_rgb(38, 41, 52, 255));
        slider.bar_color = Color::SolidColor(SolidColor::new_rgb(219, 198, 173, 255));
        slider.set_phase(0.5);
        slider.border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
        slider.border_color = Color::SolidColor(SolidColor::new_rgb(199, 178, 153, 255));
        slider.corner_rounding = ComponentCornerRounding::new(1.0, 1.0, 1.0, 1.0);
        slider.selector_size = Vec2::new(15.0, 15.0);
        slider.selector_color = Color::Gradient(selector_filling_gradient.clone());
        slider.steps_count = slider_steps_count[slider_ids.len()];
        slider.selector_border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
        slider.selector_border_color = Color::SolidColor(SolidColor::new_rgb(199, 178, 153, 255));
        ui.get_component_mut(right_window_id)?.add_child(slider_id);

        slider_ids.push(slider_id);
    }
    /* #endregion */

    /* #region Left side window */
    let left_window_id = ui.create_panel(&mut renderer, ComponentShape::Rectangle)?;
    let left_window = ui.get_component_and_cast_mut::<Panel>(left_window_id)?;
    left_window.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 0.5));
    left_window.offset = Vec2::new(15.0, 0.0);
    left_window.size = ComponentSize::Absolute(Vec2::new(300.0, 620.0));
    left_window.anchor = Vec2::new(0.0, 0.5);
    left_window.color = Color::Gradient(window_filling_gradient.clone());
    left_window.border_color = Color::Gradient(window_border_gradient.clone());
    left_window.border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
    left_window.corner_rounding = ComponentCornerRounding::new(10.0, 10.0, 10.0, 10.0);
    left_window.shadow_enabled = true;
    left_window.shadow_offset = Vec2::new(5.0, -5.0);
    left_window.shadow_color = Color::Gradient(window_shadow_gradient.clone());
    left_window.shadow_corner_rounding = ComponentCornerRounding::new(20.0, 20.0, 20.0, 20.0);
    left_window.shadow_scale = Vec2::new(1.03, 1.03);
    ui.get_component_mut(ui.main_canvas_id)?.add_child(left_window_id);

    let left_window_title_id = ui.create_label(&mut renderer, header_font_id)?;
    let left_window_title = ui.get_component_and_cast_mut::<Label>(left_window_title_id)?;
    left_window_title.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0));
    left_window_title.anchor = Vec2::new(0.5, 0.5);
    left_window_title.offset = Vec2::new(0.0, -27.0);
    left_window_title.label_text = "Statistics".to_string();
    left_window_title.shadow_enabled = true;
    left_window_title.shadow_offset = Vec2::new(1.0, -1.0);
    left_window_title.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
    ui.get_component_mut(left_window_id)?.add_child(left_window_title_id);

    let mut pie_chart_1_filling_gradient = Gradient::new(GradientType::Radial, Vec2::new(0.0, 0.0));
    pie_chart_1_filling_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(254, 135, 177, 255), 0.0));
    pie_chart_1_filling_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(234, 95, 137, 255), 1.0));

    let pie_chart_1_id = ui.create_panel(&mut renderer, ComponentShape::Disc)?;
    let pie_chart_1 = ui.get_component_and_cast_mut::<Panel>(pie_chart_1_id)?;
    pie_chart_1.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0));
    pie_chart_1.offset = Vec2::new(0.0, -180.0);
    pie_chart_1.size = ComponentSize::Absolute(Vec2::new(250.0, 250.0));
    pie_chart_1.anchor = Vec2::new(0.5, 0.5);
    pie_chart_1.color = Color::Gradient(pie_chart_1_filling_gradient);
    pie_chart_1.border_color = Color::SolidColor(SolidColor::new_rgb(0, 0, 0, 255));
    pie_chart_1.border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
    pie_chart_1.start_angle = 0.0;
    pie_chart_1.end_angle = std::f32::consts::PI * 0.7;
    pie_chart_1.on_cursor_enter = Some(|panel, _| panel.color.set_alpha(0.5));
    pie_chart_1.on_cursor_leave = Some(|panel, _| panel.color.set_alpha(1.0));
    ui.get_component_mut(left_window_id)?.add_child(pie_chart_1_id);

    let mut pie_chart_2_filling_gradient = Gradient::new(GradientType::Radial, Vec2::new(0.0, 0.0));
    pie_chart_2_filling_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(175, 69, 166, 255), 0.0));
    pie_chart_2_filling_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(135, 29, 126, 255), 1.0));

    let pie_chart_2_id = ui.create_panel(&mut renderer, ComponentShape::Disc)?;
    let pie_chart_2 = ui.get_component_and_cast_mut::<Panel>(pie_chart_2_id)?;
    pie_chart_2.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0));
    pie_chart_2.offset = Vec2::new(0.0, -180.0);
    pie_chart_2.size = ComponentSize::Absolute(Vec2::new(250.0, 250.0));
    pie_chart_2.anchor = Vec2::new(0.5, 0.5);
    pie_chart_2.color = Color::Gradient(pie_chart_2_filling_gradient);
    pie_chart_2.border_color = Color::SolidColor(SolidColor::new_rgb(0, 0, 0, 255));
    pie_chart_2.border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
    pie_chart_2.start_angle = std::f32::consts::PI * 0.7;
    pie_chart_2.end_angle = std::f32::consts::PI * 1.3;
    pie_chart_2.on_cursor_enter = Some(|panel, _| panel.color.set_alpha(0.5));
    pie_chart_2.on_cursor_leave = Some(|panel, _| panel.color.set_alpha(1.0));
    ui.get_component_mut(left_window_id)?.add_child(pie_chart_2_id);

    let mut pie_chart_3_filling_gradient = Gradient::new(GradientType::Radial, Vec2::new(0.0, 0.0));
    pie_chart_3_filling_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(147, 82, 186, 255), 0.0));
    pie_chart_3_filling_gradient.steps.push(GradientStep::new(SolidColor::new_rgb(107, 42, 146, 255), 1.0));

    let pie_chart_3_id = ui.create_panel(&mut renderer, ComponentShape::Disc)?;
    let pie_chart_3 = ui.get_component_and_cast_mut::<Panel>(pie_chart_3_id)?;
    pie_chart_3.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0));
    pie_chart_3.offset = Vec2::new(0.0, -180.0);
    pie_chart_3.size = ComponentSize::Absolute(Vec2::new(250.0, 250.0));
    pie_chart_3.anchor = Vec2::new(0.5, 0.5);
    pie_chart_3.color = Color::Gradient(pie_chart_3_filling_gradient);
    pie_chart_3.border_color = Color::SolidColor(SolidColor::new_rgb(0, 0, 0, 255));
    pie_chart_3.border_thickness = ComponentBorderThickness::new(1.0, 1.0, 1.0, 1.0);
    pie_chart_3.start_angle = std::f32::consts::PI * 1.3;
    pie_chart_3.end_angle = std::f32::consts::PI * 2.0;
    pie_chart_3.on_cursor_enter = Some(|panel, _| panel.color.set_alpha(0.5));
    pie_chart_3.on_cursor_leave = Some(|panel, _| panel.color.set_alpha(1.0));
    ui.get_component_mut(left_window_id)?.add_child(pie_chart_3_id);

    let pie_chart_legend_id = ui.create_label(&mut renderer, regular_font_id)?;
    let pie_chart_legend = ui.get_component_and_cast_mut::<Label>(pie_chart_legend_id)?;
    pie_chart_legend.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0));
    pie_chart_legend.anchor = Vec2::new(0.5, 1.0);
    pie_chart_legend.offset = Vec2::new(0.0, -320.0);
    pie_chart_legend.label_text = "°254,135,177,255°\u{CB}°255,255,255,255° - import ".to_string()
        + "°175,69,166,255°\u{CB}°255,255,255,255° - export "
        + "°147,82,186,255°\u{CB}°255,255,255,255° - domestic";
    pie_chart_legend.shadow_enabled = true;
    pie_chart_legend.shadow_offset = Vec2::new(1.0, -1.0);
    pie_chart_legend.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 0.5));
    ui.get_component_mut(left_window_id)?.add_child(pie_chart_legend_id);

    let line_chart_id = ui.create_wire(&mut renderer)?;
    let line_chart = ui.get_component_and_cast_mut::<Wire>(line_chart_id)?;
    line_chart.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0));
    line_chart.offset = Vec2::new(0.0, -480.0);
    line_chart.size = ComponentSize::Absolute(Vec2::new(250.0, 250.0));
    line_chart.anchor = Vec2::new(0.5, 0.5);
    line_chart.data = vec![
        // Helper lines
        WireChunkData::new(Color::SolidColor(SolidColor::new_rgb(255, 255, 255, 50)), Vec2::new(0.0, 0.2), Vec2::new(1.0, 0.2), 1.0),
        WireChunkData::new(Color::SolidColor(SolidColor::new_rgb(255, 255, 255, 50)), Vec2::new(0.0, 0.4), Vec2::new(1.0, 0.4), 1.0),
        WireChunkData::new(Color::SolidColor(SolidColor::new_rgb(255, 255, 255, 50)), Vec2::new(0.0, 0.6), Vec2::new(1.0, 0.6), 1.0),
        WireChunkData::new(Color::SolidColor(SolidColor::new_rgb(255, 255, 255, 50)), Vec2::new(0.0, 0.8), Vec2::new(1.0, 0.8), 1.0),
        // Data 1
        WireChunkData::new(Color::SolidColor(SolidColor::new_rgb(150, 150, 255, 255)), Vec2::new(0.0, 0.0), Vec2::new(0.5, 0.5), 2.0),
        WireChunkData::new(Color::SolidColor(SolidColor::new_rgb(150, 150, 255, 255)), Vec2::new(0.5, 0.5), Vec2::new(0.6, 0.4), 2.0),
        WireChunkData::new(Color::SolidColor(SolidColor::new_rgb(150, 150, 255, 255)), Vec2::new(0.6, 0.4), Vec2::new(1.0, 1.0), 2.0),
        // Data 2
        WireChunkData::new(Color::SolidColor(SolidColor::new_rgb(255, 150, 150, 255)), Vec2::new(0.0, 1.0), Vec2::new(0.25, 0.6), 2.0),
        WireChunkData::new(Color::SolidColor(SolidColor::new_rgb(255, 150, 150, 255)), Vec2::new(0.25, 0.6), Vec2::new(0.50, 0.4), 2.0),
        WireChunkData::new(Color::SolidColor(SolidColor::new_rgb(255, 150, 150, 255)), Vec2::new(0.50, 0.4), Vec2::new(0.75, 0.2), 2.0),
        WireChunkData::new(Color::SolidColor(SolidColor::new_rgb(255, 150, 150, 255)), Vec2::new(0.75, 0.2), Vec2::new(1.00, 0.15), 2.0),
        // Axis
        WireChunkData::new(Color::SolidColor(SolidColor::new_rgb(255, 255, 255, 255)), Vec2::new(0.0, 1.0), Vec2::new(0.0, 0.0), 1.0),
        WireChunkData::new(Color::SolidColor(SolidColor::new_rgb(255, 255, 255, 255)), Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), 1.0),
    ];
    ui.get_component_mut(left_window_id)?.add_child(line_chart_id);
    /* #endregion */

    let mut is_running = true;

    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::WindowSizeChanged(size) => {
                    renderer.set_viewport_size(size)?;
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

                    let phase = ui.get_component_and_cast_mut::<Slider>(component_id)?.phase;
                    let label_index = slider_ids.iter().position(|&p| p == component_id).unwrap();
                    let label_content = slider_labels[label_index].replace("50%", &format!("{:.0}%", phase * 100.0));

                    let slider_label = ui.get_component_and_cast_mut::<Label>(slider_label_ids[label_index])?;
                    slider_label.label_text = label_content;
                    slider_label.dirty = true;
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
        ui.draw(&mut renderer, left_window_title_id)?;
        ui.draw(&mut renderer, pie_chart_1_id)?;
        ui.draw(&mut renderer, pie_chart_2_id)?;
        ui.draw(&mut renderer, pie_chart_3_id)?;
        ui.draw(&mut renderer, pie_chart_legend_id)?;
        ui.draw(&mut renderer, line_chart_id)?;

        window.swap_buffers();
    }

    Ok(())
}

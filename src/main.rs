#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    cell::Cell,
    f32,
    rc::Rc,
    sync::{Arc, Mutex},
};

use log::trace;

use crate::{
    configuration::{Configuration, Theme, WindowMode},
    gelatin::{
        application::*,
        glium::glutin::{window::Icon},
        image,
        label::*,
        line_layout_container::*,
        misc::*,
        picture::*,
        window::{Window, WindowDescriptor},
    },
    widgets::{
        bottom_bar::BottomBar, copy_notification::CopyNotifications,
        help_screen::*, picture_widget::*,
    },
};

mod clipboard_handler;
mod cmd_line;
mod configuration;
mod gelatin;
mod handle_panic;
mod image_cache;
mod input_handling;
mod parallel_action;
mod playback_manager;
mod shaders;
mod utils;
mod version;
mod widgets;

static USAGE: &[u8] = include_bytes!("../resource/usage.png");
static LEFT_TO_PAN: &[u8] = include_bytes!("../resource/use-left-to-pan.png");

// ========================================================
// Not-so glorious main function
// ========================================================
fn main() {
    std::panic::set_hook(Box::new(handle_panic::handle_panic));
    env_logger::init();
    trace!("Starting up. Panic hook set, logger initialized.");

    let args = cmd_line::parse_args();

    let config = Configuration::load();
    let first_launch = config.is_err();
    let config = Arc::new(Mutex::new(config.unwrap_or_default()));

    let mut application = Application::new();
    let window: Rc<Window> = {
        let cfg = &mut config.lock().unwrap();
        let window_desc = WindowDescriptor::builder()
            .icon(Some(make_icon()))
            .build();
        let window = Window::new(&mut application, window_desc);

        match cfg.window_mode() {
            WindowMode::Fullscreen => window.set_fullscreen(true),
            WindowMode::Maximized => window.set_maximized(true),
            _ => (),
        }
        window
    };

    let usage_img = Picture::from_encoded_bytes(USAGE);
    let help_screen = Rc::new(HelpScreen::new(usage_img));
    let left_to_pan_img = Picture::from_encoded_bytes(LEFT_TO_PAN);
    let left_to_pan_hint = Rc::new(HelpScreen::new(left_to_pan_img));

    let copy_notifications_widget = Rc::new(Label::new());
    let copy_notifications = CopyNotifications::new(&copy_notifications_widget);

    let bottom_bar = Rc::new(BottomBar::new());
    let picture_widget = make_picture_widget(
        &window,
        bottom_bar.clone(),
        left_to_pan_hint.clone(),
        copy_notifications,
        config.clone(),
    );

    if let Some(file_path) = args.file_path {
        picture_widget.jump_to_path(file_path);
    }

    let picture_area_container = make_picture_area_container();
    picture_area_container.add_child(picture_widget.clone());
    picture_area_container.add_child(copy_notifications_widget);
    picture_area_container.add_child(left_to_pan_hint);
    picture_area_container.add_child(help_screen.clone());

    let root_container = make_root_container();
    root_container.add_child(picture_area_container);
    root_container.add_child(bottom_bar.widget.clone());

    let set_theme = {
        let picture_widget = picture_widget.clone();
        let window = window.clone();
        let bottom_bar = bottom_bar.clone();
        let config = config.clone();

        Rc::new(move || {
            let theme = config.lock().unwrap().theme();
            match theme {
                Theme::Light => {
                    picture_widget.set_bright_shade(0.96);
                    window.set_bg_color([0.85, 0.85, 0.85, 1.0]);
                }
                Theme::Dark => {
                    picture_widget.set_bright_shade(0.11);
                    window.set_bg_color([0.03, 0.03, 0.03, 1.0]);
                }
            }
            bottom_bar.set_theme(theme);
        })
    };
    set_theme();
    {
        let set_theme = set_theme;
        let config = config.clone();
        bottom_bar.theme_button.set_on_click(move || {
            let theme = config.lock().unwrap().theme().toggle();
            config.lock().unwrap().set_theme(theme);
            set_theme();
        });
    }
    {
        let slider = bottom_bar.slider.clone();
        let picture_widget = picture_widget.clone();
        bottom_bar.slider.set_on_value_change(move || {
            picture_widget.jump_to_index(slider.value());
        });
    }
    {
        let picture_widget = picture_widget.clone();
        bottom_bar.orig_scale_button.set_on_click(move || {
            picture_widget.set_img_size_to_orig();
        });
    }
    {
        let picture_widget = picture_widget.clone();
        bottom_bar.fit_best_button.set_on_click(move || {
            picture_widget.set_img_size_to_fit(false);
        });
    }
    {
        bottom_bar.fit_stretch_button.set_on_click(move || {
            picture_widget.set_img_size_to_fit(true);
        });
    }
    let help_visible = Cell::new(first_launch);
    bottom_bar.set_help_visible(help_visible.get());
    help_screen.set_visible(help_visible.get());
    {
        let help_screen = help_screen;
        let bottom_bar_clone = bottom_bar.clone();

        bottom_bar.help_button.set_on_click(move || {
            help_visible.set(!help_visible.get());
            help_screen.set_visible(help_visible.get());
            bottom_bar_clone.set_help_visible(help_visible.get());
        });
    }

    window.set_root(root_container);

    application.set_at_exit(Some(move || {
        config.lock().unwrap().save().unwrap();
    }));
    application.start_event_loop();
}
// ========================================================

fn make_icon() -> Icon {
    let img =
        image::load_from_memory(include_bytes!("../resource/alloy48.png"))
            .unwrap();
    let rgba = img.into_rgba8();
    let (w, h) = rgba.dimensions();
    Icon::from_rgba(rgba.into_raw(), w, h).unwrap()
}

fn make_root_container() -> Rc<VerticalLayoutContainer> {
    let container = Rc::new(VerticalLayoutContainer::new());
    container.set_margin_all(0.0);
    container.set_height(Length::Stretch {
        min: 0.0,
        max: f32::INFINITY,
    });
    container.set_width(Length::Stretch {
        min: 0.0,
        max: f32::INFINITY,
    });
    container
}

fn make_picture_area_container() -> Rc<VerticalLayoutContainer> {
    let picture_area_container = Rc::new(VerticalLayoutContainer::new());
    picture_area_container.set_margin_all(0.0);
    picture_area_container.set_height(Length::Stretch {
        min: 0.0,
        max: f32::INFINITY,
    });
    picture_area_container.set_width(Length::Stretch {
        min: 0.0,
        max: f32::INFINITY,
    });
    picture_area_container
}

fn make_picture_widget(
    window: &Rc<Window>,
    bottom_bar: Rc<BottomBar>,
    left_to_pan_hint: Rc<HelpScreen>,
    copy_notifications: CopyNotifications,
    config: Arc<Mutex<Configuration>>,
) -> Rc<PictureWidget> {
    let picture_widget = Rc::new(PictureWidget::new(
        &window.display_mut(),
        window,
        bottom_bar,
        left_to_pan_hint,
        copy_notifications,
        config,
    ));
    picture_widget.set_height(Length::Stretch {
        min: 0.0,
        max: f32::INFINITY,
    });
    picture_widget.set_width(Length::Stretch {
        min: 0.0,
        max: f32::INFINITY,
    });
    picture_widget
}

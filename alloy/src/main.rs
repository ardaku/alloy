#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    cell::{Cell, RefCell},
    f32,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, Mutex},
};

use directories_next::ProjectDirs;
use gelatin::{
    application::*,
    glium::glutin::{
        dpi::{PhysicalPosition, PhysicalSize},
        event::WindowEvent,
        window::Icon,
    },
    image,
    label::*,
    line_layout_container::*,
    misc::*,
    picture::*,
    window::{Window, WindowDescriptor},
};
use lazy_static::lazy_static;
use log::trace;

use crate::{
    configuration::{Cache, ConfigWindowSection, Configuration, Theme},
    widgets::{
        bottom_bar::BottomBar, copy_notification::CopyNotifications,
        help_screen::*, picture_widget::*,
    },
};

mod clipboard_handler;
mod cmd_line;
mod configuration;
mod handle_panic;
mod image_cache;
mod input_handling;
mod parallel_action;
mod playback_manager;
mod shaders;
mod utils;
mod version;
mod widgets;

lazy_static! {
    // The program name will be 'emulsion'
    // (i.e. starting with a lower-case letter) on Linux
    pub static ref PROJECT_DIRS: Option<ProjectDirs> = ProjectDirs::from("", "", "Emulsion");
}

static USAGE: &[u8] = include_bytes!("../resource/usage.png");
static LEFT_TO_PAN: &[u8] = include_bytes!("../resource/use-left-to-pan.png");

// ========================================================
// Not-so glorious main function
// ========================================================
fn main() {
    std::panic::set_hook(Box::new(handle_panic::handle_panic));
    env_logger::init();
    trace!("Starting up. Panic hook set, logger initialized.");

    // Load configuration and cache files
    let (config_path, cache_path) = get_config_and_cache_paths();

    let args = cmd_line::parse_args();

    let cache = Cache::load(&cache_path);
    let config = Configuration::load(config_path);

    let first_launch = cache.is_err();
    let cache = Arc::new(Mutex::new(cache.unwrap_or_default()));
    let config = Rc::new(RefCell::new(config.unwrap_or_default()));

    let mut application = Application::new();
    let window: Rc<Window> = {
        let window_cache = &mut cache.lock().unwrap().window;
        let window_cfg = &config.borrow().window;
        let window_defaults = configuration::CacheWindowSection::default();

        if let Some(ConfigWindowSection {
            use_last_window_area: Some(false),
            win_x,
            win_y,
            win_w,
            win_h,
            ..
        }) = window_cfg
        {
            window_cache.win_x = if let Some(x) = win_x {
                *x
            } else {
                window_defaults.win_x
            };
            window_cache.win_y = if let Some(y) = win_y {
                *y
            } else {
                window_defaults.win_y
            };
            window_cache.win_w = if let Some(w) = win_w {
                *w
            } else {
                window_defaults.win_w
            };
            window_cache.win_h = if let Some(h) = win_h {
                *h
            } else {
                window_defaults.win_h
            };
        } else {
            let right = window_cache.win_x as i64 + window_cache.win_w as i64;
            if right < 20 {
                window_cache.win_w = window_defaults.win_w;
                window_cache.win_x = window_defaults.win_x;
            }
            if window_cache.win_y < 20 {
                window_cache.win_y = window_defaults.win_y;
            }
        }
        let pos = PhysicalPosition::new(window_cache.win_x, window_cache.win_y);
        let size = PhysicalSize::new(window_cache.win_w, window_cache.win_h);
        let window_desc = WindowDescriptor::builder()
            .icon(Some(make_icon()))
            .size(size)
            .position(Some(pos))
            .app_id(Some("Alloy".into()))
            .build();
        let window = Window::new(&mut application, window_desc);
        // This is just to fix the bug on Linux that the window doesn't start up at
        // the specified position when the position is specified during initialization
        window
            .display_mut()
            .gl_window()
            .window()
            .set_outer_position(pos);

        if let Some(ConfigWindowSection {
            start_maximized: Some(true),
            ..
        }) = window_cfg
        {
            window.set_maximized(true);
        }

        if let Some(ConfigWindowSection {
            start_fullscreen: Some(true),
            ..
        }) = window_cfg
        {
            window.set_fullscreen(true);
        }
        window
    };
    add_window_movement_listener(&window, cache.clone());

    let usage_img = Picture::from_encoded_bytes(USAGE);
    let help_screen = Rc::new(HelpScreen::new(usage_img));
    let left_to_pan_img = Picture::from_encoded_bytes(LEFT_TO_PAN);
    let left_to_pan_hint = Rc::new(HelpScreen::new(left_to_pan_img));

    let copy_notifications_widget = Rc::new(Label::new());
    let copy_notifications = CopyNotifications::new(&copy_notifications_widget);

    let bottom_bar = Rc::new(BottomBar::new(&config.borrow()));
    let picture_widget = make_picture_widget(
        &window,
        bottom_bar.clone(),
        left_to_pan_hint.clone(),
        copy_notifications,
        config.clone(),
        cache.clone(),
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

    let theme = {
        Rc::new(Cell::new(match &config.borrow().window {
            Some(ConfigWindowSection {
                theme: Some(theme_cfg),
                ..
            }) => *theme_cfg,
            _ => cache.lock().unwrap().theme(),
        }))
    };

    let set_theme = {
        let picture_widget = picture_widget.clone();
        let window = window.clone();
        let theme = theme.clone();
        let bottom_bar = bottom_bar.clone();

        Rc::new(move || {
            match theme.get() {
                Theme::Light => {
                    picture_widget.set_bright_shade(0.96);
                    window.set_bg_color([0.85, 0.85, 0.85, 1.0]);
                }
                Theme::Dark => {
                    picture_widget.set_bright_shade(0.11);
                    window.set_bg_color([0.03, 0.03, 0.03, 1.0]);
                }
            }
            bottom_bar.set_theme(
                theme.get(),
            );
        })
    };
    set_theme();
    {
        let cache = cache.clone();
        let set_theme = set_theme.clone();
        bottom_bar.theme_button.set_on_click(move || {
            let new_theme = theme.get().switch_theme();
            theme.set(new_theme);
            cache.lock().unwrap().set_theme(new_theme);
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
    help_screen.set_visible(help_visible.get());
    {
        let help_screen = help_screen.clone();
        let bottom_bar_clone = bottom_bar.clone();

        bottom_bar.help_button.set_on_click(move || {
            help_visible.set(!help_visible.get());
            help_screen.set_visible(help_visible.get());
            bottom_bar_clone.set_help_visible(help_visible.get());
        });
    }

    window.set_root(root_container);

    application.set_at_exit(Some(move || {
        cache.lock().unwrap().save(cache_path).unwrap();
    }));
    application.start_event_loop();
}
// ========================================================

fn make_icon() -> Icon {
    let img =
        image::load_from_memory(include_bytes!("../resource/emulsion48.png"))
            .unwrap();
    let rgba = img.into_rgba8();
    let (w, h) = rgba.dimensions();
    Icon::from_rgba(rgba.into_raw(), w, h).unwrap()
}

fn add_window_movement_listener(window: &Window, cache: Arc<Mutex<Cache>>) {
    window.add_global_event_handler(move |event| match event {
        WindowEvent::Resized(new_size) => {
            let mut cache = cache.lock().unwrap();
            cache.window.win_w = new_size.width;
            cache.window.win_h = new_size.height;
        }
        WindowEvent::Moved(new_pos) => {
            let mut cache = cache.lock().unwrap();
            cache.window.win_x = new_pos.x;
            cache.window.win_y = new_pos.y;
        }
        _ => (),
    });
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
    config: Rc<RefCell<Configuration>>,
    cache: Arc<Mutex<Cache>>,
) -> Rc<PictureWidget> {
    let picture_widget = Rc::new(PictureWidget::new(
        &window.display_mut(),
        window,
        bottom_bar,
        left_to_pan_hint,
        copy_notifications,
        config,
        cache,
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

pub fn get_config_and_cache_paths() -> (PathBuf, PathBuf) {
    let config_folder;
    let cache_folder;

    if let Some(ref project_dirs) = *PROJECT_DIRS {
        config_folder = project_dirs.config_dir().to_owned();
        cache_folder = project_dirs.cache_dir().to_owned();
    } else {
        let exe_path = std::env::current_exe().unwrap();
        let exe_folder = exe_path.parent().unwrap();
        config_folder = exe_folder.to_owned();
        cache_folder = exe_folder.to_owned();
    }
    if !config_folder.exists() {
        std::fs::create_dir_all(&config_folder).unwrap();
    }
    if !cache_folder.exists() {
        std::fs::create_dir_all(&cache_folder).unwrap();
    }

    (
        config_folder.join("cfg.toml"),
        cache_folder.join("cache.toml"),
    )
}

use ggez::graphics;
use ggez::*;

struct State {
    dt: std::time::Duration,
}

impl State {
    pub fn new(_ctx: &mut Context) -> State {
        // Load/create resources here: images, fonts, sounds, etc.
        State {
            dt: std::time::Duration::new(0, 0),
        }
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {}
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        println!("Hello ggez! dt={}ns", self.dt.subsec_nanos());
        // Draw code here...
        // let fps = timer::fps(ctx);
        // let fps_display = Text::new(format!("FPS: {}", fps));
        // // When drawing through these calls, `DrawParam` will work as they are documented.
        // graphics::draw(
        //     ctx,
        //     &fps_display,
        //     (Point2::new(200.0, 0.0), graphics::WHITE),
        // )?;



        graphics::present(ctx)
        // Ok(());
    }
}

fn main() {
    // settings for the game window
    let other_window_mode = ggez::conf::WindowMode {
        width: 800.0,
        height: 600.0,
        maximized: false,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    };
    
    let window_mode = ggez::conf::WindowMode::default()
        .dimensions(400.0, 400.0)
        .borderless(true)
        .min_dimensions(100.0, 100.0)
        .resizable(true);
    // settings for the game title, icon, and other settings
    // https://docs.rs/ggez/0.5.1/ggez/conf/struct.WindowSetup.html
    let window_setup = ggez::conf::WindowSetup::default().title("WOW👁👅👁");

    // composite of all the configuration settings above
    // https://docs.rs/ggez/0.5.1/ggez/conf/struct.Conf.html
    let c = conf::Conf{
        window_mode: window_mode,
        window_setup: window_setup,
        // sets the OpenGL Version
        backend: ggez::conf::Backend::default(),
        modules: ggez::conf::ModuleConf::default(),
    };
    let (mut ctx, mut event_loop) = ContextBuilder::new("game_name", "author_name")
        .conf(c)
        .build()
        .unwrap();
    // ggez::graphics::set_mode(&mut ctx, window_mode).unwrap();

    // not really sure what this really does yet... I think it can be used to set resolution
    // https://docs.rs/ggez/0.5.1/ggez/graphics/fn.set_screen_coordinates.html
    // let position = ggez::graphics::Rect::new(0_f32, 0_f32, 32_f32, 32_f32);
    //  ggez::graphics::set_screen_coordinates(&mut ctx, position).unwrap();

    let mut state = State::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut state) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

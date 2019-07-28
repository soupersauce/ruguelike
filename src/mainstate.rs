struct MainState{
    canvas: graphics::Canvas,
    assets: Assets,
}

impl MainState{
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let canvas = graphics::Canvas::with_window_size(ctx)?;
        let assets = Assets::new(ctx)?;
        Ok(MainState{ canvas, assets})
    }
}

impl EventHandler for MainState{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::set_canvas(ctx, Some(&self.canvas));
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        for x in 0..80 {
            for y in 0..45 {
                let draw_pos = map_to_window_coords(x, y);
                let param = graphics::DrawParam::new()
                    .dest(draw_pos)
                    .scale(Vector2::new(0.5, 0.5));
                graphics::draw(
                    ctx,
                    &self.assets.player_sprite,
                    param,
                )?;
            }
        }

        graphics::set_canvas(ctx, None);
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
        graphics::draw(ctx, &self.canvas, DrawParam::default()
                       .dest(Point2::new(0.0, 0.0))
                       // .scale(scale),
                       )?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}

impl MainState {}
struct Assets{
    player_sprite: graphics::Image,
    orc_sprite: graphics::Image,
    troll_sprite: graphics::Image,
    sword_sprite: graphics::Image,
    dagger_sprite: graphics::Image,
    shield_sprite: graphics::Image,
    potion_sprite: graphics::Image,
    scroll_sprite: graphics::Image,
    wall_sprite: graphics::Image,
}

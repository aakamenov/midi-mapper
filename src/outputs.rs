use microui_femtovg::microui::*;

const PANEL_NAME: &str = "outputs";

#[derive(Default)]
pub struct State;

impl State {
    pub fn draw(&mut self, ctx: &mut Context) {
        ctx.layout_row(&[-1], -1);
        Panel::new(PANEL_NAME).show(ctx, |_ctx| {

        });
    }
}

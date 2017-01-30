extern crate graphics;
use graphics::{Context, Graphics, types};

use visual::types::*;

pub fn draw<G>(c: &Context, g: &mut G, display: &VisualDisplay)
    where G: Graphics
{
    use visual::types::DisplayType::*;

    match display._type {
        Rect(ref rect) => draw_rect(c, g, rect),
        Text(ref text) => (),
    }
}

pub fn draw_rect<G>(c: &Context, g: &mut G, rect: &RectDisplay)
    where G: Graphics
{
    use graphics::Rectangle;

    let bounds = [rect.x, rect.y, rect.w, rect.h];

    for background in rect.backgrounds.iter() {
        let background_color = match *background {
            Background::Filled(ref color) => color_to_piston(color),
        };

        let r = Rectangle::new(background_color);
        r.draw(bounds, &c.draw_state, c.transform, g);
    }

}

fn color_to_piston(c: &Color) -> graphics::types::Color {
    [c.r, c.g, c.b, c.a]
}

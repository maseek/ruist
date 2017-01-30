use visual::types::*;


// TODO: resolve the visuals from bottom up, then display from up to bottom
// TODO: change and rename this to convert Visual to Displayable, which will
// have calculated layout, size, etc.
pub fn display(def: &VisualDefinition, screen_size: &Sizes) -> VisualDisplay {

    VisualDisplay {
        _type: match def._type {
            DefinitionType::Rect(ref def) => {
                DisplayType::Rect(display_rect(def, screen_size, &OptionSizes::new(screen_size)))
            }
            // TODO: collage Rend with relative and popup
            DefinitionType::Text(ref def) => {
                DisplayType::Text(display_text(def, screen_size, &OptionSizes::new(screen_size)))
            }
        },
        status: def.status.clone(),
    }
}

fn display_child(def: &VisualDefinition,
                 screen_size: &Sizes,
                 parent_size: &OptionSizes)
                 -> VisualDisplay {
    VisualDisplay {
        _type: match def._type {
            DefinitionType::Rect(ref def) => {
                DisplayType::Rect(display_rect(def, screen_size, parent_size))
            }
            // TODO: collage Rend with relative and popup
            DefinitionType::Text(ref def) => {
                DisplayType::Text(display_text(def, screen_size, parent_size))
            }
        },
        status: def.status.clone(),
    }
}

fn display_rect(def: &RectDefinition,
                screen_size: &Sizes,
                parent_size: &OptionSizes)
                -> RectDisplay {
    // TODO borders size
    let maybe_size = try_calc_rect_size(&def.extents, parent_size);

    let nested = display_nested_children(&def.children.nested,
                                         screen_size,
                                         parent_size,
                                         &def.children.nested_flow);

    RectDisplay {
        x: 50.0,
        y: 50.0,
        w: 50.0,
        h: 50.0,
        backgrounds: def.backgrounds.clone(),
        children: Children::new(),
    }
}

fn display_nested_children(defs: &Vec<VisualDefinition>,
                           screen_size: &Sizes,
                           parent_size: &OptionSizes,
                           flow: &Flow)
                           -> Vec<VisualDisplay> {
    let displays: Vec<VisualDisplay> =
        defs.iter().map(|x| display_child(&x, screen_size, parent_size)).collect();

    displays
}

fn display_text(def: &TextDefinition,
                screen_size: &Sizes,
                parent_size: &OptionSizes)
                -> TextDisplay {
    // Display::Text(def, TextDisplay {})
    TextDisplay { text: def.text.clone() }
}

/// Try to calculate rectangle size from its extents.
///
/// If an extent is `Fit`, the size will be unknown until its children are
/// displayed.
fn try_calc_rect_size(extents: &Extents, parent_size: &OptionSizes) -> OptionSizes {
    use visual::types::Extent::*;

    fn calc(extent: &Extent, parent_size: Option<Size>) -> Option<Size> {
        match *extent {
            Fix(size) => Some(size),
            Fit => None,
            Fill(ratio) => parent_size.map(|size| size * ratio),
        }
    }

    OptionSizes {
        w: calc(&extents.w, parent_size.w),
        h: calc(&extents.h, parent_size.h),
    }
}

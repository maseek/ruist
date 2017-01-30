pub type Ratio = f64;
pub type Spacing = f64;
pub type Scalar = f64;

pub struct VisualDefinition {
    pub _type: DefinitionType,
    pub status: Status,
}

pub struct VisualDisplay {
    pub _type: DisplayType,
    pub status: Status,
}

pub enum DefinitionType {
    Rect(RectDefinition),
    Text(TextDefinition),
}

pub enum DisplayType {
    Rect(RectDisplay),
    Text(TextDisplay),
}

#[derive(Clone)]
pub enum Status {
    Enabled,
    Invisible, // still takes up it's space
    Disabled, // doesn't occupy any space
}

pub struct RectDefinition {
    pub extents: Extents,
    pub borders: Vec<Border>,
    /// the last background in the Vec is the foremost
    pub backgrounds: Vec<Background>,
    pub children: Children<VisualDefinition>,
}

pub struct RectDisplay {
    /// absolute position x
    pub x: Scalar,
    /// absolute position y
    pub y: Scalar,
    /// width
    pub w: Scalar,
    /// height
    pub h: Scalar,
    pub backgrounds: Vec<Background>,
    pub children: Children<VisualDisplay>,
}

pub struct Children<T> {
    pub nested_flow: Flow,
    pub nested: Vec<T>,
    pub absolute: Vec<T>,
    pub relative: Vec<T>,
}

pub struct TextDefinition {
    pub text: String,
}

pub struct TextDisplay {
    pub text: String,
}

pub struct Extents {
    /// width
    pub w: Extent,
    /// height
    pub h: Extent,
}

pub enum Extent {
    Fix(Size),
    Fit,
    Fill(Ratio),
}

pub type Size = f64;
// pub enum Size {
// Absolute(f64),
// Relative(f64), // TODO relative to what?
// }
//

pub enum Flow {
    Up(Spacing),
    Down(Spacing),
    Left(Spacing),
    Right(Spacing),
    In,
    Out,
}

pub struct Border {
    thickness: DirProp<Size>,
    color: Color,
}

pub enum DirProp<A> {
    All(A),
    HoriVert(A, A),
    TRBL(A, A, A, A), // top, right, bottom, left
}

#[derive(Clone)]
pub enum Background {
    Filled(Color), /* TODO Textured(String),
                    * TODO Gradient(Gradient), */
}

#[derive(Clone)]
pub struct Color {
    pub r: ColorComponent,
    pub g: ColorComponent,
    pub b: ColorComponent,
    pub a: ColorComponent,
}

/// The type used for color component.
pub type ColorComponent = f32;

pub struct Sizes {
    pub w: Size,
    pub h: Size,
}

pub struct OptionSizes {
    pub w: Option<Size>,
    pub h: Option<Size>,
}

impl<T> Children<T> {
    pub fn new() -> Children<T> {
        Children {
            nested_flow: Flow::Down(0.0),
            nested: Vec::new(),
            absolute: Vec::new(),
            relative: Vec::new(),
        }
    }
}

impl OptionSizes {
    pub fn new(size: &Sizes) -> OptionSizes {
        OptionSizes {
            w: Some(size.w),
            h: Some(size.h),
        }
    }
}

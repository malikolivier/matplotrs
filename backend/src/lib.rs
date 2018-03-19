pub trait Backend {
    type Err;
    fn new() -> Self;
    /// Return figure ID in back-end
    fn new_figure(&mut self, figure: &FigureRepr) -> Result<FigureId, Self::Err>;
    fn clear_figure(&mut self, fig_id: FigureId, figure: &FigureRepr) -> Result<(), Self::Err>;
    fn draw_path(&mut self, fig_id: FigureId, path: &Path) -> Result<(), Self::Err>;
    fn draw_text(&mut self, fig_id: FigureId, text: &Text) -> Result<(), Self::Err>;
    fn draw_image(&mut self, fig_id: FigureId, image: &Image) -> Result<(), Self::Err>;
    /// Iterate over each event in the backend
    fn next_event(&mut self) -> Option<Event>;
    fn save_to_file(&mut self)-> Result<(), Self::Err>;
}

pub struct Path {
    pub points: Vec<(f64, f64)>,
    pub closed: bool,
    pub line_color: Option<(f64, f64, f64, f64)>,
    pub fill_color: Option<(f64, f64, f64, f64)>,
}

pub struct Text {
    pub point: (f64, f64),
    pub text: String,
    pub font_size: f32,
}

pub struct Image {
    /// This ID is guaranteed to be unique. The back-end is free to use it for caching purposes.
    pub id: ImageId,
    pub width: usize,
    pub height: usize,
    pub interpolation: Interpolation,
    /// RGB data (row-major). Each pixel is 3 bytes long (RGB).
    pub data: Vec<u8>,
    /// Position of lower-left corner (in container's coordinates, from -1 to +1)
    pub position: (f64, f64),
    /// Actual size when displayed. The image will be scaled to be exactly this size in the
    /// container's coordinates
    pub size: (f64, f64),
}

pub struct FigureRepr {
    pub title: String,
    pub size: (f64, f64),
    pub dpi: f64,
    pub facecolor: (f64, f64, f64, f64),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FigureId(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ImageId(pub u64);

#[derive(Copy, Clone)]
pub enum Interpolation {
    None,
    Linear,
    Quadratic,
}

#[derive(Debug)]
pub struct Event {
    pub fig_id: FigureId,
    pub e: EventKind
}

#[derive(Debug)]
pub enum EventKind {
    /// Then backend asks a figure to be rendered again
    Render,
    /// Update arguments, containing delta time in seconds
    Update(f64),
    /// Emitted when the backend requests to save a file (mainly used for static backends)
    SaveToFile,
    /// Emitted when a window has been resized to size the following (width, height) in pixels
    Resize(u32, u32),
    /// A window is closed
    Close,
    Click(ClickEvent),
}

#[derive(Debug)]
pub struct ClickEvent {
    pub state: ButtonState,
    pub button: MouseButton,
}

#[derive(Debug)]
pub enum ButtonState {
    Press,
    Release,
}

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

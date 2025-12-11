pub struct AppWebView;
impl AppWebView {
    pub const PEDAL: &str = "pedals-widget";
    pub const STANDINGS: &str = "standings-widget";
    pub const TRACK_MAP: &str = "track-map-widget";
    pub const RELATIVE: &str = "relative-widget";
}

pub const WIDGETS: [&str; 4] = [
    AppWebView::PEDAL,
    AppWebView::STANDINGS,
    AppWebView::TRACK_MAP,
    AppWebView::RELATIVE,
];

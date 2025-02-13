use crate::scrape::format_url;
use std::collections::VecDeque;

#[allow(clippy::single_match_else)]
pub fn standard_starting_urls(force: bool, port: u32) -> VecDeque<String> {
    let mut kb_urls = match crate::req::get_kb_urls(port) {
        Ok(kb_urls) => kb_urls,
        Err(_) if force => {
            log::warn!("Failed to request kb_urls.csv from server");
            vec![]
        }
        Err(_) => {
            eprintln!("Failed to request kb_urls.csv from server");
            std::process::exit(1);
        }
    };
    let home_and_css = ["kb/en/"].into_iter().chain(CSS_LINKS.into_iter());
    kb_urls.extend(home_and_css.map(format_url));
    kb_urls.into()
}

const CSS_LINKS: [&str; 35] = ["static/bootstrap/fonts/glyphicons-halflings-regular.f4769f9bdb74.eot", "static/bootstrap/fonts/glyphicons-halflings-regular.f4769f9bdb74.eot?#iefix", "static/bootstrap/fonts/glyphicons-halflings-regular.448c34a56d69.woff2", "static/bootstrap/fonts/glyphicons-halflings-regular.fa2772327f55.woff", "static/bootstrap/fonts/glyphicons-halflings-regular.e18bbf611f2a.ttf", "static/bootstrap/fonts/glyphicons-halflings-regular.89889688147b.svg#glyphicons_halflingsregular", "static/images/sprites/v3.41e7072cb1db.png", "static/images/sprites/v3.41e7072cb1db.png", "static/images/sprites/v3.41e7072cb1db.png", "static/images/v3/icons/x.e7a75c39df3d.png", "static/images/sprites/v3.41e7072cb1db.png", "static/images/sprites/v3.41e7072cb1db.png", "static/images/sprites/v3.41e7072cb1db.png", "static/images/sprites/v3.41e7072cb1db.png", "static/images/sprites/v3.41e7072cb1db.png", "static/images/sprites/v3.41e7072cb1db.png", "static/images/sprites/v3.41e7072cb1db.png", "static/images/sprites/v3.41e7072cb1db.png", "static/images/jqui/mpkb/images/animated-overlay.2b912f7c0653.gif", "static/images/jqui/mpkb/images/ui-bg_flat_75_ffffff_40x100.3b58b1d86e5e.png", "static/images/jqui/mpkb/images/ui-bg_highlight-soft_75_002b64_1x100.ec4c466249c8.png", "static/images/jqui/mpkb/images/ui-bg_glass_75_e6e6e6_1x400.5e6ae3819582.png", "static/images/jqui/mpkb/images/ui-bg_glass_75_dadada_1x400.d54cef916751.png", "static/images/jqui/mpkb/images/ui-bg_glass_65_ffffff_1x400.c1983b9796ba.png", "static/images/jqui/mpkb/images/ui-bg_glass_55_fbf9ee_1x400.8ba937bb6487.png", "static/images/jqui/mpkb/images/ui-bg_glass_95_fef1ec_1x400.920774680eb8.png", "static/images/jqui/mpkb/images/ui-icons_222222_256x240.a1b3887a86cf.png", "static/images/jqui/mpkb/images/ui-icons_FFFFFF_256x240.e3f4748b19b8.png", "static/images/jqui/mpkb/images/ui-icons_888888_256x240.302ae7a7aed5.png", "static/images/jqui/mpkb/images/ui-icons_454545_256x240.6b29e362591a.png", "static/images/jqui/mpkb/images/ui-icons_454545_256x240.6b29e362591a.png", "static/images/jqui/mpkb/images/ui-icons_2e83ff_256x240.764c37efbf6d.png", "static/images/jqui/mpkb/images/ui-icons_cd0a0a_256x240.5c78585b80fb.png", "static/images/jqui/mpkb/images/ui-bg_flat_0_aaaaaa_40x100.c51d83cafa83.png", "static/images/jqui/mpkb/images/ui-bg_flat_0_aaaaaa_40x100.c51d83cafa83.png"];

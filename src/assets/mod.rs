use arc_util::ui::{
    render::Icon,
    texture::{create_texture2d_from_mem, create_texture2d_view},
};
use arcdps::d3d11_device;
use include_img::include_img;
use log::error;
use once_cell::sync::Lazy;
use windows::Win32::Graphics::{
    Direct3D11::D3D11_USAGE_IMMUTABLE, Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM,
};

fn init_icon(data: &'static [u8]) -> Option<Icon> {
    let device = d3d11_device()?;
    let format = DXGI_FORMAT_R8G8B8A8_UNORM;
    let texture =
        create_texture2d_from_mem(device, data, 32, 32, 32 * 4, format, D3D11_USAGE_IMMUTABLE)
            .map_err(|err| error!("failed to create texture: {err}"))
            .ok()?;
    create_texture2d_view(device, &texture, format)
        .map_err(|err| error!("failed to create texture view: {err}"))
        .ok()
}

pub static FOOD_ICON: Lazy<Option<Icon>> =
    Lazy::new(|| init_icon(&include_img!("./src/assets/food.png", rgba8)));

pub static UTIL_ICON: Lazy<Option<Icon>> =
    Lazy::new(|| init_icon(&include_img!("./src/assets/util.png", rgba8)));

pub static UNKNOWN_ICON: Lazy<Option<Icon>> =
    Lazy::new(|| init_icon(&include_img!("./src/assets/unknown.png", rgba8)));

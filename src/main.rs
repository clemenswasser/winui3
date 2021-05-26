use bindings::Microsoft;
use windows::implement;

#[implement(extend Microsoft::UI::Xaml::Application)]
struct App();

fn main() {}

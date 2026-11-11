use displmgr_gui::display;

fn main() {
    display::list_monitors().expect("failed to list displays");
}

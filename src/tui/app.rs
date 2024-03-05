use cursive::{
    self,
    align::HAlign,
    view::Resizable,
    views::{Dialog, TextView},
    Cursive,
};

pub fn run_tui() {
    let mut cur = cursive::default();

    cur.add_global_callback('q', |s| s.quit());

    main_page(&mut cur);

    cur.run();
}

fn main_page(cur: &mut Cursive) {
    cur.pop_layer();
    cur.add_layer(
        Dialog::around(TextView::new("Press <q> to quit."))
            .title("Automate.. 1/2")
            .title_position(HAlign::Left)
            .button("Next", next_page)
            .button("Quit", |cur| cur.quit()),
    )
}
fn next_page(cur: &mut Cursive) {
    cur.add_layer(
        Dialog::around(TextView::new("Press <q> to quit."))
            .title("the borring.. 2/2")
            .title_position(HAlign::Right)
            .button("Main", main_page)
            .button("Quit", |cur| cur.quit()),
    );
}

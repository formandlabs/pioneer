use cursive::Cursive;
use cursive::views::{Dialog, TextView};
use cursive::CursiveExt;

fn main() {
    let mut siv = Cursive::default();

    siv.add_layer(
        Dialog::new()
            .title("Pioneer CLI")
            .content(TextView::new("No nodes available. Create a new cluster."))
            .button("Create Cluster", |s| {
                s.pop_layer();
                s.add_layer(
                    Dialog::new()
                        .title("Cluster Map")
                        .content(TextView::new(
                            "Map:\n\
                             [ ] [ ] [ ]\n\
                             [ ] [X] [ ]\n\
                             [ ] [ ] [ ]"
                        ))
                        .button("Add Node", |s| {
                            s.add_layer(Dialog::info("Command to join another node: join <node_address>"));
                        })
                        .button("Quit", |s| s.quit()),
                );
            })
            .button("Quit", |s| s.quit()),
    );

    siv.run();
}


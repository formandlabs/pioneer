use cursive::Cursive;
use cursive::views::{Dialog, TextView};
use cursive_testing::ViewTesting;

fn setup_siv() -> Cursive {
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
                            "Map:\n                             . . .\n                             . X .\n                             . . ."
                        ))
                        .button("Add Node", |s| {
                            s.add_layer(Dialog::info("Command to join another node: join <node_address>"));
                        })
                        .button("Quit", |s| s.quit()),
                );
            })
            .button("Quit", |s| s.quit()),
    );

    siv
}

#[test]
fn test_initial_screen() {
    let mut siv = setup_siv();
    siv.step();

    assert!(siv.find_name::<Dialog>("Pioneer CLI").is_some());
    assert!(siv.find_name::<TextView>("No nodes available. Create a new cluster.").is_some());
}

#[test]
fn test_create_cluster() {
    let mut siv = setup_siv();
    siv.step();

    siv.call_on_name("Pioneer CLI", |view: &mut Dialog| {
        view.get_button(0).unwrap().on_event(&mut siv, cursive::event::Event::Key(cursive::event::Key::Enter));
    });
    siv.step();

    assert!(siv.find_name::<Dialog>("Cluster Map").is_some());
    assert!(siv.find_name::<TextView>("Map:\n. . .\n. X .\n. . .").is_some());
}

#[test]
fn test_add_node() {
    let mut siv = setup_siv();
    siv.step();

    siv.call_on_name("Pioneer CLI", |view: &mut Dialog| {
        view.get_button(0).unwrap().on_event(&mut siv, cursive::event::Event::Key(cursive::event::Key::Enter));
    });
    siv.step();

    siv.call_on_name("Cluster Map", |view: &mut Dialog| {
        view.get_button(0).unwrap().on_event(&mut siv, cursive::event::Event::Key(cursive::event::Key::Enter));
    });
    siv.step();

    assert!(siv.find_name::<Dialog>("Command to join another node: join <node_address>").is_some());
}



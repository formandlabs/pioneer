use cursive::Cursive;
use cursive::views::{Dialog, TextView};
use cursive::CursiveExt;

#[derive(Clone, Copy)]
enum NodeState {
    Idle,
    Receiving,
    Sending,
    Draining,
}

fn get_node_symbol(state: NodeState) -> char {
    match state {
        NodeState::Idle => '.',
        NodeState::Receiving => '+',
        NodeState::Sending => 'o',
        NodeState::Draining => '-',
    }
}

fn main() {
    let mut siv = Cursive::default();

    let node_state = NodeState::Idle;

    siv.add_layer(
        Dialog::new()
            .title("Pioneer CLI")
            .content(TextView::new("No nodes available. Create a new cluster."))
            .button("Create Cluster", move |s| {
                s.pop_layer();
                s.add_layer(
                    Dialog::new()
                        .title("Cluster Map")
                        .content(TextView::new(format!(
                            "Map:\n\
                             . . .\n\
                             . {} .\n\
                             . . .",
                            get_node_symbol(node_state)
                        )))
                        .button("Add Node", |s| {
                            s.add_layer(Dialog::info("Command to join another node: join <node_address>"));
                        })
                        .button("Update Node State", |s| {
                            // Example of updating node state
                            let new_state = NodeState::Receiving;
                            s.pop_layer();
                            s.add_layer(
                                Dialog::new()
                                    .title("Cluster Map")
                                    .content(TextView::new(format!(
                                        "Map:\n\
                                         . . .\n\
                                         . {} .\n\
                                         . . .",
                                        get_node_symbol(new_state)
                                    )))
                                    .button("Add Node", |s| {
                                        s.add_layer(Dialog::info("Command to join another node: join <node_address>"));
                                    })
                                    .button("Quit", |s| s.quit()),
                            );
                        })
                        .button("Quit", |s| s.quit()),
                );
            })
            .button("Quit", |s| s.quit()),
    );

    siv.run();
}

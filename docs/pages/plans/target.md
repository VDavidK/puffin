# Language Target

This is an example project for a Git client for reference when
implementing the parser and runtime.

```rust
require "std";

use std.git;

export enum State {
    MainMenu,
    Diff,
    List,
}

// State
let current_state = State.MainMenu;
let diffs = ["Some", "Diffs"];
let commits = [];

// Constants
const menu_id = "menu";
const diff_id = "diff";
const list_id = "list";

// Utility functions
fn refresh_diffs() {
    return git.get_diff_list();
}

fn refresh_commits() {
    return git.get_commit_log();
}

export fn set_state(new_state) {
    current_state = new_state;

    match current_state {
        State.Diff => refresh_diffs(),
        State.List => refresh_commits(),
    }
}

@onclick(MouseLeft)
fn click(key, state, el) {
    const new_state = match el.id {
        menu_id => State.MainMenu,
        diff_id => State.Diff,
        list_id => State.List,
    };
    
    set_state(new_state);
}

layout {
    match current_state {
        State.MainMenu => render menu_layout,
        State.Diff => render diff_layout,
        State.List => ListComponent commits=commits,
        default => render main_menu,
    }
}

layout menu_layout {
    vbox {
        text "Git Tool";
        button click={set_state(State.Diff)} "Diff";
        button click={set_state(State.List)} "List";

        // button click(ev)={subscriber(any_var)} "Button";
        // button click=subscriber "Button";
        // button click=[sub1, sub2] "Button";

        // Not allowed
        // button click(ev)=subscriber "Button";
        // button click(ev)=[sub1, sub2] "Button";
    }
}

layout diff_layout {
    vbox {
        text "Git Tool - Diff";
        for diff in diffs {
            text diff;
        }
    }
}

component ListComponent(commits) {
    let component_state = 0;

    @onkey(ArrowUp)
    fn scroll_up(key) {
        if component_state > 0 {
            component_state--;
        }
    }

    @onkey(ArrowDown)
    fn scroll_down(key) {
        if component_state < commits.len() - 1 {
            component_state++;
        }
    }

    layout {
        vbox {
            text "Git Tool - List";

            for i, commit in commits {
                render commit_layout(i, commit);
            }
        }
    }

    layout commit_layout(i, commit) {
        vbox {
            if i == component_state {
                style text_color=black;
                style background_color=white;
            } else {
                style {
                    text_color=white;
                    background_color=black;
                }
            }

            hbox {
                text commit.who;
                text commit.when;
            }

            text commit.msg;
        }
    }
}

```
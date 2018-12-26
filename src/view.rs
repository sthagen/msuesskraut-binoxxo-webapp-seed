use binoxxo::field::Field;
use binoxxo::rules::{is_board_full, is_board_valid};
use crate::model::*;
use crate::control::{CellPos, Message};
use seed::prelude::*;

macro_rules! table {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Table);
            $ (
                    $part.update(&mut el);
            )*
            el
        }
    };
}

macro_rules! tr {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Tr);
            $ (
                    $part.update(&mut el);
            )*
            el
        }
    };
}

macro_rules! td {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Td);
            $ (
                    $part.update(&mut el);
            )*
            el
        }
    };
}

fn view_field(field: Field) -> El<Message> {
    use seed::*;

    let classes = match field {
        Field::Empty => "fas fa-circle",
        Field::X => "fas fa-times",
        Field::O => "far fa-circle"
    };

    let mut i = i![attrs!{"class" => classes}];
    if Field::Empty == field {
        i.add_style("font-size".into(), "20%".into());
    }
    i
}

fn view_cell(model: &Model, col: usize, row: usize) -> El<Message> {
    use seed::*;

    let field = model.board.get(col, row);
    let editable = model.editable.is_editable(col, row);
    let class = if editable { "guess" } else { "" };
    let id = format!("cell-{}-{}", col, row);
    let size = model.get_size();

    let mut td = td![
        // id is required by engine for correct updates,
        // otherwise "board" gets randomized in NewGame (bug in seed?)
        attrs!{"class" => class.to_string(); "id" => id },
        style!{"width" => format!("{}%", 100.0 / (size as f64))},
        view_field(field),
    ];
    if editable {
        td.listeners.push(simple_ev("click", Message::Toggle(CellPos{col, row})));
    }
    td
}

fn view_row(model: &Model, row: usize) -> El<Message> {
    let size = model.get_size();
    let cells: Vec<El<Message>> = (0..size)
        .map(|col| view_cell(
                model,
                col,
                row))
        .collect();
    tr![cells]
} 

fn view_board(model: &Model) -> El<Message> {
    use seed::*;

    let size = model.get_size();
    let rows: Vec<El<Message>> = (0..size)
        .map(|row| view_row(model, row))
        .collect();
    div![
        attrs!{"id" => "board"},
        if is_board_full(&model.board) {
            let valid = is_board_valid(&model.board);
            let text = if valid {
                "Success!"
            } else {
                "Sorry. Try again."
            };

            div![
                attrs!{
                    "class" => if valid { "alert alert-success" } else { "alert alert-danger" };
                    "id" => "end-game-alert"
                },
                text
            ]
        } else {
            seed::empty()
        },
        table![
            rows
        ]
    ]
}

fn view_difficulty(difficulty: Difficulty) -> El<Message> {
    use seed::*;

    a![
        attrs!{
            "class" => "dropdown-item";
            "href" => "#";
        },
        format!("{}", difficulty),
        simple_ev("click", Message::NewGame(difficulty))
    ]
}

fn view_new_game(difficulty: Difficulty) -> Vec<El<Message>> {
    use seed::*;

    vec!(
        h4![
            attrs!{"id" => "Difficulty-Display"},
            format!("Difficulty: {}", difficulty)
        ],
        div![
            attrs!{"class" => "dropdown"},
            button![
                attrs!{
                    "class" => "btn btn-primary dropdown-toggle";
                    "type" => "button";
                    "id" => "New-Game-Difficulty";
                    "data-toggle" => "dropdown";
                    "aria-haspopup" => "true";
                    "aria-expanded" => "false";
                },
                "New Game"
            ],
            div![
                attrs!{
                    "class" => "dropdown-menu";
                    "aria-labelledby" => "New-Game-Difficulty";
                },
                view_difficulty(Difficulty::Easy),
                view_difficulty(Difficulty::Medium),
                view_difficulty(Difficulty::Hard),
            ]
        ],
    )
}

#[allow(clippy::needless_pass_by_value)]  // signature required by seed
pub fn view(model: Model) -> El<Message> {
    use seed::*;

    div![
        attrs!{"class" => "container"},
        div![
            attrs!{"class" => "row"},
            div![
                attrs!{"class" => "col"},
                h1![ "Let's play Binoxxo"]
            ]
        ],
        div![
            attrs!{"class" => "row"},
            div![
                attrs!{"class" => "cl-xs-8 col-sm-8 col-md-8 col-lg-8"},
                view_board(&model)
            ],
            div![
                attrs!{"class" => "col-xs-4 col-sm-4 col-md-4 col-lg-4"},
                button![
                    attrs!{
                        "class" => "btn btn-secondary";
                        "id" => "clear-board"
                    },
                    "Clear Board",
                    simple_ev("click", Message::Clear)
                ],
                view_new_game(model.difficulty),
                h4!["Rules"],
                ul![
                    li!["Never more than two adjacent Xs or Os"],
                    li!["Every row and column must contain the same number of Xs and Os"],
                    li!["Every row and every column is unique"],
                ]
            ]
        ]
    ]
}
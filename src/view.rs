use crate::control::{CellPos, Message};
use crate::model::*;
use binoxxo::field::Field;
use binoxxo::rules::{is_board_full, is_board_valid};
use fluent_bundle::{FluentBundle, FluentValue};
use seed::prelude::*;
use std::collections::HashMap;

struct ViewBuilder<'a> {
    bundle: FluentBundle<'a>,
    model: &'a Model,
}

impl<'a> ViewBuilder<'a> {
    fn tr(&self, id: &str) -> String {
        self.bundle
            .format(id, None)
            .unwrap_or_else(|| panic!("tr({}) failed", id))
            .0
    }

    fn view_field(&self, field: Field) -> El<Message> {
        use seed::*;

        let classes = match field {
            Field::Empty => "fas fa-circle",
            Field::X => "fas fa-times",
            Field::O => "far fa-circle",
        };

        let mut i = i![class![classes]];
        if Field::Empty == field {
            i.add_style("font-size".into(), "20%".into());
        }
        i
    }

    fn view_cell(&self, col: usize, row: usize) -> El<Message> {
        use seed::*;

        let field = self.model.board.get(col, row);
        let editable = self.model.editable.is_editable(col, row);
        let class_name = if editable { "guess" } else { "" };
        let cell_id = format!("cell-{}-{}", col, row);
        let size = self.model.get_size();

        let mut td = td![
            // id is required by engine for correct updates,
            // otherwise "board" gets randomized in NewGame (bug in seed?)
            class![class_name],
            id!(&cell_id),
            style! {"width" => format!("{}%", 100.0 / (size as f64))},
            self.view_field(field),
        ];
        if editable {
            td.listeners
                .push(simple_ev(Ev::Click, Message::Toggle(CellPos { col, row })));
        }
        td
    }

    fn view_row(&self, row: usize) -> El<Message> {
        use seed::*;

        let size = self.model.get_size();
        let cells: Vec<El<Message>> = (0..size).map(|col| self.view_cell(col, row)).collect();
        tr![cells]
    }

    fn view_difficulty(&self, difficulty: Difficulty) -> El<Message> {
        use seed::*;

        a![
            class!["dropdown-item"],
            attrs! {
                At::Href => "#";
            },
            self.tr(&format!("difficulty-{}", difficulty)),
            simple_ev(Ev::Click, Message::NewGame(difficulty))
        ]
    }

    fn view_new_game_button(&self) -> El<Message> {
        use seed::*;

        let new_game_button = button![
            class!["btn btn-primary dropdown-toggle"],
            id!("New-Game-Difficulty"),
            attrs! {
                At::Type => "button";
                "data-toggle" => "dropdown";
                "aria-haspopup" => "true";
                "aria-expanded" => "false";
            },
            self.tr("new-game")
        ];
        let new_game_levels = div![
            class!["dropdown-menu"],
            attrs! {
                "aria-labelledby" => "New-Game-Difficulty";
            },
            self.view_difficulty(Difficulty::Easy),
            self.view_difficulty(Difficulty::Medium),
            self.view_difficulty(Difficulty::Hard),
        ];

        div![class!["dropdown"], new_game_button, new_game_levels]
    }

    fn view_board(&self) -> Vec<El<Message>> {
        use seed::*;

        let size = self.model.get_size();
        let is_full = is_board_full(&self.model.board);
        let is_valid = is_board_valid(&self.model.board);
        let rows: Vec<El<Message>> = (0..size).map(|row| self.view_row(row)).collect();
        let mut board = vec![
            div![
                id!("board"),
                table![
                    class![if is_full && !is_valid { "error" } else { "" }],
                    rows,
                ]
            ]
        ];
        if is_valid {
            board.push(div![
                id!("success-page"),
                class!("overlay"),
                div![
                    class!("overlay-content"),
                    h1![self.tr("game-won")],
                    self.view_new_game_button()
                ]
            ]);
        }
        board
    }

    fn view_new_game(&self, difficulty: Difficulty) -> Vec<El<Message>> {
        use seed::*;

        let mut difficulty_arg = HashMap::new();
        difficulty_arg.insert(
            "difficulty",
            FluentValue::String(self.tr(&format!("difficulty-{}", difficulty))),
        );

        let text = self
            .bundle
            .format("difficulty-display", Some(&difficulty_arg));
        let diff_header = h4![
            id!("Difficulty-Display"),
            text.unwrap_or_else(|| panic!(
                "tr(difficulty-display[difficulty = {}]) failed",
                difficulty
            ))
            .0
        ];

        vec![
            diff_header,
            self.view_new_game_button()
        ]
    }

    fn view_footer(&self) -> El<Message> {
        use seed::*;

        const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
        const REPO: Option<&'static str> = option_env!("CARGO_PKG_REPOSITORY");

        div![
            class!["row"],
            div![
                class!["col footer"],
                self.tr("app-name"),
                if let Some(url) = REPO {
                    span![
                        " | ",
                        a![
                            attrs! {
                                At::Href => url;
                                At::Rel => "norefferer noopener external";
                                At::Target => "_blank"
                            },
                            "Github"
                        ],
                    ]
                } else {
                    seed::empty()
                },
                format!(
                    " | {}: {}",
                    self.tr("version"),
                    VERSION.unwrap_or(&self.tr("version-unknown"))
                )
            ]
        ]
    }

    pub fn view(&self) -> El<Message> {
        use seed::*;

        let header = div![
            class!["row"],
            div![
                class!["col"],
                div![
                    class!["language-switch"],
                    attrs! {
                        "data-toggle" => "tooltip";
                        "data-placement" => "bottom";
                        At::Title => self.tr("language-toggle");
                    },
                    i![class!["fas fa-language"]],
                    simple_ev(Ev::Click, Message::ToggleLanguage),
                ],
                h1![self.tr("header")],
            ]
        ];
        let board = div![
            class!["cl-xs-8 col-sm-8 col-md-8 col-lg-8"],
            self.view_board()
        ];
        let controls = div![
            class!["col-xs-4 col-sm-4 col-md-4 col-lg-4"],
            button![
                class!["btn btn-secondary"],
                id!("clear-board"),
                self.tr("clear-board"),
                simple_ev("click", Message::Clear)
            ],
            self.view_new_game(self.model.difficulty),
            h4![self.tr("rules-header")],
            ul![
                li![self.tr("rule-1")],
                li![self.tr("rule-2")],
                li![self.tr("rule-3")],
            ]
        ];
        div![
            class!["container"],
            header,
            div![class!["row"], board, controls],
            self.view_footer()
        ]
    }
}

fn build_view(model: &Model) -> ViewBuilder {
    ViewBuilder {
        bundle: model.res_mgr.get_bundle(&model.language.to_string()),
        model,
    }
}

pub fn view(model: &Model) -> impl ElContainer<Message> {
    let vb = build_view(model);
    vb.view()
}

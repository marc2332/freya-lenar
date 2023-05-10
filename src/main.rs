use freya::events::MouseEvent;
use freya::prelude::*;
use lenar::parser::Parser;
use lenar::runtime::Runtime;

fn main() {

    launch_cfg(
        app,
        WindowConfig::<()>::builder()
            .with_width(900.0)
            .with_height(500.0)
            .with_decorations(true)
            .with_transparency(false)
            .with_title("Editor")
            .build(),
    );
}

fn app(cx: Scope) -> Element {
    use_init_default_theme(cx);
    render!(Body {})
}

#[allow(non_snake_case)]
fn Body(cx: Scope) -> Element {
    let theme = use_theme(cx);
    let output = use_state(cx, String::new);
    let editable = use_editable(
        cx,
        || {
            EditableConfig::new("let func = fn() { \"hi\" }; \nfunc()".to_string())
        },
        EditableMode::SingleLineMultipleEditors,
    );

    let theme = theme.read();
    let cursor_attr = editable.cursor_attr(cx);
    let editor = editable.editor().clone();

    let onclick = {
        to_owned![editable];
        move |_: MouseEvent| {
            editable.process_event(&EditableEvent::Click);
        }
    };

    let onkeydown = {
        to_owned![editable];
        move |e: KeyboardEvent| {
            editable.process_event(&EditableEvent::KeyDown(e.data));
        }
    };

    let run = {
        to_owned![editable, output];
        move |_| {
            let code = editable.editor().current().to_string();
            let parser = Parser::new(&code).wrap();

            let res = Runtime::evaluate(&parser);
            if let Ok(res) = res {
                output.set(res.to_string());
            } else if let Err(err) = res {
                output.set(format!("Error -> {err:?}"));
            }
            
            
        }
    };

    render!(
        rect {
            width: "100%",
            height: "100%",
            background: "{theme.body.background}",
            rect {
                width: "100%",
                height: "calc(100% - 40)",
                onkeydown: onkeydown,
                cursor_reference: cursor_attr,
                onglobalclick: onclick,
                rect {
                    width: "100%",
                    height: "100%",
                    direction: "horizontal",
                    VirtualScrollView {
                        width: "50%",
                        height: "100%",
                        show_scrollbar: true,
                        length: editor.len_lines(),
                        item_size: 35.0,
                        builder_values: editable.clone(),
                        builder: Box::new(move |(key, line_index, cx, values)| {
                            let editable = values.as_ref().unwrap();
                            let editor = editable.editor();
                            let line = editor.line(line_index).unwrap();

                            let is_line_selected = editor.cursor_row() == line_index;

                            // Only show the cursor in the active line
                            let character_index = if is_line_selected {
                                editor.cursor_col().to_string()
                            } else {
                                "none".to_string()
                            };

                            // Only highlight the active line
                            let line_background = if is_line_selected {
                                "rgb(37, 37, 37)"
                            } else {
                                ""
                            };

                            let onmousedown = {
                                to_owned![editable];
                                move |e: MouseEvent| {
                                    editable.process_event(&EditableEvent::MouseDown(e.data, line_index));
                                }
                            };

                            let onmouseover = {
                                to_owned![editable];
                                move |e: MouseEvent| {
                                    editable.process_event(&EditableEvent::MouseOver(e.data, line_index));
                                }
                            };

                            let highlights = editable.highlights_attr(&cx, line_index);

                            rsx! {
                                rect {
                                    key: "{key}",
                                    width: "100%",
                                    height: "35",
                                    display: "center",
                                    background: "{line_background}",
                                    rect {
                                        direction: "horizontal",
                                        width: "100%",
                                        height: "25",
                                        rect {
                                            width: "30",
                                            height: "100%",
                                            display: "center",
                                            direction: "horizontal",
                                            label {
                                                font_size: "15",
                                                color: "rgb(200, 200, 200)",
                                                "{line_index + 1} "
                                            }
                                        }
                                        paragraph {
                                            height: "100%",
                                            width: "100%",
                                            cursor_index: "{character_index}",
                                            cursor_color: "white",
                                            max_lines: "1",
                                            cursor_mode: "editable",
                                            cursor_id: "{line_index}",
                                            onmousedown: onmousedown,
                                            onmouseover: onmouseover,
                                            highlights: highlights,
                                            text {
                                                color: "rgb(240, 240, 240)",
                                                font_size: "15",
                                                "{line}"
                                            }
                                        }
                                    }
                                }
                            }
                        })
                    }
                    rect {
                        width: "50%",
                        height: "100%",
                        padding: "15",
                        label {
                            color: "white",
                            "{output}"
                        }
                    }

                }
                rect {
                    display: "both",
                    direction: "horizontal",
                    height: "40",
                    width: "100%",
                    Button {
                        onclick: run,
                        label {
                            width: "100%",
                            align: "center",
                            "Run"
                        }
                    }
                }
            }
        }
    )
}

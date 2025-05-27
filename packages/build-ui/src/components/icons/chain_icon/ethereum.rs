use bdk::prelude::*;

#[component]
pub fn EthereumIcon() -> Element {
    rsx! {

        svg {
            clip_rule: "evenodd",
            fill_rule: "evenodd",
            height: "100%",
            image_rendering: "optimizeQuality",
            shape_rendering: "geometricPrecision",
            "space": "preserve",
            text_rendering: "geometricPrecision",
            version: "1.1",
            view_box: "0 0 784.37 1277.39",
            width: "100%",
            "xlink": "http://www.w3.org/1999/xlink",
            xmlns: "http://www.w3.org/2000/svg",
            "xodm": "http://www.corel.com/coreldraw/odm/2003",
            g { id: "Layer_x0020_1",
                meta { data: "false", id: "CorelCorpID_0Corel-Layer" }
                g { id: "_1421394342400",
                    g {
                        polygon {
                            fill: "#343434",
                            fill_rule: "nonzero",
                            points: "392.07,0 383.5,29.11 383.5,873.74 392.07,882.29 784.13,650.54 ",
                        }
                        polygon {
                            fill: "#8C8C8C",
                            fill_rule: "nonzero",
                            points: "392.07,0 -0,650.54 392.07,882.29 392.07,472.33 ",
                        }
                        polygon {
                            fill: "#3C3C3B",
                            fill_rule: "nonzero",
                            points: "392.07,956.52 387.24,962.41 387.24,1263.28 392.07,1277.38 784.37,724.89 ",
                        }
                        polygon {
                            fill: "#8C8C8C",
                            fill_rule: "nonzero",
                            points: "392.07,1277.38 392.07,956.52 -0,724.89 ",
                        }
                        polygon {
                            fill: "#141414",
                            fill_rule: "nonzero",
                            points: "392.07,882.29 784.13,650.54 392.07,472.33 ",
                        }
                        polygon {
                            fill: "#393939",
                            fill_rule: "nonzero",
                            points: "0,650.54 392.07,882.29 392.07,472.33 ",
                        }
                    }
                }
            }
        }
    }
}

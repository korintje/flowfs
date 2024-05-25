use dioxus::prelude::*;

pub fn tree() -> Element {
    rsx! {
        ul { class: "menu menu-xs bg-base-0 rounded-lg max-w-xs w-full",
            li {
                a {
                    svg {
                        "viewBox": "0 0 24 24",
                        "stroke": "currentColor",
                        "fill": "none",
                        "stroke-width": "1.5",
                        "xmlns": "http://www.w3.org/2000/svg",
                        class: "w-4 h-4",
                        path {
                            "stroke-linecap": "round",
                            "d": "M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z",
                            "stroke-linejoin": "round"
                        }
                    }
                    "\n    resume.pdf\n  "
                }
            }
            li {
                details { open: "false",
                    summary {
                        svg {
                            "fill": "none",
                            "viewBox": "0 0 24 24",
                            "stroke": "currentColor",
                            "xmlns": "http://www.w3.org/2000/svg",
                            "stroke-width": "1.5",
                            class: "w-4 h-4",
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                "d": "M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z"
                            }
                        }
                        "\n        My Files\n      "
                    }
                    ul {
                        li {
                            a {
                                svg {
                                    "viewBox": "0 0 24 24",
                                    "fill": "none",
                                    "stroke": "currentColor",
                                    "stroke-width": "1.5",
                                    "xmlns": "http://www.w3.org/2000/svg",
                                    class: "w-4 h-4",
                                    path {
                                        "stroke-linejoin": "round",
                                        "stroke-linecap": "round",
                                        "d": "M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z"
                                    }
                                }
                                "\n          Project-final.psd\n        "
                            }
                        }
                        li {
                            a {
                                svg {
                                    "stroke-width": "1.5",
                                    "xmlns": "http://www.w3.org/2000/svg",
                                    "stroke": "currentColor",
                                    "viewBox": "0 0 24 24",
                                    "fill": "none",
                                    class: "w-4 h-4",
                                    path {
                                        "stroke-linejoin": "round",
                                        "d": "M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z",
                                        "stroke-linecap": "round"
                                    }
                                }
                                "\n          Project-final-2.psd\n        "
                            }
                        }
                        li {
                            details { open: "false",
                                summary {
                                    svg {
                                        "stroke": "currentColor",
                                        "fill": "none",
                                        "viewBox": "0 0 24 24",
                                        "xmlns": "http://www.w3.org/2000/svg",
                                        "stroke-width": "1.5",
                                        class: "w-4 h-4",
                                        path {
                                            "d": "M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z",
                                            "stroke-linejoin": "round",
                                            "stroke-linecap": "round"
                                        }
                                    }
                                    "\n              Images\n            "
                                }
                                ul {
                                    li {
                                        a {
                                            svg {
                                                "stroke": "currentColor",
                                                "fill": "none",
                                                "stroke-width": "1.5",
                                                "viewBox": "0 0 24 24",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                class: "w-4 h-4",
                                                path {
                                                    "d": "M2.25 15.75l5.159-5.159a2.25 2.25 0 013.182 0l5.159 5.159m-1.5-1.5l1.409-1.409a2.25 2.25 0 013.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 001.5-1.5V6a1.5 1.5 0 00-1.5-1.5H3.75A1.5 1.5 0 002.25 6v12a1.5 1.5 0 001.5 1.5zm10.5-11.25h.008v.008h-.008V8.25zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z",
                                                    "stroke-linecap": "round",
                                                    "stroke-linejoin": "round"
                                                }
                                            }
                                            "\n                Screenshot1.png\n              "
                                        }
                                    }
                                    li {
                                        a {
                                            svg {
                                                "viewBox": "0 0 24 24",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "stroke": "currentColor",
                                                "fill": "none",
                                                "stroke-width": "1.5",
                                                class: "w-4 h-4",
                                                path {
                                                    "d": "M2.25 15.75l5.159-5.159a2.25 2.25 0 013.182 0l5.159 5.159m-1.5-1.5l1.409-1.409a2.25 2.25 0 013.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 001.5-1.5V6a1.5 1.5 0 00-1.5-1.5H3.75A1.5 1.5 0 002.25 6v12a1.5 1.5 0 001.5 1.5zm10.5-11.25h.008v.008h-.008V8.25zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z",
                                                    "stroke-linecap": "round",
                                                    "stroke-linejoin": "round"
                                                }
                                            }
                                            "\n                Screenshot2.png\n              "
                                        }
                                    }
                                    li {
                                        details { open: "false",
                                            summary {
                                                svg {
                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                    "stroke": "currentColor",
                                                    "fill": "none",
                                                    "stroke-width": "1.5",
                                                    "viewBox": "0 0 24 24",
                                                    class: "w-4 h-4",
                                                    path {
                                                        "d": "M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z",
                                                        "stroke-linejoin": "round",
                                                        "stroke-linecap": "round"
                                                    }
                                                }
                                                "Others"
                                            }
                                            ul {
                                                li {
                                                    a {
                                                        svg {
                                                            "fill": "none",
                                                            "viewBox": "0 0 24 24",
                                                            "xmlns": "http://www.w3.org/2000/svg",
                                                            "stroke": "currentColor",
                                                            "stroke-width": "1.5",
                                                            class: "w-4 h-4",
                                                            path {
                                                                "stroke-linejoin": "round",
                                                                "d": "M2.25 15.75l5.159-5.159a2.25 2.25 0 013.182 0l5.159 5.159m-1.5-1.5l1.409-1.409a2.25 2.25 0 013.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 001.5-1.5V6a1.5 1.5 0 00-1.5-1.5H3.75A1.5 1.5 0 002.25 6v12a1.5 1.5 0 001.5 1.5zm10.5-11.25h.008v.008h-.008V8.25zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z",
                                                                "stroke-linecap": "round"
                                                            }
                                                        }
                                                        "\n                      Screenshot3.png\n                    "
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            li {
                a {
                    svg {
                        "stroke-width": "1.5",
                        "stroke": "currentColor",
                        "viewBox": "0 0 24 24",
                        "fill": "none",
                        "xmlns": "http://www.w3.org/2000/svg",
                        class: "w-4 h-4",
                        path {
                            "d": "M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z",
                            "stroke-linejoin": "round",
                            "stroke-linecap": "round"
                        }
                    }
                    "\n    reports-final-2.pdf\n  "
                }
            }
        }
    }
}
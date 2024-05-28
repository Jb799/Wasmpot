(this.webpackJsonp = this.webpackJsonp || []).push([[826], {
    LLbv: function(t, e, n) {
        "use strict";
        n.d(e, "a", (function() {
            return G
        }
        ));
        var i = n("0zRR")
          , r = n("MtBe")
          , a = n("h3Ey")
          , l = n("TjC/")
          , o = n("8ENL")
          , c = n("t8l0")
          , s = n("EGUT")
          , u = n("qTlp")
          , d = n("tTwu")
          , b = n("BrvI")
          , f = n("NSGy")
          , g = n("Ddgg")
          , h = n("7bmO")
          , p = n("aM4G")
          , v = n("RhHz");
        function y(t, e) {
            var n = Object.keys(t);
            if (Object.getOwnPropertySymbols) {
                var i = Object.getOwnPropertySymbols(t);
                e && (i = i.filter((function(e) {
                    return Object.getOwnPropertyDescriptor(t, e).enumerable
                }
                ))),
                n.push.apply(n, i)
            }
            return n
        }
        function m(t) {
            for (var e = 1; e < arguments.length; e++) {
                var n = null != arguments[e] ? arguments[e] : {};
                e % 2 ? y(Object(n), !0).forEach((function(e) {
                    O(t, e, n[e])
                }
                )) : Object.getOwnPropertyDescriptors ? Object.defineProperties(t, Object.getOwnPropertyDescriptors(n)) : y(Object(n)).forEach((function(e) {
                    Object.defineProperty(t, e, Object.getOwnPropertyDescriptor(n, e))
                }
                ))
            }
            return t
        }
        function O(t, e, n) {
            return e in t ? Object.defineProperty(t, e, {
                value: n,
                enumerable: !0,
                configurable: !0,
                writable: !0
            }) : t[e] = n,
            t
        }
        var j = "__BV_Tooltip__"
          , _ = {
            focus: !0,
            hover: !0,
            click: !0,
            blur: !0,
            manual: !0
        }
          , w = /^html$/i
          , k = /^noninteractive$/i
          , C = /^nofade$/i
          , P = /^(auto|top(left|right)?|bottom(left|right)?|left(top|bottom)?|right(top|bottom)?)$/i
          , x = /^(window|viewport|scrollParent)$/i
          , K = /^d\d+$/i
          , $ = /^ds\d+$/i
          , E = /^dh\d+$/i
          , L = /^o-?\d+$/i
          , S = /^v-.+$/i
          , T = /\s+/
          , M = function(t, e, n) {
            if (r.g) {
                var y = function(t, e) {
                    var n = {
                        title: void 0,
                        trigger: "",
                        placement: "top",
                        fallbackPlacement: "flip",
                        container: !1,
                        animation: !0,
                        offset: 0,
                        id: null,
                        html: !1,
                        interactive: !0,
                        disabled: !1,
                        delay: Object(c.b)(i.ib, "delay", 50),
                        boundary: String(Object(c.b)(i.ib, "boundary", "scrollParent")),
                        boundaryPadding: Object(g.c)(Object(c.b)(i.ib, "boundaryPadding", 5), 0),
                        variant: Object(c.b)(i.ib, "variant"),
                        customClass: Object(c.b)(i.ib, "customClass")
                    };
                    if (Object(b.m)(t.value) || Object(b.g)(t.value) || Object(b.e)(t.value) ? n.title = t.value : Object(b.j)(t.value) && (n = m(m({}, n), t.value)),
                    Object(b.n)(n.title)) {
                        var r = o.d ? e.props : (e.data || {}).attrs;
                        n.title = r && !Object(b.o)(r.title) ? r.title : void 0
                    }
                    Object(b.j)(n.delay) || (n.delay = {
                        show: Object(g.c)(n.delay, 0),
                        hide: Object(g.c)(n.delay, 0)
                    }),
                    t.arg && (n.container = "#".concat(t.arg)),
                    Object(h.h)(t.modifiers).forEach((function(t) {
                        if (w.test(t))
                            n.html = !0;
                        else if (k.test(t))
                            n.interactive = !1;
                        else if (C.test(t))
                            n.animation = !1;
                        else if (P.test(t))
                            n.placement = t;
                        else if (x.test(t))
                            t = "scrollparent" === t ? "scrollParent" : t,
                            n.boundary = t;
                        else if (K.test(t)) {
                            var e = Object(g.c)(t.slice(1), 0);
                            n.delay.show = e,
                            n.delay.hide = e
                        } else
                            $.test(t) ? n.delay.show = Object(g.c)(t.slice(2), 0) : E.test(t) ? n.delay.hide = Object(g.c)(t.slice(2), 0) : L.test(t) ? n.offset = Object(g.c)(t.slice(1), 0) : S.test(t) && (n.variant = t.slice(2) || null)
                    }
                    ));
                    var a = {};
                    return Object(l.b)(n.trigger || "").filter(u.a).join(" ").trim().toLowerCase().split(T).forEach((function(t) {
                        _[t] && (a[t] = !0)
                    }
                    )),
                    Object(h.h)(t.modifiers).forEach((function(t) {
                        t = t.toLowerCase(),
                        _[t] && (a[t] = !0)
                    }
                    )),
                    n.trigger = Object(h.h)(a).join(" "),
                    "blur" === n.trigger && (n.trigger = "focus"),
                    n.trigger || (n.trigger = "hover focus"),
                    n
                }(e, n);
                if (!t[j]) {
                    var O = Object(d.a)(n, e);
                    t[j] = Object(p.a)(O, v.a, {
                        _scopeId: Object(s.a)(O, void 0)
                    }),
                    t[j].__bv_prev_data__ = {},
                    t[j].$on(a.L, (function() {
                        Object(b.e)(y.title) && t[j].updateData({
                            title: y.title(t)
                        })
                    }
                    ))
                }
                var M = {
                    title: y.title,
                    triggers: y.trigger,
                    placement: y.placement,
                    fallbackPlacement: y.fallbackPlacement,
                    variant: y.variant,
                    customClass: y.customClass,
                    container: y.container,
                    boundary: y.boundary,
                    delay: y.delay,
                    offset: y.offset,
                    noFade: !y.animation,
                    id: y.id,
                    interactive: y.interactive,
                    disabled: y.disabled,
                    html: y.html
                }
                  , G = t[j].__bv_prev_data__;
                if (t[j].__bv_prev_data__ = M,
                !Object(f.a)(M, G)) {
                    var z = {
                        target: t
                    };
                    Object(h.h)(M).forEach((function(e) {
                        M[e] !== G[e] && (z[e] = "title" === e && Object(b.e)(M[e]) ? M[e](t) : M[e])
                    }
                    )),
                    t[j].updateData(z)
                }
            }
        }
          , G = {
            bind: function(t, e, n) {
                M(t, e, n)
            },
            componentUpdated: function(t, e, n) {
                Object(o.e)((function() {
                    M(t, e, n)
                }
                ))
            },
            unbind: function(t) {
                !function(t) {
                    t[j] && (t[j].$destroy(),
                    t[j] = null),
                    delete t[j]
                }(t)
            }
        }
    },
    exIq: function(t, e, n) {
        "use strict";
        n.r(e);
        n("3UXl"),
        n("iyoE"),
        n("FMw2"),
        n("UezY"),
        n("z6RN"),
        n("hG7+");
        var i = n("1cpz")
          , r = n("iN9h")
          , a = n("MV2A")
          , l = n("30su")
          , o = n("/lV4")
          , c = n("3twG")
          , s = n("d08M")
          , u = (n("RFHG"),
        n("xuo1"),
        n("ZzK0"),
        n("BzOf"),
        n("xPX6"));
        var d = {
            functional: !0,
            props: {
                shortcuts: {
                    type: Array,
                    required: !0
                }
            },
            render(t, e) {
                const n = function() {
                    var t, e;
                    const n = {
                        up: "↑",
                        down: "↓",
                        left: "←",
                        right: "→",
                        ctrl: Object(o.g)("KeyboardKey|Ctrl"),
                        shift: Object(o.g)("KeyboardKey|Shift"),
                        enter: Object(o.g)("KeyboardKey|Enter"),
                        esc: Object(o.g)("KeyboardKey|Esc"),
                        command: "⌘",
                        option: null !== (t = window.gl) && void 0 !== t && null !== (e = t.client) && void 0 !== e && e.isMac ? "⌥" : Object(o.g)("KeyboardKey|Alt")
                    };
                    return n.meta = n.command,
                    n.alt = n.option,
                    n.mod = Object(u.a)(!0),
                    n
                }()
                  , {staticClass: i} = e.data
                  , r = e.props.shortcuts.reduce((function(e, i, r) {
                    var a, l;
                    if ((null === (a = window.gl) || void 0 === a || null === (l = a.client) || void 0 === l || !l.isMac) && (i.includes("command") || i.includes("meta")))
                        return e;
                    const c = i.split(/([ +])/);
                    return 0 !== r && e.length && (e.push(` ${Object(o.a)("or")} `),
                    c.length > 1 && e.push(t("br"))),
                    c.forEach((function(i) {
                        if ("+" === i)
                            e.push(" + ");
                        else if (" " === i)
                            e.push(` ${Object(o.a)("then")} `);
                        else {
                            var r;
                            e.push(t("kbd", {}, [null !== (r = n[i]) && void 0 !== r ? r : i]))
                        }
                    }
                    )),
                    e
                }
                ), []);
                return t("div", {
                    staticClass: i
                }, r)
            }
        }
          , b = n("tBpV")
          , f = Object(b.a)(d, void 0, void 0, !1, null, null, null).exports
          , g = {
            components: {
                GlModal: i.a,
                GlSearchBoxByType: r.a,
                GlLink: a.a,
                GlSprintf: l.a,
                Shortcut: f
            },
            data: ()=>({
                searchTerm: ""
            }),
            computed: {
                filteredKeybindings() {
                    if (!this.searchTerm)
                        return s.vb;
                    const t = this.searchTerm.toLocaleLowerCase();
                    return s.vb.map((function(e) {
                        return e.name.toLocaleLowerCase().includes(t) ? e : {
                            ...e,
                            keybindings: e.keybindings.filter((function(e) {
                                return e.description.toLocaleLowerCase().includes(t)
                            }
                            ))
                        }
                    }
                    )).filter((function(t) {
                        return t.keybindings.length
                    }
                    ))
                },
                absoluteUserPreferencesPath: ()=>Object(c.C)(gon.relative_url_root || "/", "/-/profile/preferences")
            },
            i18n: {
                title: Object(o.a)("Keyboard shortcuts"),
                search: Object(o.g)("KeyboardShortcuts|Search keyboard shortcuts"),
                noMatch: Object(o.g)("KeyboardShortcuts|No shortcuts matched your search")
            }
        }
          , h = Object(b.a)(g, (function() {
            var t = this
              , e = t._self._c;
            return e("gl-modal", {
                attrs: {
                    "modal-id": "keyboard-shortcut-modal",
                    size: "lg",
                    title: t.$options.i18n.title,
                    "data-testid": "modal-shortcuts",
                    "body-class": "shortcut-help-body gl-p-0!",
                    visible: !0,
                    "hide-footer": !0
                },
                on: {
                    hidden: function(e) {
                        return t.$emit("hidden")
                    }
                }
            }, [e("div", {
                staticClass: "gl-sticky gl-top-0 gl-py-5 gl-px-5 gl-display-flex gl-align-items-center gl-bg-white"
            }, [e("gl-search-box-by-type", {
                staticClass: "gl-w-half gl-mr-3",
                attrs: {
                    "aria-label": t.$options.i18n.search
                },
                model: {
                    value: t.searchTerm,
                    callback: function(e) {
                        t.searchTerm = "string" == typeof e ? e.trim() : e
                    },
                    expression: "searchTerm"
                }
            }), t._v(" "), e("span", [e("gl-sprintf", {
                attrs: {
                    message: t.__("Enable or disable keyboard shortcuts in your %{linkStart}user preferences%{linkEnd}.")
                },
                scopedSlots: t._u([{
                    key: "link",
                    fn: function({content: n}) {
                        return [e("gl-link", {
                            attrs: {
                                href: t.absoluteUserPreferencesPath
                            }
                        }, [t._v("\n            " + t._s(n) + "\n          ")])]
                    }
                }])
            })], 1)], 1), t._v(" "), 0 === t.filteredKeybindings.length ? e("div", {
                staticClass: "gl-px-5"
            }, [t._v("\n    " + t._s(t.$options.i18n.noMatch) + "\n  ")]) : e("div", {
                staticClass: "shortcut-help-container gl-mt-8 gl-px-5 gl-pb-5"
            }, t._l(t.filteredKeybindings, (function(n) {
                return e("section", {
                    key: n.id,
                    staticClass: "shortcut-help-mapping gl-mb-4"
                }, [e("strong", {
                    staticClass: "shortcut-help-mapping-title gl-w-half gl-display-inline-block"
                }, [t._v("\n        " + t._s(n.name) + "\n      ")]), t._v(" "), t._l(n.keybindings, (function(n) {
                    return e("div", {
                        key: n.id,
                        staticClass: "gl-display-flex gl-align-items-center"
                    }, [e("shortcut", {
                        staticClass: "gl-w-40p gl-flex-shrink-0 gl-text-right gl-pr-4",
                        attrs: {
                            shortcuts: n.defaultKeys
                        }
                    }), t._v(" "), e("div", {
                        staticClass: "gl-w-half gl-flex-shrink-0 gl-flex-grow-1"
                    }, [t._v("\n          " + t._s(n.description) + "\n        ")])], 1)
                }
                ))], 2)
            }
            )), 0)])
        }
        ), [], !1, null, null, null);
        e.default = h.exports
    },
    tTwu: function(t, e, n) {
        "use strict";
        n.d(e, "a", (function() {
            return r
        }
        ));
        var i = n("8ENL")
          , r = function(t, e) {
            return i.d ? e.instance : t.context
        }
    },
    xPX6: function(t, e, n) {
        "use strict";
        n.d(e, "a", (function() {
            return i
        }
        ));
        const i = function(t=!1) {
            var e, n;
            const i = "Ctrl" + (t ? "" : "+");
            return null !== (e = window.gl) && void 0 !== e && null !== (n = e.client) && void 0 !== n && n.isMac ? "⌘" : i
        }
    }
}]);
//# sourceMappingURL=826.1565d49f.chunk.js.map

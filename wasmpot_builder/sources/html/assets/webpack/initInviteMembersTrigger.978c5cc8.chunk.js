(this.webpackJsonp = this.webpackJsonp || []).push([[425], {
    "6fAW": function(t, e, n) {
        "use strict";
        n.r(e),
        n.d(e, "default", (function() {
            return i
        }
        ));
        n("ZzK0"),
        n("z6RN"),
        n("BzOf");
        var r = n("ewH8")
          , o = n("LgEk");
        function i() {
            const t = document.querySelectorAll(".js-invite-members-trigger");
            return !!t && t.forEach((function(t) {
                return new r.default({
                    el: t,
                    name: "InviteMembersTriggerRoot",
                    render: function(e) {
                        return e(o.a, {
                            props: {
                                ...t.dataset
                            }
                        })
                    }
                })
            }
            ))
        }
    },
    "7xOh": function(t, e, n) {
        "use strict";
        n("Tznw"),
        n("IYH6"),
        n("6yen"),
        n("OeRx"),
        n("l/dT"),
        n("RqS2"),
        n("Zy7a"),
        n("cjZU"),
        n("OAhk"),
        n("X42P"),
        n("mHhP"),
        n("fn0I"),
        n("UB/6"),
        n("imhG"),
        n("UezY"),
        n("z6RN"),
        n("hG7+"),
        n("ZzK0"),
        n("BzOf");
        class r {
            constructor() {
                this.$_all = new Map
            }
            dispose() {
                this.$_all.clear()
            }
            $on(t, e) {
                const n = this.$_all.get(t);
                n && n.push(e) || this.$_all.set(t, [e])
            }
            $off(t, e) {
                const n = this.$_all.get(t) || []
                  , r = e ? n.filter((function(t) {
                    return t !== e
                }
                )) : [];
                r.length ? this.$_all.set(t, r) : this.$_all.delete(t)
            }
            $once(t, e) {
                var n = this;
                const r = function(...o) {
                    n.$off(t, r),
                    e(...o)
                };
                this.$on(t, r)
            }
            $emit(t, ...e) {
                (this.$_all.get(t) || []).forEach((function(t) {
                    t(...e)
                }
                ))
            }
        }
        e.a = function() {
            return new r
        }
    },
    "7z1+": function(t, e, n) {
        "use strict";
        n.d(e, "a", (function() {
            return l
        }
        )),
        n.d(e, "b", (function() {
            return a
        }
        )),
        n.d(e, "c", (function() {
            return s
        }
        )),
        n.d(e, "d", (function() {
            return c
        }
        ));
        var r = n("KFC0")
          , o = n.n(r)
          , i = n("BglX");
        const s = t=>Boolean(t) && (t=>{
            var e;
            return (null == t || null === (e = t.text) || void 0 === e ? void 0 : e.length) > 0 && !Array.isArray(null == t ? void 0 : t.items)
        }
        )(t)
          , a = t=>Boolean(t) && Array.isArray(t.items) && Boolean(t.items.length) && t.items.every(s)
          , c = t=>t.every(s) || t.every(a)
          , l = t=>{
            let {default: e} = t;
            if (!o()(e))
                return !1;
            const n = e();
            if (!Array.isArray(n))
                return !1;
            const r = n.filter(t=>t.tag);
            return r.length && r.every(t=>{
                return e = t,
                [i.c, i.b].includes(null === (n = e.type) || void 0 === n ? void 0 : n.name) || "li" === e.type || (t=>{
                    var e, n;
                    return Boolean(t) && (n = (null === (e = t.componentOptions) || void 0 === e ? void 0 : e.tag) || t.tag,
                    ["gl-disclosure-dropdown-group", "gl-disclosure-dropdown-item", "li"].includes(n))
                }
                )(t);
                var e, n
            }
            )
        }
    },
    A2UA: function(t, e, n) {
        "use strict";
        var r = n("7xOh");
        e.a = Object(r.a)()
    },
    BglX: function(t, e, n) {
        "use strict";
        n.d(e, "a", (function() {
            return i
        }
        )),
        n.d(e, "b", (function() {
            return o
        }
        )),
        n.d(e, "c", (function() {
            return r
        }
        ));
        const r = "GlDisclosureDropdownItem"
          , o = "GlDisclosureDropdownGroup"
          , i = {
            top: "top",
            bottom: "bottom"
        }
    },
    LgEk: function(t, e, n) {
        "use strict";
        var r = n("4lAS")
          , o = n("MV2A")
          , i = n("wP8z")
          , s = n("XiQx")
          , a = n("/lV4")
          , c = n("A2UA")
          , l = n("i7S8")
          , u = {
            components: {
                GlButton: r.a,
                GlLink: o.a,
                GlDropdownItem: i.a,
                GlDisclosureDropdownItem: s.b
            },
            props: {
                displayText: {
                    type: String,
                    required: !1,
                    default: Object(a.g)("InviteMembers|Invite team members")
                },
                icon: {
                    type: String,
                    required: !1,
                    default: ""
                },
                classes: {
                    type: String,
                    required: !1,
                    default: ""
                },
                variant: {
                    type: String,
                    required: !1,
                    default: void 0
                },
                triggerSource: {
                    type: String,
                    required: !0
                },
                triggerElement: {
                    type: String,
                    required: !1,
                    default: "button"
                }
            },
            computed: {
                componentAttributes() {
                    return {
                        class: this.classes,
                        "data-testid": "invite-members-button"
                    }
                },
                item() {
                    return {
                        text: this.displayText
                    }
                },
                isButtonTrigger() {
                    return this.triggerElement === l.A
                },
                isWithEmojiTrigger() {
                    return this.triggerElement === l.D
                },
                isDropdownWithEmojiTrigger() {
                    return this.triggerElement === l.C
                },
                isDisclosureTrigger() {
                    return this.triggerElement === l.B
                }
            },
            methods: {
                openModal() {
                    c.a.$emit("openModal", {
                        source: this.triggerSource
                    })
                },
                handleDisclosureDropdownAction() {
                    this.openModal(),
                    this.$emit("modal-opened")
                }
            }
        }
          , d = n("tBpV")
          , m = Object(d.a)(u, (function() {
            var t = this
              , e = t._self._c;
            return t.isButtonTrigger ? e("gl-button", t._b({
                attrs: {
                    variant: t.variant,
                    icon: t.icon
                },
                on: {
                    click: t.openModal
                }
            }, "gl-button", t.componentAttributes, !1), [t._v("\n  " + t._s(t.displayText) + "\n")]) : t.isWithEmojiTrigger ? e("gl-link", t._b({
                on: {
                    click: t.openModal
                }
            }, "gl-link", t.componentAttributes, !1), [t._v("\n  " + t._s(t.displayText) + "\n  "), e("gl-emoji", {
                staticClass: "gl-vertical-align-baseline gl-reset-font-size gl-mr-1",
                attrs: {
                    "data-name": t.icon
                }
            })], 1) : t.isDropdownWithEmojiTrigger ? e("gl-dropdown-item", t._b({
                attrs: {
                    "button-class": "top-nav-menu-item"
                },
                on: {
                    click: t.openModal
                }
            }, "gl-dropdown-item", t.componentAttributes, !1), [t._v("\n  " + t._s(t.displayText) + "\n  "), e("gl-emoji", {
                staticClass: "gl-vertical-align-baseline gl-reset-font-size gl-mr-1",
                attrs: {
                    "data-name": t.icon
                }
            })], 1) : t.isDisclosureTrigger ? e("gl-disclosure-dropdown-item", t._b({
                attrs: {
                    item: t.item
                },
                on: {
                    action: t.handleDisclosureDropdownAction
                }
            }, "gl-disclosure-dropdown-item", t.componentAttributes, !1)) : e("gl-link", t._b({
                attrs: {
                    "data-is-link": "true"
                },
                on: {
                    click: t.openModal
                }
            }, "gl-link", t.componentAttributes, !1), [t._v("\n  " + t._s(t.displayText) + "\n")])
        }
        ), [], !1, null, null, null);
        e.a = m.exports
    },
    "V5u/": function(t, e, n) {
        "use strict";
        n.d(e, "a", (function() {
            return a
        }
        )),
        n.d(e, "b", (function() {
            return c
        }
        )),
        n.d(e, "c", (function() {
            return l
        }
        )),
        n.d(e, "d", (function() {
            return u
        }
        )),
        n.d(e, "e", (function() {
            return i
        }
        )),
        n.d(e, "f", (function() {
            return b
        }
        )),
        n.d(e, "g", (function() {
            return s
        }
        )),
        n.d(e, "h", (function() {
            return o
        }
        )),
        n.d(e, "i", (function() {
            return r
        }
        )),
        n.d(e, "j", (function() {
            return d
        }
        )),
        n.d(e, "k", (function() {
            return p
        }
        )),
        n.d(e, "l", (function() {
            return g
        }
        )),
        n.d(e, "m", (function() {
            return m
        }
        ));
        const r = "shown"
          , o = "hidden"
          , i = "beforeClose"
          , s = "focusContent"
          , a = "ArrowDown"
          , c = "ArrowUp"
          , l = "End"
          , u = "Enter"
          , d = "Home"
          , m = "Space"
          , p = "absolute"
          , g = "fixed"
          , b = "gl-new-dropdown-contents"
    },
    XiQx: function(t, e, n) {
        "use strict";
        n.d(e, "a", (function() {
            return l
        }
        ));
        var r = n("V5u/")
          , o = n("Qog8")
          , i = n("7z1+")
          , s = n("BglX")
          , a = n("Pyw5")
          , c = n.n(a);
        const l = "gl-new-dropdown-item";
        const u = {
            name: s.c,
            ITEM_CLASS: l,
            props: {
                item: {
                    type: Object,
                    required: !1,
                    default: null,
                    validator: i.c
                }
            },
            computed: {
                isLink() {
                    var t;
                    return "string" == typeof (null === (t = this.item) || void 0 === t ? void 0 : t.href)
                },
                isCustomContent() {
                    return Boolean(this.$scopedSlots.default)
                },
                itemComponent() {
                    const {item: t} = this;
                    return this.isLink ? {
                        is: "a",
                        attrs: {
                            href: t.href,
                            ...t.extraAttrs
                        },
                        listeners: {
                            click: this.action
                        }
                    } : {
                        is: "button",
                        attrs: {
                            ...null == t ? void 0 : t.extraAttrs,
                            type: "button"
                        },
                        listeners: {
                            click: ()=>{
                                var e;
                                null == t || null === (e = t.action) || void 0 === e || e.call(void 0, t),
                                this.action()
                            }
                        }
                    }
                },
                listIndex() {
                    var t, e;
                    return null !== (t = this.item) && void 0 !== t && null !== (e = t.extraAttrs) && void 0 !== e && e.disabled ? null : 0
                },
                componentIndex() {
                    var t, e;
                    return null !== (t = this.item) && void 0 !== t && null !== (e = t.extraAttrs) && void 0 !== e && e.disabled ? null : -1
                },
                wrapperClass() {
                    var t, e;
                    return null !== (t = null === (e = this.item) || void 0 === e ? void 0 : e.wrapperClass) && void 0 !== t ? t : ""
                },
                wrapperListeners() {
                    const t = {
                        keydown: this.onKeydown
                    };
                    return this.isCustomContent && (t.click = this.action),
                    t
                }
            },
            methods: {
                onKeydown(t) {
                    const {code: e} = t;
                    var n;
                    e !== r.d && e !== r.m || (this.isCustomContent ? this.action() : (Object(o.k)(t),
                    null === (n = this.$refs.item) || void 0 === n || n.dispatchEvent(new MouseEvent("click",{
                        bubbles: !0,
                        cancelable: !0
                    }))))
                },
                action() {
                    this.$emit("action", this.item)
                }
            }
        };
        const d = c()({
            render: function() {
                var t = this
                  , e = t.$createElement
                  , n = t._self._c || e;
                return n("li", t._g({
                    class: [t.$options.ITEM_CLASS, t.wrapperClass],
                    attrs: {
                        tabindex: t.listIndex,
                        "data-testid": "disclosure-dropdown-item"
                    }
                }, t.wrapperListeners), [t._t("default", (function() {
                    return [n(t.itemComponent.is, t._g(t._b({
                        ref: "item",
                        tag: "component",
                        staticClass: "gl-new-dropdown-item-content",
                        attrs: {
                            tabindex: t.componentIndex
                        }
                    }, "component", t.itemComponent.attrs, !1), t.itemComponent.listeners), [n("span", {
                        staticClass: "gl-new-dropdown-item-text-wrapper"
                    }, [t._t("list-item", (function() {
                        return [t._v("\n          " + t._s(t.item.text) + "\n        ")]
                    }
                    ))], 2)])]
                }
                ))], 2)
            },
            staticRenderFns: []
        }, void 0, u, void 0, !1, void 0, !1, void 0, void 0, void 0);
        e.b = d
    },
    i7S8: function(t, e, n) {
        "use strict";
        n.d(e, "s", (function() {
            return i
        }
        )),
        n.d(e, "w", (function() {
            return s
        }
        )),
        n.d(e, "G", (function() {
            return a
        }
        )),
        n.d(e, "m", (function() {
            return c
        }
        )),
        n.d(e, "x", (function() {
            return l
        }
        )),
        n.d(e, "g", (function() {
            return u
        }
        )),
        n.d(e, "E", (function() {
            return d
        }
        )),
        n.d(e, "F", (function() {
            return m
        }
        )),
        n.d(e, "A", (function() {
            return p
        }
        )),
        n.d(e, "z", (function() {
            return g
        }
        )),
        n.d(e, "D", (function() {
            return b
        }
        )),
        n.d(e, "C", (function() {
            return f
        }
        )),
        n.d(e, "B", (function() {
            return v
        }
        )),
        n.d(e, "p", (function() {
            return h
        }
        )),
        n.d(e, "j", (function() {
            return j
        }
        )),
        n.d(e, "k", (function() {
            return w
        }
        )),
        n.d(e, "b", (function() {
            return q
        }
        )),
        n.d(e, "a", (function() {
            return z
        }
        )),
        n.d(e, "y", (function() {
            return R
        }
        )),
        n.d(e, "l", (function() {
            return G
        }
        )),
        n.d(e, "v", (function() {
            return N
        }
        )),
        n.d(e, "n", (function() {
            return U
        }
        )),
        n.d(e, "o", (function() {
            return H
        }
        )),
        n.d(e, "c", (function() {
            return Y
        }
        )),
        n.d(e, "i", (function() {
            return V
        }
        )),
        n.d(e, "q", (function() {
            return Q
        }
        )),
        n.d(e, "h", (function() {
            return Z
        }
        )),
        n.d(e, "r", (function() {
            return J
        }
        )),
        n.d(e, "H", (function() {
            return tt
        }
        )),
        n.d(e, "f", (function() {
            return et
        }
        )),
        n.d(e, "u", (function() {
            return nt
        }
        )),
        n.d(e, "e", (function() {
            return rt
        }
        )),
        n.d(e, "t", (function() {
            return ot
        }
        )),
        n.d(e, "d", (function() {
            return it
        }
        ));
        var r = n("/lV4")
          , o = n("qLpH");
        const i = "project-select"
          , s = 200
          , a = "gl-bg-green-100"
          , c = "gl-bg-red-100"
          , l = "members_invited_successfully"
          , u = {
            ALL: "all",
            DESCENDANT_GROUPS: "descendant_groups"
        }
          , d = "all"
          , m = "saml_provider_id"
          , p = "button"
          , g = "invite_members"
          , b = "text-emoji"
          , f = "dropdown-text-emoji"
          , v = "dropdown-text"
          , h = "invite_members_modal"
          , j = "invite_project_members_modal"
          , w = "project-members-page"
          , O = Object(r.g)("InviteMembersModal|Invite members")
          , y = Object(r.g)("InviteMembersModal|GitLab is better with colleagues!")
          , M = Object(r.g)("InviteMembersModal|How about inviting a colleague or two to join you?")
          , k = Object(r.g)("InviteMembersModal|You're inviting members to the %{strongStart}%{name}%{strongEnd} group.")
          , C = Object(r.g)("InviteMembersModal|You're inviting members to the %{strongStart}%{name}%{strongEnd} project.")
          , _ = Object(r.g)("InviteMembersModal|Congratulations on creating your project, you're almost there!")
          , I = Object(r.g)("InviteMembersModal|Username, name or email address")
          , x = Object(r.g)("InviteMembersModal|Select members or type email addresses")
          , S = Object(r.g)("InviteMembersModal|Invite a group")
          , A = Object(r.g)("InviteMembersModal|You're inviting a group to the %{strongStart}%{name}%{strongEnd} group.")
          , D = Object(r.g)("InviteMembersModal|You're inviting a group to the %{strongStart}%{name}%{strongEnd} project.")
          , E = Object(r.g)("InviteMembersModal|Inviting a group %{linkStart}adds its members to your group%{linkEnd}, including members who join after the invite. This might put your group over the free %{count} user limit.")
          , T = Object(o.a)("user/group/manage", {
            anchor: "share-a-group-with-another-group"
        })
          , P = Object(r.g)("InviteMembersModal|Inviting a group %{linkStart}adds its members to your project%{linkEnd}, including members who join after the invite. This might put your group over the free %{count} user limit.")
          , $ = Object(o.a)("user/project/members/index", {
            anchor: "add-groups-to-a-project"
        })
          , B = Object(r.g)("InviteMembersModal|Select a group to invite")
          , L = Object(r.g)("InviteMembersModal|Search for a group to invite")
          , q = Object(r.g)("InviteMembersModal|Select a role")
          , z = Object(r.g)("InviteMembersModal|Access expiration date (optional)")
          , R = Object(r.g)("InviteMembersModal|Members were successfully added")
          , G = Object(r.g)("InviteMembersModal|Something went wrong")
          , N = Object(r.g)("InviteMembersModal|%{linkStart}Read more%{linkEnd} about role permissions")
          , U = Object(r.g)("InviteMembersModal|Invite")
          , H = Object(r.g)("InviteMembersModal|Manage members")
          , Y = Object(r.g)("InviteMembersModal|Cancel")
          , V = Object(r.g)("InviteMembersModal|Close invite team members")
          , X = Object(r.g)("InviteMembersModal|Review the invite errors and try again:")
          , F = Object(r.g)("InviteMembersModal|Show more (%{count})")
          , K = Object(r.g)("InviteMembersModal|Show less")
          , W = Object(r.g)("InviteMembersModal|Please add members to invite")
          , Q = {
            modal: {
                default: {
                    title: O
                },
                celebrate: {
                    title: y,
                    intro: M
                }
            },
            toGroup: {
                default: {
                    introText: k
                }
            },
            toProject: {
                default: {
                    introText: C
                },
                celebrate: {
                    introText: _
                }
            },
            searchField: I,
            placeHolder: x,
            toastMessageSuccessful: R,
            memberErrorListText: X,
            collapsedErrors: F,
            expandedErrors: K,
            emptyInvitesAlertText: W
        }
          , Z = {
            title: S,
            toGroup: {
                introText: A,
                notificationText: E,
                notificationLink: T
            },
            toProject: {
                introText: D,
                notificationText: P,
                notificationLink: $
            },
            searchField: B,
            placeHolder: L,
            toastMessageSuccessful: R
        }
          , J = "over_limit_modal_viewed"
          , tt = Object(r.g)("InviteMembersModal|You only have space for %{count} more %{members} in %{name}")
          , et = Object(r.g)("InviteMembersModal|You've reached your %{count} %{members} limit for %{name}")
          , nt = "reached"
          , rt = "close"
          , ot = Object(r.g)("InviteMembersModal|To invite new users to this top-level group, you must remove existing users. You can still add existing users from the top-level group, including any subgroups and projects.").concat(Object(r.g)("InviteMembersModal| To get more members, the owner of this top-level group can %{trialLinkStart}start a trial%{trialLinkEnd} or %{upgradeLinkStart}upgrade%{upgradeLinkEnd} to a paid tier."))
          , it = Object(r.g)("InviteMembersModal|To get more members an owner of the group can %{trialLinkStart}start a trial%{trialLinkEnd} or %{upgradeLinkStart}upgrade%{upgradeLinkEnd} to a paid tier.")
    },
    wP8z: function(t, e, n) {
        "use strict";
        var r = n("8ENL")
          , o = n("0zRR")
          , i = n("h3Ey")
          , s = n("fkuG")
          , a = n("o/E4")
          , c = n("7bmO")
          , l = n("ua/H")
          , u = n("P/DV")
          , d = n("EQ0+")
          , m = n("1m+M");
        function p(t, e) {
            var n = Object.keys(t);
            if (Object.getOwnPropertySymbols) {
                var r = Object.getOwnPropertySymbols(t);
                e && (r = r.filter((function(e) {
                    return Object.getOwnPropertyDescriptor(t, e).enumerable
                }
                ))),
                n.push.apply(n, r)
            }
            return n
        }
        function g(t) {
            for (var e = 1; e < arguments.length; e++) {
                var n = null != arguments[e] ? arguments[e] : {};
                e % 2 ? p(Object(n), !0).forEach((function(e) {
                    b(t, e, n[e])
                }
                )) : Object.getOwnPropertyDescriptors ? Object.defineProperties(t, Object.getOwnPropertyDescriptors(n)) : p(Object(n)).forEach((function(e) {
                    Object.defineProperty(t, e, Object.getOwnPropertyDescriptor(n, e))
                }
                ))
            }
            return t
        }
        function b(t, e, n) {
            return e in t ? Object.defineProperty(t, e, {
                value: n,
                enumerable: !0,
                configurable: !0,
                writable: !0
            }) : t[e] = n,
            t
        }
        var f = Object(c.j)(m.b, ["event", "routerTag"])
          , v = Object(l.c)(Object(c.m)(g(g({}, f), {}, {
            linkClass: Object(l.b)(s.e),
            variant: Object(l.b)(s.r)
        })), o.o)
          , h = Object(r.c)({
            name: o.o,
            mixins: [u.a, d.a],
            inject: {
                getBvDropdown: {
                    default: function() {
                        return function() {
                            return null
                        }
                    }
                }
            },
            inheritAttrs: !1,
            props: v,
            computed: {
                bvDropdown: function() {
                    return this.getBvDropdown()
                },
                computedAttrs: function() {
                    return g(g({}, this.bvAttrs), {}, {
                        role: "menuitem"
                    })
                }
            },
            methods: {
                closeDropdown: function() {
                    var t = this;
                    Object(a.B)((function() {
                        t.bvDropdown && t.bvDropdown.hide(!0)
                    }
                    ))
                },
                onClick: function(t) {
                    this.$emit(i.f, t),
                    this.closeDropdown()
                }
            },
            render: function(t) {
                var e = this.linkClass
                  , n = this.variant
                  , r = this.active
                  , o = this.disabled
                  , i = this.onClick
                  , s = this.bvAttrs;
                return t("li", {
                    class: s.class,
                    style: s.style,
                    attrs: {
                        role: "presentation"
                    }
                }, [t(m.a, {
                    staticClass: "dropdown-item",
                    class: [e, b({}, "text-".concat(n), n && !(r || o))],
                    props: Object(l.d)(f, this.$props),
                    attrs: this.computedAttrs,
                    on: {
                        click: i
                    },
                    ref: "item"
                }, this.normalizeSlot())])
            }
        });
        function j(t, e) {
            var n = Object.keys(t);
            if (Object.getOwnPropertySymbols) {
                var r = Object.getOwnPropertySymbols(t);
                e && (r = r.filter((function(e) {
                    return Object.getOwnPropertyDescriptor(t, e).enumerable
                }
                ))),
                n.push.apply(n, r)
            }
            return n
        }
        function w(t) {
            for (var e = 1; e < arguments.length; e++) {
                var n = null != arguments[e] ? arguments[e] : {};
                e % 2 ? j(Object(n), !0).forEach((function(e) {
                    O(t, e, n[e])
                }
                )) : Object.getOwnPropertyDescriptors ? Object.defineProperties(t, Object.getOwnPropertyDescriptors(n)) : j(Object(n)).forEach((function(e) {
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
        var y = Object(l.c)({
            active: Object(l.b)(s.g, !1),
            activeClass: Object(l.b)(s.r, "active"),
            buttonClass: Object(l.b)(s.e),
            disabled: Object(l.b)(s.g, !1),
            variant: Object(l.b)(s.r)
        }, o.p)
          , M = Object(r.c)({
            name: o.p,
            mixins: [u.a, d.a],
            inject: {
                getBvDropdown: {
                    default: function() {
                        return function() {
                            return null
                        }
                    }
                }
            },
            inheritAttrs: !1,
            props: y,
            computed: {
                bvDropdown: function() {
                    return this.getBvDropdown()
                },
                computedAttrs: function() {
                    return w(w({}, this.bvAttrs), {}, {
                        role: "menuitem",
                        type: "button",
                        disabled: this.disabled
                    })
                }
            },
            methods: {
                closeDropdown: function() {
                    this.bvDropdown && this.bvDropdown.hide(!0)
                },
                onClick: function(t) {
                    this.$emit(i.f, t),
                    this.closeDropdown()
                }
            },
            render: function(t) {
                var e, n = this.active, r = this.variant, o = this.bvAttrs;
                return t("li", {
                    class: o.class,
                    style: o.style,
                    attrs: {
                        role: "presentation"
                    }
                }, [t("button", {
                    staticClass: "dropdown-item",
                    class: [this.buttonClass, (e = {},
                    O(e, this.activeClass, n),
                    O(e, "text-".concat(r), r && !(n || this.disabled)),
                    e)],
                    attrs: this.computedAttrs,
                    on: {
                        click: this.onClick
                    },
                    ref: "button"
                }, this.normalizeSlot())])
            }
        })
          , k = n("XBTk")
          , C = n("tbP8")
          , _ = n("4lAS")
          , I = n("s1D3")
          , x = n("Pyw5")
          , S = n.n(x);
        const A = {
            name: "GlDropdownItem",
            components: {
                GlIcon: I.a,
                GlAvatar: C.a,
                GlButton: _.a
            },
            inheritAttrs: !1,
            props: {
                avatarUrl: {
                    type: String,
                    required: !1,
                    default: ""
                },
                iconColor: {
                    type: String,
                    required: !1,
                    default: ""
                },
                iconName: {
                    type: String,
                    required: !1,
                    default: ""
                },
                iconRightAriaLabel: {
                    type: String,
                    required: !1,
                    default: ""
                },
                iconRightName: {
                    type: String,
                    required: !1,
                    default: ""
                },
                isChecked: {
                    type: Boolean,
                    required: !1,
                    default: !1
                },
                isCheckItem: {
                    type: Boolean,
                    required: !1,
                    default: !1
                },
                isCheckCentered: {
                    type: Boolean,
                    required: !1,
                    default: !1
                },
                secondaryText: {
                    type: String,
                    required: !1,
                    default: ""
                }
            },
            computed: {
                bootstrapComponent() {
                    const {href: t, to: e} = this.$attrs;
                    return t || e ? h : M
                },
                iconColorCss() {
                    return k.M[this.iconColor] || "gl-text-gray-700"
                },
                shouldShowCheckIcon() {
                    return this.isChecked || this.isCheckItem
                },
                checkedClasses() {
                    return this.isCheckCentered ? "" : "gl-mt-3 gl-align-self-start"
                }
            },
            methods: {
                handleClickIconRight() {
                    this.$emit("click-icon-right")
                }
            }
        };
        const D = S()({
            render: function() {
                var t = this
                  , e = t.$createElement
                  , n = t._self._c || e;
                return n(t.bootstrapComponent, t._g(t._b({
                    tag: "component",
                    staticClass: "gl-dropdown-item"
                }, "component", t.$attrs, !1), t.$listeners), [t.shouldShowCheckIcon ? n("gl-icon", {
                    class: ["gl-dropdown-item-check-icon", {
                        "gl-visibility-hidden": !t.isChecked
                    }, t.checkedClasses],
                    attrs: {
                        name: "mobile-issue-close",
                        "data-testid": "dropdown-item-checkbox"
                    }
                }) : t._e(), t._v(" "), t.iconName ? n("gl-icon", {
                    class: ["gl-dropdown-item-icon", t.iconColorCss],
                    attrs: {
                        name: t.iconName
                    }
                }) : t._e(), t._v(" "), t.avatarUrl ? n("gl-avatar", {
                    attrs: {
                        size: 32,
                        src: t.avatarUrl
                    }
                }) : t._e(), t._v(" "), n("div", {
                    staticClass: "gl-dropdown-item-text-wrapper"
                }, [n("p", {
                    staticClass: "gl-dropdown-item-text-primary"
                }, [t._t("default")], 2), t._v(" "), t.secondaryText ? n("p", {
                    staticClass: "gl-dropdown-item-text-secondary"
                }, [t._v(t._s(t.secondaryText))]) : t._e()]), t._v(" "), t.iconRightName ? n("gl-button", {
                    attrs: {
                        size: "small",
                        icon: t.iconRightName,
                        "aria-label": t.iconRightAriaLabel || t.iconRightName
                    },
                    on: {
                        click: function(e) {
                            return e.stopPropagation(),
                            e.preventDefault(),
                            t.handleClickIconRight.apply(null, arguments)
                        }
                    }
                }) : t._e()], 1)
            },
            staticRenderFns: []
        }, void 0, A, void 0, !1, void 0, !1, void 0, void 0, void 0);
        e.a = D
    }
}]);
//# sourceMappingURL=initInviteMembersTrigger.978c5cc8.chunk.js.map

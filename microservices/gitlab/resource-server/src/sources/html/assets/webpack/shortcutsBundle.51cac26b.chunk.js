(this.webpackJsonp = this.webpackJsonp || []).push([[765], {
    O6Bj: function(e, t, o) {
        "use strict";
        o.d(t, "c", (function() {
            return c
        }
        )),
        o.d(t, "a", (function() {
            return d
        }
        ));
        var i = o("t6Yz")
          , n = o.n(i);
        o.d(t, "b", (function() {
            return n.a
        }
        ));
        const s = []
          , r = n.a.prototype.stopCallback;
        n.a.prototype.stopCallback = function(e, t, o) {
            for (const i of s) {
                const n = i.call(this, e, t, o);
                if (void 0 !== n)
                    return n
            }
            return r.call(this, e, t, o)
        }
        ;
        const c = function(e) {
            s.unshift(e)
        }
          , d = "mod+c"
    },
    U6io: function(e, t) {
        !function(e) {
            var t = e.prototype.stopCallback;
            e.prototype.stopCallback = function(e, o, i) {
                return !!this.paused || t.call(this, e, o, i)
            }
            ,
            e.prototype.pause = function() {
                this.paused = !0
            }
            ,
            e.prototype.unpause = function() {
                this.paused = !1
            }
            ,
            e.init()
        }(Mousetrap)
    },
    d08M: function(e, t, o) {
        "use strict";
        o.d(t, "rb", (function() {
            return d
        }
        )),
        o.d(t, "A", (function() {
            return u
        }
        )),
        o.d(t, "x", (function() {
            return a
        }
        )),
        o.d(t, "d", (function() {
            return l
        }
        )),
        o.d(t, "e", (function() {
            return p
        }
        )),
        o.d(t, "C", (function() {
            return f
        }
        )),
        o.d(t, "ob", (function() {
            return b
        }
        )),
        o.d(t, "c", (function() {
            return g
        }
        )),
        o.d(t, "y", (function() {
            return h
        }
        )),
        o.d(t, "z", (function() {
            return j
        }
        )),
        o.d(t, "B", (function() {
            return y
        }
        )),
        o.d(t, "D", (function() {
            return m
        }
        )),
        o.d(t, "tb", (function() {
            return O
        }
        )),
        o.d(t, "E", (function() {
            return K
        }
        )),
        o.d(t, "ub", (function() {
            return v
        }
        )),
        o.d(t, "qb", (function() {
            return T
        }
        )),
        o.d(t, "a", (function() {
            return k
        }
        )),
        o.d(t, "P", (function() {
            return w
        }
        )),
        o.d(t, "pb", (function() {
            return S
        }
        )),
        o.d(t, "Q", (function() {
            return G
        }
        )),
        o.d(t, "F", (function() {
            return M
        }
        )),
        o.d(t, "bb", (function() {
            return I
        }
        )),
        o.d(t, "sb", (function() {
            return C
        }
        )),
        o.d(t, "b", (function() {
            return E
        }
        )),
        o.d(t, "kb", (function() {
            return R
        }
        )),
        o.d(t, "lb", (function() {
            return x
        }
        )),
        o.d(t, "nb", (function() {
            return q
        }
        )),
        o.d(t, "jb", (function() {
            return F
        }
        )),
        o.d(t, "mb", (function() {
            return P
        }
        )),
        o.d(t, "ib", (function() {
            return B
        }
        )),
        o.d(t, "p", (function() {
            return z
        }
        )),
        o.d(t, "f", (function() {
            return A
        }
        )),
        o.d(t, "r", (function() {
            return H
        }
        )),
        o.d(t, "i", (function() {
            return N
        }
        )),
        o.d(t, "j", (function() {
            return _
        }
        )),
        o.d(t, "g", (function() {
            return L
        }
        )),
        o.d(t, "t", (function() {
            return U
        }
        )),
        o.d(t, "s", (function() {
            return Y
        }
        )),
        o.d(t, "k", (function() {
            return W
        }
        )),
        o.d(t, "ab", (function() {
            return V
        }
        )),
        o.d(t, "l", (function() {
            return J
        }
        )),
        o.d(t, "o", (function() {
            return Z
        }
        )),
        o.d(t, "q", (function() {
            return Q
        }
        )),
        o.d(t, "m", (function() {
            return X
        }
        )),
        o.d(t, "h", (function() {
            return $
        }
        )),
        o.d(t, "n", (function() {
            return ee
        }
        )),
        o.d(t, "u", (function() {
            return te
        }
        )),
        o.d(t, "w", (function() {
            return oe
        }
        )),
        o.d(t, "v", (function() {
            return ie
        }
        )),
        o.d(t, "gb", (function() {
            return ne
        }
        )),
        o.d(t, "fb", (function() {
            return se
        }
        )),
        o.d(t, "hb", (function() {
            return re
        }
        )),
        o.d(t, "cb", (function() {
            return ce
        }
        )),
        o.d(t, "eb", (function() {
            return de
        }
        )),
        o.d(t, "db", (function() {
            return ue
        }
        )),
        o.d(t, "H", (function() {
            return ae
        }
        )),
        o.d(t, "J", (function() {
            return le
        }
        )),
        o.d(t, "G", (function() {
            return pe
        }
        )),
        o.d(t, "I", (function() {
            return fe
        }
        )),
        o.d(t, "L", (function() {
            return be
        }
        )),
        o.d(t, "M", (function() {
            return ge
        }
        )),
        o.d(t, "V", (function() {
            return he
        }
        )),
        o.d(t, "X", (function() {
            return je
        }
        )),
        o.d(t, "U", (function() {
            return ye
        }
        )),
        o.d(t, "Z", (function() {
            return me
        }
        )),
        o.d(t, "W", (function() {
            return Oe
        }
        )),
        o.d(t, "Y", (function() {
            return Ke
        }
        )),
        o.d(t, "T", (function() {
            return ve
        }
        )),
        o.d(t, "R", (function() {
            return Te
        }
        )),
        o.d(t, "S", (function() {
            return ke
        }
        )),
        o.d(t, "N", (function() {
            return we
        }
        )),
        o.d(t, "O", (function() {
            return Se
        }
        )),
        o.d(t, "K", (function() {
            return Ge
        }
        )),
        o.d(t, "vb", (function() {
            return Ce
        }
        )),
        o.d(t, "wb", (function() {
            return De
        }
        ));
        var i = o("htNe")
          , n = o.n(i)
          , s = o("n7CP")
          , r = o("/lV4");
        const c = n()((function() {
            let e = {};
            if (s.a.canUseLocalStorage())
                try {
                    e = JSON.parse(localStorage.getItem("gl-keyboard-shortcuts-customizations") || "{}")
                } catch (e) {}
            return e
        }
        ))
          , d = {
            id: "globalShortcuts.toggleKeyboardShortcutsDialog",
            description: Object(r.a)("Toggle keyboard shortcuts help dialog"),
            defaultKeys: ["?"]
        }
          , u = {
            id: "globalShortcuts.goToYourProjects",
            description: Object(r.a)("Go to your projects"),
            defaultKeys: ["shift+p"]
        }
          , a = {
            id: "globalShortcuts.goToYourGroups",
            description: Object(r.a)("Go to your groups"),
            defaultKeys: ["shift+g"]
        }
          , l = {
            id: "globalShortcuts.goToActivityFeed",
            description: Object(r.a)("Go to the activity feed"),
            defaultKeys: ["shift+a"]
        }
          , p = {
            id: "globalShortcuts.goToMilestoneList",
            description: Object(r.a)("Go to the milestone list"),
            defaultKeys: ["shift+l"]
        }
          , f = {
            id: "globalShortcuts.goToYourSnippets",
            description: Object(r.a)("Go to your snippets"),
            defaultKeys: ["shift+s"]
        }
          , b = {
            id: "globalShortcuts.startSearch",
            description: Object(r.a)("Start search"),
            defaultKeys: ["s", "/"]
        }
          , g = {
            id: "globalShortcuts.focusFilterBar",
            description: Object(r.a)("Focus filter bar"),
            defaultKeys: ["f"]
        }
          , h = {
            id: "globalShortcuts.goToYourIssues",
            description: Object(r.a)("Go to your issues"),
            defaultKeys: ["shift+i"]
        }
          , j = {
            id: "globalShortcuts.goToYourMergeRequests",
            description: Object(r.a)("Go to your merge requests"),
            defaultKeys: ["shift+m"]
        }
          , y = {
            id: "globalShortcuts.goToYourReviewRequests",
            description: Object(r.a)("Go to your review requests"),
            defaultKeys: ["shift+r"]
        }
          , m = {
            id: "globalShortcuts.goToYourTodoList",
            description: Object(r.a)("Go to your To-Do list"),
            defaultKeys: ["shift+t"]
        }
          , O = {
            id: "globalShortcuts.togglePerformanceBar",
            description: Object(r.a)("Toggle the Performance Bar"),
            defaultKeys: ["p b"]
        }
          , K = {
            id: "globalShortcuts.hideAppearingContent",
            description: Object(r.a)("Hide tooltips or popovers"),
            defaultKeys: ["esc"]
        }
          , v = {
            id: "globalShortcuts.toggleSuperSidebar",
            description: Object(r.a)("Toggle the navigation sidebar"),
            defaultKeys: ["mod+\\"]
        }
          , T = {
            id: "globalShortcuts.toggleCanary",
            description: Object(r.a)("Toggle GitLab Next"),
            defaultKeys: ["g x"]
        }
          , k = {
            id: "editing.boldText",
            description: Object(r.a)("Bold text"),
            defaultKeys: ["mod+b"],
            customizable: !1
        }
          , w = {
            id: "editing.italicText",
            description: Object(r.a)("Italic text"),
            defaultKeys: ["mod+i"],
            customizable: !1
        }
          , S = {
            id: "editing.strikethroughText",
            description: Object(r.a)("Strikethrough text"),
            defaultKeys: ["mod+shift+x"],
            customizable: !1
        }
          , G = {
            id: "editing.linkText",
            description: Object(r.a)("Link text"),
            defaultKeys: ["mod+k"],
            customizable: !1
        }
          , M = {
            id: "editing.indentLine",
            description: Object(r.a)("Indent line"),
            defaultKeys: ["mod+]"],
            customizable: !1
        }
          , I = {
            id: "editing.outdentLine",
            description: Object(r.a)("Outdent line"),
            defaultKeys: ["mod+["],
            customizable: !1
        }
          , C = {
            id: "editing.toggleMarkdownPreview",
            description: Object(r.a)("Toggle Markdown preview"),
            defaultKeys: ["ctrl+shift+p", "command+shift+p"]
        }
          , D = {
            id: "editing.editRecentComment",
            description: Object(r.a)("Edit your most recent comment in a thread (from an empty textarea)"),
            defaultKeys: ["up"]
        }
          , E = {
            id: "wiki.editWikiPage",
            description: Object(r.a)("Edit wiki page"),
            defaultKeys: ["e"]
        }
          , R = {
            id: "repositoryGraph.scrollLeft",
            description: Object(r.a)("Scroll left"),
            defaultKeys: ["left", "h"]
        }
          , x = {
            id: "repositoryGraph.scrollRight",
            description: Object(r.a)("Scroll right"),
            defaultKeys: ["right", "l"]
        }
          , q = {
            id: "repositoryGraph.scrollUp",
            description: Object(r.a)("Scroll up"),
            defaultKeys: ["up", "k"]
        }
          , F = {
            id: "repositoryGraph.scrollDown",
            description: Object(r.a)("Scroll down"),
            defaultKeys: ["down", "j"]
        }
          , P = {
            id: "repositoryGraph.scrollToTop",
            description: Object(r.a)("Scroll to top"),
            defaultKeys: ["shift+up", "shift+k"]
        }
          , B = {
            id: "repositoryGraph.scrollToBottom",
            description: Object(r.a)("Scroll to bottom"),
            defaultKeys: ["shift+down", "shift+j"]
        }
          , z = {
            id: "project.goToOverview",
            description: Object(r.a)("Go to the project's overview page"),
            defaultKeys: ["g o"]
        }
          , A = {
            id: "project.goToActivityFeed",
            description: Object(r.a)("Go to the project's activity feed"),
            defaultKeys: ["g v"]
        }
          , H = {
            id: "project.goToReleases",
            description: Object(r.a)("Go to releases"),
            defaultKeys: ["g r"]
        }
          , N = {
            id: "project.goToFiles",
            description: Object(r.a)("Go to files"),
            defaultKeys: ["g f"]
        }
          , _ = {
            id: "project.goToFindFile",
            description: Object(r.a)("Go to find file"),
            defaultKeys: ["t"]
        }
          , L = {
            id: "project.goToCommits",
            description: Object(r.a)("Go to commits"),
            defaultKeys: ["g c"]
        }
          , U = {
            id: "project.goToRepoGraph",
            description: Object(r.a)("Go to repository graph"),
            defaultKeys: ["g n"]
        }
          , Y = {
            id: "project.goToRepoCharts",
            description: Object(r.a)("Go to repository charts"),
            defaultKeys: ["g d"]
        }
          , W = {
            id: "project.goToIssues",
            description: Object(r.a)("Go to issues"),
            defaultKeys: ["g i"]
        }
          , V = {
            id: "project.newIssue",
            description: Object(r.a)("New issue"),
            defaultKeys: ["i"]
        }
          , J = {
            id: "project.goToIssueBoards",
            description: Object(r.a)("Go to issue boards"),
            defaultKeys: ["g b"]
        }
          , Z = {
            id: "project.goToMergeRequests",
            description: Object(r.a)("Go to merge requests"),
            defaultKeys: ["g m"]
        }
          , Q = {
            id: "project.goToPipelines",
            description: Object(r.a)("Go to pipelines"),
            defaultKeys: ["g p"]
        }
          , X = {
            id: "project.goToJobs",
            description: Object(r.a)("Go to jobs"),
            defaultKeys: ["g j"]
        }
          , $ = {
            id: "project.goToEnvironments",
            description: Object(r.a)("Go to environments"),
            defaultKeys: ["g e"]
        }
          , ee = {
            id: "project.goToKubernetes",
            description: Object(r.a)("Go to kubernetes"),
            defaultKeys: ["g k"]
        }
          , te = {
            id: "project.goToSnippets",
            description: Object(r.a)("Go to snippets"),
            defaultKeys: ["g s"]
        }
          , oe = {
            id: "project.goToWiki",
            description: Object(r.a)("Go to wiki"),
            defaultKeys: ["g w"]
        }
          , ie = {
            id: "project.goToWebIDE",
            description: Object(r.a)("Open in Web IDE"),
            defaultKeys: ["."]
        }
          , ne = {
            id: "projectFiles.moveSelectionUp",
            description: Object(r.a)("Move selection up"),
            defaultKeys: ["up"]
        }
          , se = {
            id: "projectFiles.moveSelectionDown",
            description: Object(r.a)("Move selection down"),
            defaultKeys: ["down"]
        }
          , re = {
            id: "projectFiles.openSelection",
            description: Object(r.a)("Open Selection"),
            defaultKeys: ["enter"]
        }
          , ce = {
            id: "projectFiles.goBack",
            description: Object(r.a)("Go back (while searching for files)"),
            defaultKeys: ["esc"]
        }
          , de = {
            id: "projectFiles.goToFilePermalink",
            description: Object(r.a)("Go to file permalink (while viewing a file)"),
            defaultKeys: ["y"]
        }
          , ue = {
            id: "projectFiles.goToCompare",
            description: Object(r.a)("Compare Branches"),
            defaultKeys: ["shift+c"]
        }
          , ae = {
            id: "issuables.commentReply",
            description: Object(r.a)("Comment/Reply (quoting selected text)"),
            defaultKeys: ["r"]
        }
          , le = {
            id: "issuables.editDescription",
            description: Object(r.a)("Edit description"),
            defaultKeys: ["e"]
        }
          , pe = {
            id: "issuables.changeLabel",
            description: Object(r.a)("Change label"),
            defaultKeys: ["l"]
        }
          , fe = {
            id: "issuables.copyIssuableRef",
            description: Object(r.a)("Copy reference"),
            defaultKeys: ["c r"]
        }
          , be = {
            id: "issuesMRs.changeAssignee",
            description: Object(r.a)("Change assignee"),
            defaultKeys: ["a"]
        }
          , ge = {
            id: "issuesMRs.changeMilestone",
            description: Object(r.a)("Change milestone"),
            defaultKeys: ["m"]
        }
          , he = {
            id: "mergeRequests.nextFileInDiff",
            description: Object(r.a)("Next file in diff"),
            defaultKeys: ["]", "j"]
        }
          , je = {
            id: "mergeRequests.previousFileInDiff",
            description: Object(r.a)("Previous file in diff"),
            defaultKeys: ["[", "k"]
        }
          , ye = {
            id: "mergeRequests.goToFile",
            description: Object(r.a)("Go to file"),
            defaultKeys: ["mod+p", "t"],
            customizable: !1
        }
          , me = {
            id: "mergeRequests.toggleFileBrowser",
            description: Object(r.a)("Toggle file browser"),
            defaultKeys: ["f"],
            customizable: !1
        }
          , Oe = {
            id: "mergeRequests.nextUnresolvedDiscussion",
            description: Object(r.a)("Next unresolved discussion"),
            defaultKeys: ["n"]
        }
          , Ke = {
            id: "mergeRequests.previousUnresolvedDiscussion",
            description: Object(r.a)("Previous unresolved discussion"),
            defaultKeys: ["p"]
        }
          , ve = {
            id: "mergeRequests.copySourceBranchName",
            description: Object(r.a)("Copy source branch name"),
            defaultKeys: ["b"]
        }
          , Te = {
            id: "mergeRequestCommits.nextCommit",
            description: Object(r.a)("Next commit"),
            defaultKeys: ["c"]
        }
          , ke = {
            id: "mergeRequestCommits.previousCommit",
            description: Object(r.a)("Previous commit"),
            defaultKeys: ["x"]
        }
          , we = {
            id: "issues.nextDesign",
            description: Object(r.a)("Next design"),
            defaultKeys: ["right"]
        }
          , Se = {
            id: "issues.previousDesign",
            description: Object(r.a)("Previous design"),
            defaultKeys: ["left"]
        }
          , Ge = {
            id: "issues.closeDesign",
            description: Object(r.a)("Close design"),
            defaultKeys: ["esc"]
        }
          , Me = {
            id: "webIDE.goToFile",
            description: Object(r.a)("Go to file"),
            defaultKeys: ["mod+p", "t"],
            customizable: !1
        }
          , Ie = {
            id: "webIDE.commit",
            description: Object(r.a)("Commit (when editing commit message)"),
            defaultKeys: ["mod+enter"],
            customizable: !1
        }
          , Ce = [{
            id: "globalShortcuts",
            name: Object(r.a)("Global Shortcuts"),
            keybindings: [d, u, a, l, p, f, b, g, h, j, y, m, O, K, v]
        }, {
            id: "editing",
            name: Object(r.a)("Editing"),
            keybindings: [k, w, S, G, C, D]
        }, {
            id: "wiki",
            name: Object(r.a)("Wiki"),
            keybindings: [E]
        }, {
            id: "repositoryGraph",
            name: Object(r.a)("Repository Graph"),
            keybindings: [R, x, q, F, P, B]
        }, {
            id: "project",
            name: Object(r.a)("Project"),
            keybindings: [z, A, H, N, _, L, U, Y, W, V, J, Z, Q, X, $, ee, te, oe, ie]
        }, {
            id: "projectFiles",
            name: Object(r.a)("Project Files"),
            keybindings: [ne, se, re, ce, de, ue]
        }, {
            id: "issuables",
            name: Object(r.a)("Epics, issues, and merge requests"),
            keybindings: [ae, le, pe, fe]
        }, {
            id: "issuesMRs",
            name: Object(r.a)("Issues and merge requests"),
            keybindings: [be, ge]
        }, {
            id: "mergeRequests",
            name: Object(r.a)("Merge requests"),
            keybindings: [he, je, ye, Oe, Ke, ve, me]
        }, {
            id: "mergeRequestCommits",
            name: Object(r.a)("Merge request commits"),
            keybindings: [Te, ke]
        }, {
            id: "issues",
            name: Object(r.a)("Issues"),
            keybindings: [we, Se, Ge]
        }, {
            id: "webIDE",
            name: Object(r.a)("Legacy Web IDE"),
            keybindings: [Me, Ie]
        }, {
            id: "misc",
            name: Object(r.a)("Miscellaneous"),
            keybindings: [T]
        }]
          , De = function(e) {
            return function({customizable: e}) {
                return Boolean(null == e || e)
            }(e) && c()[e.id] || e.defaultKeys
        }
    },
    "v+Mp": function(e, t, o) {
        "use strict";
        o.r(t),
        o.d(t, "LOCAL_MOUSETRAP_DATA_KEY", (function() {
            return f
        }
        )),
        o.d(t, "default", (function() {
            return g
        }
        ));
        var i = o("8Doe")
          , n = o.n(i)
          , s = (o("B++/"),
        o("z6RN"),
        o("47t/"),
        o("Tznw"),
        o("IYH6"),
        o("6yen"),
        o("OeRx"),
        o("l/dT"),
        o("RqS2"),
        o("Zy7a"),
        o("cjZU"),
        o("OAhk"),
        o("X42P"),
        o("mHhP"),
        o("fn0I"),
        o("UB/6"),
        o("imhG"),
        o("TPye"),
        o("FMw2"),
        o("dHQd"),
        o("yoDG"),
        o("tWNI"),
        o("8d6S"),
        o("VwWG"),
        o("IYHS"),
        o("MViX"),
        o("GDOA"),
        o("a0mT"),
        o("ta8/"),
        o("IKCR"),
        o("nmTw"),
        o("W2kU"),
        o("58fc"),
        o("Rhav"),
        o("eppl"),
        o("ZzK0"),
        o("BzOf"),
        o("EmJ/"))
          , r = o.n(s)
          , c = o("ewH8")
          , d = o("O6Bj")
          , u = o("NmEs")
          , a = o("yQ8t")
          , l = o("3twG")
          , p = o("d08M");
        o("U6io");
        const f = "local-mousetrap-instance";
        function b(e) {
            const t = e.closest(".md-area").find(".js-md")
              , o = new Map;
            return t.each((function() {
                const e = r()(this)
                  , t = e.data("md-shortcuts");
                null != t && t.length && o.set(e, t)
            }
            )),
            o
        }
        class g {
            constructor() {
                this.extensions = new Map,
                this.onToggleHelp = this.onToggleHelp.bind(this),
                this.helpModalElement = null,
                this.helpModalVueInstance = null,
                this.addAll([[p.rb, this.onToggleHelp], [p.ob, g.focusSearch], [p.c, this.focusFilter.bind(this)], [p.tb, g.onTogglePerfBar], [p.E, g.hideAppearingContent], [p.qb, g.onToggleCanary], [p.D, function() {
                    return Object(a.a)(".shortcuts-todos")
                }
                ], [p.d, function() {
                    return Object(a.a)(".dashboard-shortcuts-activity")
                }
                ], [p.y, function() {
                    return Object(a.a)(".dashboard-shortcuts-issues")
                }
                ], [p.z, function() {
                    return Object(a.a)(".dashboard-shortcuts-merge_requests")
                }
                ], [p.B, function() {
                    return Object(a.a)(".dashboard-shortcuts-review_requests")
                }
                ], [p.A, function() {
                    return Object(a.a)(".dashboard-shortcuts-projects")
                }
                ], [p.x, function() {
                    return Object(a.a)(".dashboard-shortcuts-groups")
                }
                ], [p.e, function() {
                    return Object(a.a)(".dashboard-shortcuts-milestones")
                }
                ], [p.C, function() {
                    return Object(a.a)(".dashboard-shortcuts-snippets")
                }
                ], [p.sb, g.toggleMarkdownPreview]]),
                Object(d.c)((function(e, t, o) {
                    return !Object(p.wb)(p.sb).includes(o) && void 0
                }
                ));
                const e = document.body.dataset.findFile;
                null != e && this.add(p.j, (function() {
                    Object(l.U)(e)
                }
                )),
                r()(document).on("click", ".js-shortcuts-modal-trigger", this.onToggleHelp),
                window.gon.keyboard_shortcuts_enabled || (localStorage.setItem("shortcutsDisabled", !0),
                d.b.pause())
            }
            addExtension(e, t=[], o=new Set) {
                o.add(e);
                let i = this.extensions.get(e);
                if (!i) {
                    for (const t of null !== (n = e.dependencies) && void 0 !== n ? n : []) {
                        var n;
                        o.has(t) || t === g || (o.add(t),
                        this.addExtension(t, [], o))
                    }
                    i = new e(this,...t),
                    this.extensions.set(e, i)
                }
                return o.delete(e),
                i
            }
            add(e, t) {
                d.b.bind(Object(p.wb)(e), t)
            }
            addAll(e) {
                var t = this;
                e.forEach((function(e) {
                    return t.add(...e)
                }
                ))
            }
            onToggleHelp(e) {
                var t = this;
                null != e && e.preventDefault && e.preventDefault(),
                this.helpModalElement && this.helpModalVueInstance ? (this.helpModalVueInstance.$destroy(),
                this.helpModalElement.remove(),
                this.helpModalElement = null,
                this.helpModalVueInstance = null) : (this.helpModalElement = document.createElement("div"),
                document.body.append(this.helpModalElement),
                this.helpModalVueInstance = new c.default({
                    el: this.helpModalElement,
                    components: {
                        ShortcutsHelp: function() {
                            return Promise.all([o.e(50), o.e(826)]).then(o.bind(null, "exIq"))
                        }
                    },
                    render: function(e) {
                        return e("shortcuts-help", {
                            on: {
                                hidden: t.onToggleHelp
                            }
                        })
                    }
                }))
            }
            static onTogglePerfBar(e) {
                e.preventDefault();
                Object(u.H)(Object(u.k)("perf_bar_enabled")) ? Object(u.N)("perf_bar_enabled", "false", {
                    path: "/"
                }) : Object(u.N)("perf_bar_enabled", "true", {
                    path: "/"
                }),
                Object(l.I)()
            }
            static onToggleCanary(e) {
                e.preventDefault();
                const t = Object(u.H)(Object(u.k)("gitlab_canary"));
                Object(u.N)("gitlab_canary", (!t).toString(), {
                    expires: 365,
                    path: "/"
                }),
                Object(l.I)()
            }
            static toggleMarkdownPreview(e) {
                r()(document).triggerHandler("markdown-preview:toggle", [e])
            }
            focusFilter(e) {
                this.filterInput || (this.filterInput = r()("input[type=search]", ".nav-controls")),
                this.filterInput.focus(),
                e.preventDefault()
            }
            static focusSearch(e) {
                var t;
                null === (t = document.querySelector("#super-sidebar-search")) || void 0 === t || t.click(),
                e.preventDefault && e.preventDefault()
            }
            static hideAppearingContent(e) {
                document.querySelectorAll(".tooltip, .popover").forEach((function(e) {
                    e.style.display = "none"
                }
                )),
                e.preventDefault && e.preventDefault()
            }
            static initMarkdownEditorShortcuts(e, t) {
                const o = b(e)
                  , i = new d.b(e[0]);
                e.data(f, i),
                o.forEach((function(e, o) {
                    i.bind(e, (function(e) {
                        e.preventDefault(),
                        t(o)
                    }
                    ))
                }
                ));
                const s = n()([...o.values()])
                  , r = d.b.prototype.stopCallback;
                i.stopCallback = function(e, t, o) {
                    return !s.includes(o) && r.call(this, e, t, o)
                }
            }
            static removeMarkdownEditorShortcuts(e) {
                const t = e.data(f);
                t && b(e).forEach((function(e) {
                    t.unbind(e)
                }
                ))
            }
        }
    }
}]);
//# sourceMappingURL=shortcutsBundle.51cac26b.chunk.js.map

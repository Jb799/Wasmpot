(this.webpackJsonp=this.webpackJsonp||[]).push([[749],{"05sH":function(e,t,r){var n=r("8FVE"),i=r("zxUd"),a=r("r6dd"),s=r("aTSC"),o=r("7Zqv"),c=r("M1vi"),u=r("etTJ"),l=r("/NDV"),d=r("Ksks"),h=r("WlOc"),f=r("rhmX"),m=/\b__p \+= '';/g,p=/\b(__p \+=) '' \+/g,b=/(__e\(.*?\)|\b__t\)) \+\n'';/g,v=/[()=,{}\[\]\/\s]/,g=/\$\{([^\\}]*(?:\\.[^\\}]*)*)\}/g,y=/($^)/,w=/['\n\r\u2028\u2029\\]/g,j=Object.prototype.hasOwnProperty;e.exports=function(e,t,r){var O=h.imports._.templateSettings||h;r&&u(e,t,r)&&(t=void 0),e=f(e),t=n({},t,O,s);var E,_,k=n({},t.imports,O.imports,s),S=l(k),T=a(k,S),A=0,P=t.interpolate||y,V="__p += '",x=RegExp((t.escape||y).source+"|"+P.source+"|"+(P===d?g:y).source+"|"+(t.evaluate||y).source+"|$","g"),I=j.call(t,"sourceURL")?"//# sourceURL="+(t.sourceURL+"").replace(/\s/g," ")+"\n":"";e.replace(x,(function(t,r,n,i,a,s){return n||(n=i),V+=e.slice(A,s).replace(w,o),r&&(E=!0,V+="' +\n__e("+r+") +\n'"),a&&(_=!0,V+="';\n"+a+";\n__p += '"),n&&(V+="' +\n((__t = ("+n+")) == null ? '' : __t) +\n'"),A=s+t.length,t})),V+="';\n";var C=j.call(t,"variable")&&t.variable;if(C){if(v.test(C))throw new Error("Invalid `variable` option passed into `_.template`")}else V="with (obj) {\n"+V+"\n}\n";V=(_?V.replace(m,""):V).replace(p,"$1").replace(b,"$1;"),V="function("+(C||"obj")+") {\n"+(C?"":"obj || (obj = {});\n")+"var __t, __p = ''"+(E?", __e = _.escape":"")+(_?", __j = Array.prototype.join;\nfunction print() { __p += __j.call(arguments, '') }\n":";\n")+V+"return __p\n}";var F=i((function(){return Function(S,I+"return "+V).apply(void 0,T)}));if(F.source=V,c(F))throw F;return F}},313:function(e,t,r){r("HVBj"),r("t9jZ"),e.exports=r("XG5e")},"7Zqv":function(e,t){var r={"\\":"\\","'":"'","\n":"n","\r":"r","\u2028":"u2028","\u2029":"u2029"};e.exports=function(e){return"\\"+r[e]}},"8FVE":function(e,t,r){var n=r("xJuT"),i=r("eiA/"),a=r("UwPs"),s=i((function(e,t,r,i){n(t,a(t),e,i)}));e.exports=s},Btec:function(e,t,r){"use strict";r.d(t,"a",(function(){return a})),r.d(t,"b",(function(){return s})),r.d(t,"c",(function(){return o})),r.d(t,"d",(function(){return c})),r.d(t,"e",(function(){return u})),r.d(t,"f",(function(){return l})),r.d(t,"g",(function(){return d})),r.d(t,"h",(function(){return h})),r.d(t,"i",(function(){return f})),r.d(t,"j",(function(){return m})),r.d(t,"k",(function(){return p})),r.d(t,"l",(function(){return b})),r.d(t,"m",(function(){return v})),r.d(t,"n",(function(){return g})),r.d(t,"o",(function(){return y})),r.d(t,"p",(function(){return w})),r.d(t,"q",(function(){return j})),r.d(t,"r",(function(){return O})),r.d(t,"s",(function(){return E})),r.d(t,"t",(function(){return _})),r.d(t,"v",(function(){return k})),r.d(t,"u",(function(){return S}));var n=r("/lV4"),i=r("qLpH");const a=Object(n.a)("Register device"),s=Object(n.a)("Set up new device"),o=Object(n.a)("Try again?"),c=Object(n.a)("Device name"),u=Object(n.a)("Excluding USB security keys, you should include the browser name together with the device name."),l=Object(n.a)("Macbook Touch ID on Edge"),d=Object(n.a)("WebAuthn only works with HTTPS-enabled websites. Contact your administrator for more details."),h=Object(n.a)("Your browser doesn't support WebAuthn. Please use a supported browser, e.g. Chrome (67+) or Firefox (60+)."),f=Object(n.a)("Your device needs to be set up. Plug it in (if needed) and click the button on the left."),m=Object(n.a)("You must save your recovery codes after you first register a two-factor authenticator, so you do not lose access to your account. %{linkStart}See the documentation on managing your WebAuthn device for more information.%{linkEnd}"),p=Object(n.a)("Current password"),b=Object(n.a)("Your current password is required to register a new device."),v=Object(n.a)("Your device was successfully set up! Give it a name and register it with the GitLab server."),g=Object(n.a)("Trying to communicate with your device. Plug it in (if needed) and press the button on the device now."),y="error",w="ready",j="success",O="unsupported",E="waiting",_="authenticate",k="register",S=Object(i.a)("user/profile/account/two_factor_authentication",{anchor:"set-up-a-webauthn-device"})},F6ad:function(e,t,r){"use strict";var n=r("Q78f"),i=r("Pyw5"),a=r.n(i);const s={name:"GlForm",components:{BForm:n.a},inheritAttrs:!1};const o=a()({render:function(){var e=this.$createElement;return(this._self._c||e)("b-form",this._g(this._b({},"b-form",this.$attrs,!1),this.$listeners),[this._t("default")],2)},staticRenderFns:[]},void 0,s,void 0,!1,void 0,!1,void 0,void 0,void 0);t.a=o},GVjJ:function(e,t,r){"use strict";r.d(t,"g",(function(){return i})),r.d(t,"b",(function(){return s})),r.d(t,"a",(function(){return o})),r.d(t,"c",(function(){return c})),r.d(t,"d",(function(){return l})),r.d(t,"e",(function(){return h})),r.d(t,"f",(function(){return f}));r("RFHG"),r("z6RN"),r("xuo1"),r("3UXl"),r("iyoE"),r("UezY"),r("hG7+"),r("TPye"),r("FMw2");var n=r("0AwG");const i=function(e){const t=new FormData(e);return function(e){return e.reduce((function(e,{name:t,value:r}){return Object.assign(e,{[t]:r})}),{})}(Array.from(t.keys()).map((function(e){let r=t.getAll(e);return r=r.filter((function(e){return e})),{name:e,value:1===r.length?r[0]:r}})))},a=function(e){return"string"==typeof e?e.trim():e},s=function(e){return null==e||0===a(e).length},o=function(e,t){return!s(e)&&e.length>=t},c=function(e,t){return function(e){return!s(e)&&Number.isInteger(Number(a(e)))}(e)&&parseInt(e,10)>t},u=/^[\w\-._]+@[\w\-.]+\.[a-zA-Z]{2,}$/,l=function(e){return u.test(e)},d=/^[^@\s]+@[^@\s]+$/,h=function(e){return d.test(e)},f=function(e){if(!e)throw new TypeError("`mountEl` argument is required");return[...e.querySelectorAll("[name]")].reduce((function(e,t){const r=t.dataset.jsName;if(!r)return e;const i=Object(n.g)(r),{id:a,placeholder:s,name:o,value:c,type:u,checked:l,maxLength:d,pattern:h}=t,f={name:o,id:a,value:c,...s&&{placeholder:s},...t.hasAttribute("maxlength")&&{maxLength:d},...h&&{pattern:h}};return["radio","checkbox"].includes(u)?{...e,[i]:[...e[i]||[],{...f,checked:l}]}:{...e,[i]:f}}),{})}},GWY9:function(e,t,r){"use strict";r.d(t,"a",(function(){return m})),r.d(t,"b",(function(){return p}));var n=r("EmJ/"),i=r.n(n),a=r("Btec"),s=r("jToU"),o=r("05sH"),c=r.n(o);class u{constructor(e,t){this.container=e,this.templates=t}renderTemplate(e,t){const r=document.querySelector(this.templates[e]).innerHTML,n=c()(r);this.container.html(n(t))}renderError(e){this.renderTemplate("error",{error_message:e.message(),error_name:e.errorName})}}var l=r("fQCf");class d{constructor(e,t,r,n,i){this.container=e,this.webauthnParams=Object(l.f)(JSON.parse(r.options)),this.renderInProgress=this.renderInProgress.bind(this),this.form=t,this.fallbackButton=n,this.fallbackUI=i,this.fallbackButton&&this.fallbackButton.addEventListener("click",this.switchToFallbackUI.bind(this)),this.flow=new u(e,{inProgress:"#js-authenticate-token-2fa-in-progress",error:"#js-authenticate-token-2fa-error",authenticated:"#js-authenticate-token-2fa-authenticated"}),this.container.on("click","#js-token-2fa-try-again",this.renderInProgress)}start(){Object(l.i)()?this.renderInProgress():this.switchToFallbackUI()}authenticate(){var e=this;navigator.credentials.get({publicKey:this.webauthnParams}).then((function(t){const r=Object(l.g)(t);e.renderAuthenticated(JSON.stringify(r))})).catch((function(t){e.flow.renderError(new s.a(t,a.t))}))}renderInProgress(){this.flow.renderTemplate("inProgress"),this.authenticate()}renderAuthenticated(e){this.flow.renderTemplate("authenticated");const t=this.container[0];t.querySelector("#js-device-response").value=e,t.querySelector(this.form).submit(),this.fallbackButton.classList.add("hidden")}switchToFallbackUI(){this.fallbackButton.classList.add("hidden"),this.container[0].classList.add("hidden"),this.fallbackUI.classList.remove("hidden")}}r("B++/"),r("z6RN"),r("47t/");var h=r("/lV4");class f{constructor(e,t){this.container=e,this.renderInProgress=this.renderInProgress.bind(this),this.webauthnOptions=Object(l.d)(t.options),this.flow=new u(e,{message:"#js-register-2fa-message",setup:"#js-register-token-2fa-setup",error:"#js-register-token-2fa-error",registered:"#js-register-token-2fa-registered"}),this.container.on("click","#js-token-2fa-try-again",this.renderInProgress)}start(){Object(l.i)()?this.renderSetup():this.renderNotSupported(!Object(l.h)())}register(){var e=this;navigator.credentials.create({publicKey:this.webauthnOptions}).then((function(t){return e.renderRegistered(JSON.stringify(Object(l.e)(t)))})).catch((function(t){return e.flow.renderError(new s.a(t,a.v))}))}renderSetup(){this.flow.renderTemplate("setup"),this.container.find("#js-setup-token-2fa-device").on("click",this.renderInProgress)}renderInProgress(){return this.flow.renderTemplate("message",{message:Object(h.a)("Trying to communicate with your device. Plug it in (if needed) and press the button on the device now.")}),this.register()}renderRegistered(e){this.flow.renderTemplate("registered"),this.container.find("#js-device-response").val(e)}renderNotSupported(e){const t=e?Object(h.a)("WebAuthn only works with HTTPS-enabled websites. Contact your administrator for more details."):Object(h.a)("Your browser doesn't support WebAuthn. Please use a supported browser, e.g. Chrome (67+) or Firefox (60+).");this.flow.renderTemplate("message",{message:t})}}const m=function(){!function(){if(!gon.webauthn)return;new d(i()("#js-authenticate-token-2fa"),"#js-login-token-2fa-form",gon.webauthn,document.querySelector("#js-login-2fa-device"),document.querySelector(".js-2fa-form")).start()}()},p=function(){!function(){const e=i()("#js-register-token-2fa");if(!e.length)return;new f(e,gon.webauthn).start()}()}},Ksks:function(e,t){e.exports=/<%=([\s\S]+?)%>/g},M1vi:function(e,t,r){var n=r("XpzN"),i=r("QA6A"),a=r("8Ei6");e.exports=function(e){if(!i(e))return!1;var t=n(e);return"[object Error]"==t||"[object DOMException]"==t||"string"==typeof e.message&&"string"==typeof e.name&&!a(e)}},Q78f:function(e,t,r){"use strict";r.d(t,"b",(function(){return c})),r.d(t,"a",(function(){return u}));var n=r("8ENL"),i=r("lgrP"),a=r("0zRR"),s=r("fkuG"),o=r("ua/H"),c=Object(o.c)({id:Object(o.b)(s.r),inline:Object(o.b)(s.g,!1),novalidate:Object(o.b)(s.g,!1),validated:Object(o.b)(s.g,!1)},a.r),u=Object(n.c)({name:a.r,functional:!0,props:c,render:function(e,t){var r=t.props,n=t.data,a=t.children;return e("form",Object(i.a)(n,{class:{"form-inline":r.inline,"was-validated":r.validated},attrs:{id:r.id,novalidate:r.novalidate}}),a)}})},WlOc:function(e,t,r){var n=r("G3fq"),i={escape:r("ecp+"),evaluate:r("cKyt"),interpolate:r("Ksks"),variable:"",imports:{_:{escape:n}}};e.exports=i},XG5e:function(e,t,r){"use strict";r.r(t);var n=r("EmJ/"),i=r.n(n),a=r("YbE4"),s=r("pqEc"),o=r("lOo6"),c=r("SZIZ"),u=r("ewH8"),l=r("NmEs"),d=r("30su"),h=r("F6ad"),f=r("l5WF"),m=r("PrLL"),p=r("4lAS"),b=r("3twG"),v=r("jlnU"),g=r("2ibD"),y=r("/lV4");const w=Object(y.g)("IdentityVerification|For added security, you'll need to verify your identity. We've sent a verification code to %{email}"),j=Object(y.g)("IdentityVerification|Verification code"),O=Object(y.g)("IdentityVerification|Enter a code."),E=Object(y.g)("IdentityVerification|Please enter a valid code"),_=Object(y.g)("IdentityVerification|Verify code"),k=Object(y.g)("IdentityVerification|Resend code"),S=Object(y.g)("IdentityVerification|A new code has been sent."),T=Object(y.g)("IdentityVerification|Something went wrong. Please try again."),A=Object(y.a)("Email"),P=Object(y.g)("IdentityVerification|Update email"),V=Object(y.g)("EmailVerification|Update your email to a valid permanent address. If you use a temporary email, you won't be able to sign in later."),x=Object(y.a)("Cancel"),I=Object(y.g)("IdentityVerification|Please enter a valid email address."),C=Object(y.g)("IdentityVerification|A new code has been sent to your updated email address."),F=/^\d{6}$/;var N=r("GVjJ"),R={name:"UpdateEmail",components:{GlForm:h.a,GlFormGroup:f.a,GlFormInput:m.a,GlButton:p.a},props:{updateEmailPath:{type:String,required:!0}},data:()=>({email:"",submitted:!1,verifyError:""}),computed:{inputValidation(){return{state:!(this.submitted&&this.invalidFeedback),message:this.invalidFeedback}},invalidFeedback(){return this.submitted?Object(N.e)(this.email)?this.verifyError:I:""}},watch:{email(){this.verifyError=""}},methods:{updateEmail(){this.submitted=!0,this.inputValidation.state&&g.a.patch(this.updateEmailPath,{user:{email:this.email}}).then(this.handleResponse).catch(this.handleError)},handleResponse(e){void 0===e.data.status?this.handleError():"success"===e.data.status?this.handleSuccess():"failure"===e.data.status&&(this.verifyError=e.data.message)},handleSuccess(){Object(v.createAlert)({message:C,variant:v.VARIANT_SUCCESS}),this.$emit("verifyToken",this.email)},handleError(e){Object(v.createAlert)({message:T,captureError:!0,error:e})}},i18n:{email:A,updateEmail:P,cancel:x,guidance:V}},U=r("tBpV"),$=Object(U.a)(R,(function(){var e=this,t=e._self._c;return t("gl-form",{attrs:{novalidate:""},on:{submit:function(t){return t.preventDefault(),e.updateEmail.apply(null,arguments)}}},[t("gl-form-group",{attrs:{label:e.$options.i18n.email,"label-for":"update-email",state:e.inputValidation.state,"invalid-feedback":e.inputValidation.message}},[t("gl-form-input",{attrs:{id:"update-email",type:"email",autofocus:"",state:e.inputValidation.state},model:{value:e.email,callback:function(t){e.email=t},expression:"email"}}),e._v(" "),t("p",{staticClass:"gl-mt-3 gl-text-secondary"},[e._v(e._s(e.$options.i18n.guidance))])],1),e._v(" "),t("section",{staticClass:"gl-mt-5"},[t("gl-button",{attrs:{block:"",variant:"confirm",type:"submit",disabled:!e.inputValidation.state}},[e._v(e._s(e.$options.i18n.updateEmail))]),e._v(" "),t("gl-button",{staticClass:"gl-mt-3 gl-h-7",attrs:{block:"",variant:"link"},on:{click:function(t){return e.$emit("verifyToken")}}},[e._v(e._s(e.$options.i18n.cancel))])],1)],1)}),[],!1,null,null,null).exports,q={name:"EmailVerification",components:{GlSprintf:d.a,GlForm:h.a,GlFormGroup:f.a,GlFormInput:m.a,GlButton:p.a,UpdateEmail:$},props:{obfuscatedEmail:{type:String,required:!0},verifyPath:{type:String,required:!0},resendPath:{type:String,required:!0},isOfferEmailReset:{type:Boolean,required:!0},updateEmailPath:{type:String,required:!0}},data(){return{email:this.obfuscatedEmail,verificationCode:"",submitted:!1,verifyError:"",showUpdateEmail:!1}},computed:{inputValidation(){return{state:!(this.submitted&&this.invalidFeedback),message:this.invalidFeedback}},invalidFeedback(){return this.submitted?this.verificationCode?F.test(this.verificationCode)?this.verifyError:E:O:""}},watch:{verificationCode(){this.verifyError=""}},methods:{verify(){this.submitted=!0,this.inputValidation.state&&g.a.post(this.verifyPath,{user:{verification_token:this.verificationCode}}).then(this.handleVerificationResponse).catch(this.handleError)},handleVerificationResponse(e){void 0===e.data.status?this.handleError():"success"===e.data.status?Object(b.U)(e.data.redirect_path):"failure"===e.data.status&&(this.verifyError=e.data.message)},resend(){g.a.post(this.resendPath).then(this.handleResendResponse).catch(this.handleError).finally(this.resetForm)},handleResendResponse(e){void 0===e.data.status?this.handleError():"success"===e.data.status?Object(v.createAlert)({message:S,variant:v.VARIANT_SUCCESS}):"failure"===e.data.status&&Object(v.createAlert)({message:e.data.message})},handleError(e){Object(v.createAlert)({message:T,captureError:!0,error:e})},resetForm(){this.verificationCode="",this.submitted=!1,this.$refs.input.$el.focus()},updateEmail(){this.showUpdateEmail=!0},verifyToken(e=""){this.showUpdateEmail=!1,e.length&&(this.email=e),this.$nextTick(this.resetForm)}},i18n:{explanation:w,inputLabel:j,submitButton:_,resendLink:k,updateEmail:P}},G=Object(U.a)(q,(function(){var e=this,t=e._self._c;return t("div",[e.showUpdateEmail?t("update-email",{attrs:{"update-email-path":e.updateEmailPath},on:{verifyToken:e.verifyToken}}):t("gl-form",{on:{submit:function(t){return t.preventDefault(),e.verify.apply(null,arguments)}}},[t("section",{staticClass:"gl-mb-5"},[t("gl-sprintf",{attrs:{message:e.$options.i18n.explanation},scopedSlots:e._u([{key:"email",fn:function(){return[t("strong",[e._v(e._s(e.email))])]},proxy:!0}])})],1),e._v(" "),t("gl-form-group",{attrs:{label:e.$options.i18n.inputLabel,"label-for":"verification-code",state:e.inputValidation.state,"invalid-feedback":e.inputValidation.message}},[t("gl-form-input",{ref:"input",attrs:{id:"verification-code",autofocus:"",autocomplete:"one-time-code",inputmode:"numeric",maxlength:"6",state:e.inputValidation.state},model:{value:e.verificationCode,callback:function(t){e.verificationCode=t},expression:"verificationCode"}})],1),e._v(" "),t("section",{staticClass:"gl-mt-5"},[t("gl-button",{attrs:{block:"",variant:"confirm",type:"submit",disabled:!e.inputValidation.state}},[e._v(e._s(e.$options.i18n.submitButton))]),e._v(" "),t("gl-button",{staticClass:"gl-mt-3 gl-h-7",attrs:{block:"",variant:"link"},on:{click:e.resend}},[e._v(e._s(e.$options.i18n.resendLink))]),e._v(" "),e.isOfferEmailReset?t("gl-button",{staticClass:"gl-mt-3 gl-h-7",attrs:{block:"",variant:"link"},on:{click:e.updateEmail}},[e._v(e._s(e.$options.i18n.updateEmail))]):e._e()],1)],1)],1)}),[],!1,null,null,null).exports,L=r("EJYk");var D=r("n7CP");new(r("BQcc").a),new c.a,new class{constructor({currentTabKey:e="current_signin_tab",tabSelector:t="ul.new-session-tabs"}={}){this.currentTabKey=e,this.tabSelector=t,this.isLocalStorageAvailable=D.a.canUseLocalStorage(),window.location.hash&&this.saveData(window.location.hash),this.bootstrap()}bootstrap(){var e=this;const t=document.querySelectorAll(this.tabSelector);t.length>0&&t[0].addEventListener("click",(function(t){if(t.target&&"A"===t.target.nodeName){const r=t.target.getAttribute("href");e.saveData(r)}})),this.showTab()}showTab(){const e=this.readData();if(e){const t=document.querySelector(`${this.tabSelector} a[href="${e}"]`);if(t)t.click();else{const e=document.querySelector(this.tabSelector+" a");e&&e.click()}}}saveData(e){if(this.isLocalStorageAvailable)return window.localStorage.setItem(this.currentTabKey,e)}readData(){return this.isLocalStorageAvailable?window.localStorage.getItem(this.currentTabKey):null}},new s.a,new class{constructor(e={}){this.container=e.container||""}bindEvents(){i()("#remember_me_omniauth",this.container).on("click",this.toggleRememberMe)}toggleRememberMe(e){const t=i()(e.target).is(":checked");i()(".js-oauth-login",this.container).each((function(e,r){const n=i()(r).parent("form"),a=n.attr("action");t?n.attr("action",Object(b.D)({remember_me:1},a)):n.attr("action",Object(b.L)(["remember_me"],a))}))}}({container:i()(".omniauth-container")}).bindEvents(),function(e=""){if(e){const t=e.replace(/^#/,""),r=document.querySelectorAll("#signin-container .tab-content form");Array.prototype.forEach.call(r,(function(e){const r=Object(b.O)(e.getAttribute("action"),"#"+t);e.setAttribute("action",r)}));const n=document.querySelectorAll("#signin-container .omniauth-container form");Array.prototype.forEach.call(n,(function(e){const r=Object(b.D)({redirect_fragment:t},e.getAttribute("action"));e.setAttribute("action",r)}))}}(window.location.hash),Object(a.a)(),Object(o.a)(),function(){const e=document.querySelector(".js-email-verification");if(!e)return null;const{obfuscatedEmail:t,verifyPath:r,resendPath:n,offerEmailReset:i,updateEmailPath:a}=e.dataset;new u.default({el:e,name:"EmailVerificationRoot",render:e=>e(G,{props:{obfuscatedEmail:t,verifyPath:r,resendPath:n,isOfferEmailReset:Object(l.H)(i),updateEmailPath:a}})})}(),Object(L.a)(document.body)},YbE4:function(e,t,r){"use strict";r("3UXl"),r("iyoE");var n=r("ewH8"),i=r("NmEs"),a=r("dsWN"),s=r("Mp8J"),o={name:"DismissibleAlert",components:{GlAlert:a.a},directives:{SafeHtml:s.a},props:{html:{type:String,required:!1,default:""}},data:()=>({isDismissed:!1}),methods:{dismiss(){this.isDismissed=!0,this.$emit("alertDismissed")}}},c=r("tBpV"),u=Object(c.a)(o,(function(){var e=this._self._c;return this.isDismissed?this._e():e("gl-alert",this._g(this._b({on:{dismiss:this.dismiss}},"gl-alert",this.$attrs,!1),this.$listeners),[e("div",{directives:[{name:"safe-html",rawName:"v-safe-html",value:this.html,expression:"html"}]})])}),[],!1,null,null,null).exports;const l=function(e){const t=Number(e);return!e||Number.isNaN(t)?30:t},d=function(e){const t={html:e.innerHTML},r={...e.dataset,dismissible:Object(i.H)(e.dataset.dismissible)},{dismissCookieName:a,dismissCookieExpire:s}=e.dataset;return new n.default({el:e,render:e=>e(u,{props:t,attrs:r,on:{alertDismissed(){a&&Object(i.N)(a,!0,{expires:l(s)})}}})})};t.a=function(){return[...document.querySelectorAll(".js-vue-alert")].map(d)}},aTSC:function(e,t,r){var n=r("hmyg"),i=Object.prototype,a=i.hasOwnProperty;e.exports=function(e,t,r,s){return void 0===e||n(e,i[r])&&!a.call(s,r)?t:e}},cKyt:function(e,t){e.exports=/<%([\s\S]+?)%>/g},"ecp+":function(e,t){e.exports=/<%-([\s\S]+?)%>/g},"eiA/":function(e,t,r){var n=r("Opi0"),i=r("etTJ");e.exports=function(e){return n((function(t,r){var n=-1,a=r.length,s=a>1?r[a-1]:void 0,o=a>2?r[2]:void 0;for(s=e.length>3&&"function"==typeof s?(a--,s):void 0,o&&i(r[0],r[1],o)&&(s=a<3?void 0:s,a=1),t=Object(t);++n<a;){var c=r[n];c&&e(t,c,n,s)}return t}))}},jToU:function(e,t,r){"use strict";r.d(t,"a",(function(){return s}));var n=r("/lV4"),i=r("Btec"),a=r("fQCf");class s{constructor(e,t){this.error=e,this.errorName=e.name||"UnknownError",this.message=this.message.bind(this),this.httpsDisabled=!Object(a.h)(),this.flowType=t}message(){return"NotSupportedError"===this.errorName?Object(n.a)("Your device is not compatible with GitLab. Please try another device"):"InvalidStateError"===this.errorName&&this.flowType===i.t?Object(n.a)("This device has not been registered with us."):"InvalidStateError"===this.errorName&&this.flowType===i.v?Object(n.a)("This device has already been registered with us."):"SecurityError"===this.errorName&&this.httpsDisabled?Object(n.a)("WebAuthn only works with HTTPS-enabled websites. Contact your administrator for more details."):Object(n.a)("There was a problem communicating with your device.")}}},t9jZ:function(e,t,r){"use strict";r.r(t);var n=r("GWY9"),i=r("wDRK");Object(n.a)(),Object(i.a)()},xJuT:function(e,t,r){var n=r("1/+g"),i=r("vkS7");e.exports=function(e,t,r,a){var s=!r;r||(r={});for(var o=-1,c=t.length;++o<c;){var u=t[o],l=a?a(r[u],e[u],u,r,e):void 0;void 0===l&&(l=e[u]),s?i(r,u,l):n(r,u,l)}return r}},zxUd:function(e,t,r){var n=r("PIq0"),i=r("Opi0"),a=r("M1vi"),s=i((function(e,t){try{return n(e,void 0,t)}catch(e){return a(e)?e:new Error(e)}}));e.exports=s}},[[313,1,0,2,4,90,195]]]);
//# sourceMappingURL=pages.sessions.new.e7e8036e.chunk.js.map
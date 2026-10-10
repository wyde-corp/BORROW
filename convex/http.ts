import { httpRouter } from "convex/server";
import { exchange, logout } from "./auth";
import { checkout, checkoutCancel, checkoutSuccess, portal } from "./billing";
import { bachsWebhook } from "./bachsWebhook";
import { me } from "./me";
import { getProgress, postProgress } from "./progressSync";

const http = httpRouter();

// Auth — the desktop's loopback completes the OAuth dance, then hands the
// code here.
http.route({ path: "/auth/exchange", method: "POST", handler: exchange });
http.route({ path: "/auth/logout", method: "POST", handler: logout });

// Session-scoped reads and billing entry points. `/entitlement` is the
// polling alias of `/me` the checkout flow hammers while the browser tab is
// open — same payload, a name that says why it is being asked.
http.route({ path: "/me", method: "GET", handler: me });
http.route({ path: "/entitlement", method: "GET", handler: me });
http.route({ path: "/checkout", method: "POST", handler: checkout });
// Where hosted checkout sends the browser — static pages, because the
// desktop polls entitlement rather than trusting a redirect.
http.route({ path: "/checkout/success", method: "GET", handler: checkoutSuccess });
http.route({ path: "/checkout/cancel", method: "GET", handler: checkoutCancel });
http.route({ path: "/portal", method: "POST", handler: portal });

// Progress sync: pull on launch, push after each completion.
http.route({ path: "/progress", method: "GET", handler: getProgress });
http.route({ path: "/progress", method: "POST", handler: postProgress });

// Bachs → us. Public by design (signature does the authenticating).
http.route({ path: "/webhooks/bachs", method: "POST", handler: bachsWebhook });

export default http;

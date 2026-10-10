/* eslint-disable */
/**
 * Generated `api` utility.
 *
 * THIS CODE IS AUTOMATICALLY GENERATED.
 *
 * To regenerate, run `npx convex dev`.
 * @module
 */

import type * as auth from "../auth.js";
import type * as bachsWebhook from "../bachsWebhook.js";
import type * as billing from "../billing.js";
import type * as http from "../http.js";
import type * as lib_entitlement from "../lib/entitlement.js";
import type * as lib_http from "../lib/http.js";
import type * as lib_parseEvents from "../lib/parseEvents.js";
import type * as lib_sessionTokens from "../lib/sessionTokens.js";
import type * as lib_signature from "../lib/signature.js";
import type * as me from "../me.js";
import type * as progress from "../progress.js";
import type * as progressSync from "../progressSync.js";
import type * as sessions from "../sessions.js";
import type * as subscriptions from "../subscriptions.js";
import type * as users from "../users.js";

import type {
  ApiFromModules,
  FilterApi,
  FunctionReference,
} from "convex/server";

declare const fullApi: ApiFromModules<{
  auth: typeof auth;
  bachsWebhook: typeof bachsWebhook;
  billing: typeof billing;
  http: typeof http;
  "lib/entitlement": typeof lib_entitlement;
  "lib/http": typeof lib_http;
  "lib/parseEvents": typeof lib_parseEvents;
  "lib/sessionTokens": typeof lib_sessionTokens;
  "lib/signature": typeof lib_signature;
  me: typeof me;
  progress: typeof progress;
  progressSync: typeof progressSync;
  sessions: typeof sessions;
  subscriptions: typeof subscriptions;
  users: typeof users;
}>;

/**
 * A utility for referencing Convex functions in your app's public API.
 *
 * Usage:
 * ```js
 * const myFunctionReference = api.myModule.myFunction;
 * ```
 */
export declare const api: FilterApi<
  typeof fullApi,
  FunctionReference<any, "public">
>;

/**
 * A utility for referencing Convex functions in your app's internal API.
 *
 * Usage:
 * ```js
 * const myFunctionReference = internal.myModule.myFunction;
 * ```
 */
export declare const internal: FilterApi<
  typeof fullApi,
  FunctionReference<any, "internal">
>;

export declare const components: {};

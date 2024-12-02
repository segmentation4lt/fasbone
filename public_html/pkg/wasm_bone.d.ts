/* tslint:disable */
/* eslint-disable */
export function pageload(): void;
export function gethost(): void;
/**
 * @param {string} templat_name
 * @param {string} tag_id
 * @param {string} json_line
 */
export function bonerender(templat_name: string, tag_id: string, json_line: string): void;
/**
 * @param {string} form_name
 * @param {boolean} validation_only
 * @returns {any}
 */
export function reqdataform(form_name: string, validation_only: boolean): any;
/**
 * @param {string} request_method
 * @param {string} request_url
 * @param {string} serialize
 * @returns {any}
 */
export function fasconextendmanual(request_method: string, request_url: string, serialize: string): any;
/**
 * @param {string} form_name
 * @param {boolean} validation_only
 * @returns {any}
 */
export function fasconextendform(form_name: string, validation_only: boolean): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly pageload: () => void;
  readonly gethost: () => void;
  readonly bonerender: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly reqdataform: (a: number, b: number, c: number) => number;
  readonly fasconextendmanual: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly fasconextendform: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;

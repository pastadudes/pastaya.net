/* tslint:disable */
/* eslint-disable */
export function init_counter(): number;
export function increment_counter(): number;
/**
 * This function basically shows the 67 kid image when called
 */
export function bootstrap_67_kid_image(): void;
/**
 * This function is only meant for testing purposes, however I have no objections to anyone that
 * tries this
 * DOES NOT WRITE TO STORAGE!!!
 */
export function set_counter(v: number): void;
/**
 * Resets the counter. Pretty self explantory
 */
export function reset_counter(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init_counter: () => number;
  readonly increment_counter: () => number;
  readonly bootstrap_67_kid_image: () => [number, number];
  readonly set_counter: (a: number) => void;
  readonly reset_counter: () => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
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

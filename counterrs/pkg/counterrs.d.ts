/* tslint:disable */
/* eslint-disable */
/**
 * Sets IS_PAUSED to true.
 */
export function pause_counter(): void;
/**
 * Sets IS_PAUSED to false.
 */
export function resume_counter(): void;
export function init_counter(): number;
/**
 * Increments the counter.  
 * It also checks if IS_PAUSED is true or not.  
 * If IS_PAUSED is true then it doesn't increment, pretty simple
 */
export function increment_counter(): number;
/**
 * This function bootstraps an image and puts it in the DOM  
 * Examples:  
 * ```rust
 * let _ = bootstrap_image(
 *     "https://media1.tenor.com/m/WUWygJ0Fwz8AAAAd/jago33-slot-machine.gif", // This is what image to display
 *         Some("slot machine go BRRRRRRRRRRRR".to_string()), // Alt
 *         6000, // Duration in milliseconds
 *         None, // We don't use a class
 *    );
 * ```
 */
export function bootstrap_image(src: string, alt: string | null | undefined, duration: number, _class?: string | null): void;
/**
 * This function is only meant for testing purposes, however I have no objections to anyone that  
 * tries this  
 * DOES WRITE TO STORAGE!!!
 */
export function set_counter(v: number): void;
/**
 * Resets the counter. Pretty self explantory  
 * Probably considered the same as "set_counter(0)"
 */
export function reset_counter(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly pause_counter: () => void;
  readonly resume_counter: () => void;
  readonly init_counter: () => number;
  readonly increment_counter: () => number;
  readonly bootstrap_image: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number];
  readonly set_counter: (a: number) => void;
  readonly reset_counter: () => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_5: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly closure13_externref_shim: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__convert__closures_____invoke__heb5f8baa63f50e1d: (a: number, b: number) => void;
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
